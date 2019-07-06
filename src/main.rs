use priority_queue::PriorityQueue;
use ordered_float::OrderedFloat;
use std::collections::HashMap;
use std::cmp::{min, max};

//-----------------------------------------------------------------------------
// POINT

/// represents coordinates in space
type Coordinates = Vec<f64>;

/// represents an already evaluated point in space
#[derive(Clone)]
struct Point
{
   coordinates: Coordinates,
   value: f64
}

impl Point
{
   /// computes the euclidian distance between two points
   fn distance(p1: &Coordinates, p2: &Coordinates) -> f64
   {
      p1.iter().zip(p2.iter()).map(|(x, y)| (x - y) * (x - y)).sum::<f64>().sqrt()
   }

   /// adds the point into the coordinates and returns the coordinates
   fn add_to(&self, coordinates: Coordinates) -> Coordinates
   {
      coordinates.iter().zip(self.coordinates.iter()).map(|(x, y)| x + y).collect()
   }

   /// computes the average of the coordinates
   fn average_coordinate(points: &[Point]) -> Coordinates
   {
      let length = points.len() as f64;
      let mut points = points.iter();
      let first = points.next().expect("Tried to average zero coordinates!").coordinates.clone();
      let sum = points.fold(first, |acc, x| x.add_to(acc));
      sum.iter().map(|sum| sum / length).collect()
   }
}

//-----------------------------------------------------------------------------
// SIMPLEX

/// represents a simplex
struct Simplex
{
   corners: Vec<Point>,
   center: Coordinates,
   ratio: f64
}

impl Simplex
{
   /// creates a new simplex
   fn new(corners: Vec<Point>, ratio: f64) -> Simplex
   {
      let center = Point::average_coordinate(&corners);
      //println!("center: x{} y{}", center[0], center[1]);
      Simplex { corners, center, ratio }
   }

   /// takes an hypercube and produces a simplex that contains the given hypercube
   /// NOTE: the ratio is set to 1. and the difference to 0.
   /// WARNING: this will create coordinates that are out of the given hypercube
   fn from_hypercube(hypercube: &[(f64, f64)], f: fn(&Coordinates) -> f64) -> Simplex
   {
      // builds all the corners of the simplex
      let min_coordinates: Vec<f64> = hypercube.iter().map(|(inf, _sup)| *inf).collect();
      let mut corners = vec![];
      for i in 0..hypercube.len()
      {
         let mut coordinates = min_coordinates.clone();
         let (inf_i, sup_i) = hypercube[i];
         coordinates[i] = sup_i + (sup_i - inf_i);

         let value = f(&coordinates);

         let corner = Point { coordinates, value };
         corners.push(corner);
      }

      // adds the corner that corresponds to the min coordinates
      let value_at_min = f(&min_coordinates);
      let corner_min = Point { coordinates: min_coordinates, value: value_at_min };
      corners.push(corner_min);

      Simplex::new(corners, 1.)
   }

   /// takes a simplex and splits it
   fn split(self, new_point: &Point) -> Vec<Simplex>
   {
      let mut result = vec![];
      let distances: Box<[f64]> = self.corners
                                      .iter()
                                      .map(|c| &c.coordinates)
                                      .map(|c| Point::distance(c, &new_point.coordinates))
                                      .collect();
      let total_distance: f64 = distances.iter().sum();
      for i in 0..self.corners.len()
      {
         // builds the corners of the new simplex
         let mut corners = self.corners.clone();
         corners[i] = new_point.clone();

         // computes the ratio of the child
         // which is the ratio of its father multiplied by the fraction of its father occupied by the child
         let ratio = self.ratio * (distances[i] / total_distance);

         let simplex = Simplex::new(corners, ratio);
         result.push(simplex);
      }
      result
   }

   /// returns a score for a simplex
   fn evaluate(&self, exploration_preference: f64, difference: f64) -> f64
   {
      // computes the value interpolated from the corners
      let mut total_inverse_distance = 0.;
      let interpolated_value = self.corners
                                   .iter()
                                   .map(|c| {
                                      let d = 1. / Point::distance(&c.coordinates, &self.center);
                                      total_inverse_distance += d;
                                      c.value * d
                                   })
                                   .sum::<f64>()
                               / total_inverse_distance;

      // computes the number of split needed to reach the given ratio if we start from a regular simplex
      let dim = self.center.len() as f64;
      let split_number = self.ratio.log(dim + 1.).abs();

      if difference == 0.
      {
         -split_number
      }
      else
      {
         //println!("{} - {} * {} * {}", interpolated_value, exploration_preference, difference, split_number);
         interpolated_value - exploration_preference * difference * split_number
      }
   }
}

