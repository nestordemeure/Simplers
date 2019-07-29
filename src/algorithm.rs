use crate::point::*;
use crate::simplex::*;
use crate::search_space::*;
use priority_queue::PriorityQueue;
use ordered_float::OrderedFloat;
use std::rc::Rc;

/// Stores the parameters and current state of a search.
pub struct Optimizer
{
   exploration_depth: f64,
   search_space: SearchSpace,
   best_point: Rc<Point>,
   min_value: f64,
   queue: PriorityQueue<Simplex, OrderedFloat<f64>>
}

impl Optimizer
{
   /// Creates a new optimizer to explore the given search space with the iterator interface.
   ///
   /// Takes a function, a vector of intervals describing the input and a boolean describing wether it is a minimization problem (as oppozed to a miximization problem).
   /// Each cal to the `.next()` function (cf iterator trait) will run an iteration of search and output the best result so far.
   ///
   /// ```rust
   /// # fn main() {
   /// let f = |v| v[0] * v[1];
   /// let input_interval = vec![(-10., 10.), (-20., 20.)];
   /// let should_minimize = true;
   ///
   /// // runs the search for 30 iterations
   /// // then waits until we find a point good enough
   /// // finally stores the best value so far
   /// let (min_value, coordinates) = Optimizer::new(f, input_interval, should_minimize)
   ///                                          .skip(30)
   ///                                          .take_while(|(value,coordinates)| value > 1. )
   ///                                          .next().unwrap();
   ///
   /// println!("min value: {} found in [{}, {}]", min_value, coordinates[0], coordinates[1]);
   /// # }
   /// ```
   pub fn new(f: impl Fn(&[f64]) -> f64 + 'static,
              input_interval: Vec<(f64, f64)>,
              should_minimize: bool)
              -> Optimizer
   {
      // builds initial conditions
      let search_space = SearchSpace::new(f, input_interval, should_minimize);
      let initial_simplex = Simplex::initial_simplex(&search_space);

      // various values track through the iterations
      let best_point = initial_simplex.corners
                                      .iter()
                                      .max_by_key(|c| OrderedFloat(c.value))
                                      .expect("You need at least one dimension!")
                                      .clone();
      let min_value = initial_simplex.corners
                                     .iter()
                                     .map(|c| c.value)
                                     .min_by_key(|&v| OrderedFloat(v))
                                     .expect("You need at least one dimension!");

      // initialize priority queue
      // no need to evaluate the initial simplex as it will be poped immediatly
      let mut queue: PriorityQueue<Simplex, OrderedFloat<f64>> = PriorityQueue::new();
      queue.push(initial_simplex, OrderedFloat(0.));

      let exploration_depth = 6.;
      Optimizer { exploration_depth, search_space, best_point, min_value, queue }
   }

   /// Sets the exploration depth for the algorithm, useful when using the iterator interface.
   ///
   /// `exploration_depth` represents the number of splits we can exploit before requiring higher-level exploration.
   /// As long as one stays in a reasonable range (5-10), the algorithm should not be very sensible to the parameter :
   ///
   /// - 0 represents full exploration (similar to grid search)
   /// - high numbers focus on exploitation (no need to go very high)
   /// - 5 appears to be a good default value
   ///
   /// **WARNING**: this function should not be used before after an iteration
   /// (as it will not update the score of already computed points for the next iterations
   /// which will degrade the quality of the algorithm)
   ///
   /// ```rust
   /// # fn main() {
   /// let f = |v| v[0] * v[1];
   /// let input_interval = vec![(-10., 10.), (-20., 20.)];
   /// let should_minimize = true;
   ///
   /// // sets exploration_depth to be very greedy
   /// let (min_value_greedy, _) = Optimizer::new(f, input_interval, should_minimize)
   ///                                          .set_exploration_depth(20)
   ///                                          .skip(100)
   ///                                          .next().unwrap();
   ///
   /// // sets exploration_depth to focus on exploration
   /// let (min_value_explore, _) = Optimizer::new(f, input_interval, should_minimize)
   ///                                          .set_exploration_depth(0)
   ///                                          .skip(100)
   ///                                          .next().unwrap();
   ///
   /// println!("greedy result : {} vs exploration result : {}", min_value_greedy, min_value_explore);
   /// # }
   /// ```
   pub fn set_exploration_depth(mut self, exploration_depth: usize) -> Self
   {
      self.exploration_depth = 1. + (exploration_depth as f64);
      self
   }

