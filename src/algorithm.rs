use crate::point::*;
use crate::simplex::*;
use crate::function::*;
use priority_queue::PriorityQueue;
use ordered_float::OrderedFloat;

/// takes a fucntion to maximise, an array of input intervals and a number of iterations
///
/// exploration_depth represents the number of splits we can exploit before requiring higher-level exploration
/// 0 represents full exploration (as sort of grid search) while high numbers focus on exploitation (no need to go very high)
/// 5 appears to be a good default value
/// as long as one stays in a reasonable range (5-10), the algorithm appears to not be very sensible to the parameter
pub fn simple_optimizer(f: fn(&Coordinates) -> f64,
                        input_interval: Vec<(f64, f64)>,
                        exploration_depth: usize,
                        nb_iter: usize)
                        -> (f64, Coordinates)
{
   let exploration_depth = 1. + (exploration_depth as f64);

   // builds initial conditions
   let f = TargetFunction::new(f, input_interval);
   let initial_simplex = Simplex::initial_simplex(&f);
   let mut best_point = initial_simplex.corners
                                       .iter()
                                       .max_by_key(|c| OrderedFloat(c.value))
                                       .expect("You need at least one dimension!")
                                       .clone();
   let mut iter = initial_simplex.corners.len();

   // used to compute the difference
   let mut best_value = best_point.value;
   let mut worst_value = initial_simplex.corners
                                        .iter()
                                        .map(|c| OrderedFloat(c.value))
                                        .min()
                                        .map(|v| *v)
                                        .expect("You need at least one dimension!");

   // initialize priority queue
   let mut queue: PriorityQueue<Simplex, OrderedFloat<f64>> = PriorityQueue::new();
   queue.push(initial_simplex, OrderedFloat(0.)); // no value computed for the initial simplex as it is alone anyway

   while (iter <= nb_iter) && !queue.is_empty()
   {
      // gets an up to date point
      let (mut simplex, mut evaluation) = queue.pop().expect("The queue cannot be empty!");
      /*let mut evaluation = simplex.evaluate(best_value - worst_value, exploration_preference);
      while evaluation != *_evaluation
      {
         queue.push(simplex, OrderedFloat(evaluation));
         let (simplex2, _evaluation2) = queue.pop().expect("The queue cannot be empty!");
         simplex = simplex2;
         _evaluation = _evaluation2;
         evaluation = simplex.evaluate(best_value - worst_value, exploration_preference);
      }*/

      // evaluate the center of the simplex
      let coordinates = simplex.center.clone();
      let value = f.evaluate(&coordinates);
      let new_point = Point { coordinates, value };

      // splits the simplex aroud its center and push the subsimplex into the queue
      simplex.split(&new_point)
             .into_iter()
             .map(|s| (OrderedFloat(s.evaluate(best_value - worst_value, exploration_depth)), s))
             .for_each(|(e, s)| {
                queue.push(s, e);
             });

      // updates the best value so far
      //println!("iter:{} best_value_so_far:{} current_value:{}", iter, best_point.value, new_point.value);
      let c = f.to_hypercube(new_point.coordinates.clone());
      println!("iter:{} value:{} in [{}, {}] <- [{}, {}]",
               iter, new_point.value, c[0], c[1], new_point.coordinates[0], new_point.coordinates[1]);
      if value > best_point.value
      {
         best_point = new_point;
         //let c = f.to_hypercube(best_point.coordinates.clone());
         //println!("iter:{} best_value_so_far:{} in [{}, {}]", iter, best_point.value, c[0], c[1]);
      }

      if value > best_value
      {
         best_value = value;
      }
      if value < worst_value
      {
         worst_value = value;
      }

      iter += 1;
   }

   (best_point.value, f.to_hypercube(best_point.coordinates))
}