//-----------------------------------------------------------------------------
// QUEUE

struct Queue
{
   queue: PriorityQueue<usize, OrderedFloat<f64>>,
   simplexes_index: HashMap<usize, Simplex>,
   exploration_preference: f64,
   previous_difference: f64,
   simplex_number: usize
}

impl Queue
{
   fn new(exploration_preference: f64) -> Queue
   {
      let queue = PriorityQueue::new();
      let simplexes_index = HashMap::new();
      let previous_difference = 0.;
      let simplex_number = 0;
      Queue { queue, simplexes_index, exploration_preference, previous_difference, simplex_number }
   }

   fn push(&mut self, key: Simplex, difference: f64)
   {
      // updates all the priorities if the difference has changed
      // TODO we could update on pop but it require storing the preference for each simplex in memory
      if difference != self.previous_difference
      {
         self.previous_difference = difference;
         for (id, simplex) in self.simplexes_index.iter()
         {
            let value = simplex.evaluate(self.exploration_preference, difference);
            self.queue.change_priority(&id, OrderedFloat(value));
         }
      }

      // inserts the new element
      let value = key.evaluate(self.exploration_preference, difference);
      let id = self.simplex_number;
      self.simplexes_index.insert(id, key);
      self.queue.push(id, OrderedFloat(value));
      self.simplex_number += 1;
   }

   fn pop(&mut self) -> Simplex
   {
      let (id, _value) = self.queue.pop().expect("The queue is empty!");
      self.simplexes_index.remove(&id).expect("Tried to remove a simplex that did not exist!")
   }
}

//-----------------------------------------------------------------------------
// ALGORITHM

/// takes a fucntion to maximise, an array of input intervals and a number of iterations
fn simple_optimizer(f: fn(&Coordinates) -> f64,
                    input_interval: &[(f64, f64)],
                    nb_iter: usize,
                    exploration_preference: f64)
                    -> f64
{
   let initial_simplex = Simplex::from_hypercube(input_interval, f);
   let mut worst_value = initial_simplex.corners
                                        .iter()
                                        .map(|c| OrderedFloat(c.value))
                                        .min()
                                        .expect("You need at least one dimenssion");
   let mut best_value = initial_simplex.corners
                                       .iter()
                                       .map(|c| OrderedFloat(c.value))
                                       .max()
                                       .expect("You need at least one dimenssion");
   let nb_consumed_iter = initial_simplex.corners.len();

   let mut queue = Queue::new(exploration_preference);
   queue.push(initial_simplex, *best_value - *worst_value);

   for iter in nb_consumed_iter..=nb_iter
   {
      let simplex = queue.pop();
      let coordinates = simplex.center.clone();

      let value = f(&coordinates);
      best_value = max(best_value, OrderedFloat(value));
      worst_value = min(worst_value, OrderedFloat(value));
      let difference = *best_value - *worst_value;
      println!("iter:{} best_value:{} current_value:{}", iter, best_value, value);

      let new_point = Point { coordinates, value };
      simplex.split(&new_point).into_iter().for_each(|s| queue.push(s, difference))
   }

   *best_value
}

//-----------------------------------------------------------------------------
// TEST

fn main()
{
   //let f: fn(&Coordinates) -> f64 = |v| -((v[0] - 0.2).powf(2.) + (v[1] - 0.3).powf(2.)).sqrt();
   let f: fn(&Coordinates) -> f64 = |v| {
      let x = v[0];
      let y = v[1];
      1. + 1. / ((x + 3.) * (x + 3.) + y * y).exp() + 0.5 / ((x + 1.).powf(2.) + (y + 2.).powf(2.)).exp()
   };
   let input_interval = vec![(-10., 10.), (-10., 10.)];
   let nb_iter = 300;
   let exploration_preference = 0.05;
   let best_value = simple_optimizer(f, &input_interval, nb_iter, exploration_preference);
   println!("best value : {}", best_value);
}