   /// Self contained optimization algorithm.
   ///
   /// Takes a function to maximize, a vector of intervals describing the input and a number of iterations.
   ///
   /// ```rust
   /// # fn main() {
   /// let f = |v| v[0] + v[1];
   /// let input_interval = vec![(-10., 10.), (-20., 20.)];
   /// let nb_iterations = 100;
   ///
   /// let (max_value, coordinates) = Optimizer::maximize(f, input_interval, nb_iterations);
   /// println!("max value: {} found in [{}, {}]", max_value, coordinates[0], coordinates[1]);
   /// # }
   /// ```
   pub fn maximize(f: impl Fn(&[f64]) -> f64 + 'static,
                   input_interval: Vec<(f64, f64)>,
                   nb_iterations: usize)
                   -> (f64, Coordinates)
   {
      let initial_iteration_number = input_interval.len() + 1;
      let should_minimize = false;
      Optimizer::new(f, input_interval, should_minimize).skip(nb_iterations - initial_iteration_number)
                                                        .next()
                                                        .unwrap()
   }

   /// Self contained optimization algorithm.
   ///
   /// Takes a function to minimize, a vector of intervals describing the input and a number of iterations.
   ///
   /// ```rust
   /// # fn main() {
   /// let f = |v| v[0] * v[1];
   /// let input_interval = vec![(-10., 10.), (-20., 20.)];
   /// let nb_iterations = 100;
   ///
   /// let (min_value, coordinates) = Optimizer::minimize(f, input_interval, nb_iterations);
   /// println!("min value: {} found in [{}, {}]", min_value, coordinates[0], coordinates[1]);
   /// # }
   /// ```
   pub fn minimize(f: impl Fn(&[f64]) -> f64 + 'static,
                   input_interval: Vec<(f64, f64)>,
                   nb_iterations: usize)
                   -> (f64, Coordinates)
   {
      let initial_iteration_number = input_interval.len() + 1;
      let should_minimize = true;
      Optimizer::new(f, input_interval, should_minimize).skip(nb_iterations - initial_iteration_number)
                                                        .next()
                                                        .unwrap()
   }
}

/// implements iterator for the Optimizer to give full control on the stopping condition to the user
impl Iterator for Optimizer
{
   type Item = (f64, Coordinates);

   /// runs an iteration of the optimization algorithm and returns the best result so far
   fn next(&mut self) -> Option<Self::Item>
   {
      // gets the exploration depth for later use
      let exploration_depth = self.exploration_depth;

      // gets an up to date simplex
      let mut simplex = self.queue.pop().expect("Impossible: The queue cannot be empty!").0;
      let current_difference = self.best_point.value - self.min_value;
      while simplex.difference != current_difference
      {
         // updates the simplex and pushes it back into the queue
         simplex.difference = current_difference;
         let new_evaluation = simplex.evaluate(exploration_depth);
         self.queue.push(simplex, OrderedFloat(new_evaluation));
         // pops a new simplex
         simplex = self.queue.pop().expect("Impossible: The queue cannot be empty!").0;
      }

      // evaluate the center of the simplex
      let coordinates = simplex.center.clone();
      let value = self.search_space.evaluate(&coordinates);
      let new_point = Rc::new(Point { coordinates, value });

      // splits the simplex around its center and push the subsimplex into the queue
      simplex.split(new_point.clone(), current_difference)
             .into_iter()
             .map(|s| (OrderedFloat(s.evaluate(exploration_depth)), s))
             .for_each(|(e, s)| {
                self.queue.push(s, e);
             });

      // updates the difference
      if value > self.best_point.value
      {
         self.best_point = new_point;
      }
      else if value < self.min_value
      {
         self.min_value = value;
      }

      // gets the best value so far
      let best_value =
         if self.search_space.minimize { -self.best_point.value } else { self.best_point.value };
      let best_coordinate = self.search_space.to_hypercube(self.best_point.coordinates.clone());
      Some((best_value, best_coordinate))
   }
}
