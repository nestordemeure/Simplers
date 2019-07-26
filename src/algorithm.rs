use crate::point::*;
use crate::simplex::*;
use crate::function::*;
use priority_queue::PriorityQueue;
use ordered_float::OrderedFloat;

/// takes a function to **maximise**, a vector of input intervals, an exploration depth and a number of iterations
///
/// exploration_depth represents the number of splits we can exploit before requiring higher-level exploration
/// 0 represents full exploration (similar to grid search) while high numbers focus on exploitation (no need to go very high)
/// 5 appears to be a good default value
/// as long as one stays in a reasonable range (5-10), the algorithm should not be very sensible to the parameter
pub fn simple_optimizer(f: fn(&Coordinates) -> f64,
                        input_interval: Vec<(f64, f64)>,
                        exploration_depth: usize,
                        nb_iter: usize)
                        -> (f64, Coordinates)
{
   // builds initial conditions
   let exploration_depth = 1. + (exploration_depth as f64);
   let f = TargetFunction::new(f, input_interval);
   let initial_simplex = Simplex::initial_simplex(&f);

   // various values track through the iterations
   let mut max_point = initial_simplex.corners
                                      .iter()
                                      .max_by_key(|c| OrderedFloat(c.value))
                                      .expect("You need at least one dimension!")
                                      .clone();
   let mut min_value = initial_simplex.corners
                                      .iter()
                                      .map(|c| c.value)
                                      .min_by_key(|&v| OrderedFloat(v))
                                      .expect("You need at least one dimension!");
   let mut iter = initial_simplex.corners.len();

   // initialize priority queue
   let mut queue: PriorityQueue<Simplex, OrderedFloat<f64>> = PriorityQueue::new();
   queue.push(initial_simplex, OrderedFloat(0.)); // no need to evaluate the initial simplex as it will be poped immediatly

   // runs for as many iterations as schredules or until the queue runs out
   while (iter <= nb_iter) && !queue.is_empty()
   {
      // gets an up to date simplex
      let mut simplex = queue.pop().expect("Impossible: The queue cannot be empty!").0;
      let current_difference = max_point.value - min_value;
      while simplex.difference != current_difference
      {
         // updates the simplex and pushes it back into the queue
         simplex.difference = current_difference;
         let new_evaluation = simplex.evaluate(exploration_depth);
         queue.push(simplex, OrderedFloat(new_evaluation));
         // pops a new simplex
         simplex = queue.pop().expect("Impossible: The queue cannot be empty!").0;
      }

      // evaluate the center of the simplex
      let coordinates = simplex.center.clone();
      let value = f.evaluate(&coordinates);
      let new_point = Point { coordinates, value };

      // splits the simplex around its center and push the subsimplex into the queue
      simplex.split(&new_point, current_difference)
             .into_iter()
             .map(|s| (OrderedFloat(s.evaluate(exploration_depth)), s))
             .for_each(|(e, s)| {
                queue.push(s, e);
             });

      // updates the difference
      //let c = f.to_hypercube(new_point.coordinates.clone());
      //println!("iter:{} value:{} in [{}, {}] <- [{}, {}]", iter, new_point.value, c[0], c[1], new_point.coordinates[0], new_point.coordinates[1]);
      if value > max_point.value
      {
         max_point = new_point;
      }
      else if value < min_value
      {
         min_value = value;
      }
      iter += 1;
   }

   (max_point.value, f.to_hypercube(max_point.coordinates))
}
