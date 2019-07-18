use crate::point::{Coordinates, Point};
use crate::simplex::Simplex;
use crate::queue::*;

use ordered_float::OrderedFloat;
use std::cmp::{min, max};

//-----------------------------------------------------------------------------
// ALGORITHM

/// takes a fucntion to maximise, an array of input intervals and a number of iterations
pub fn simple_optimizer(f: fn(&Coordinates) -> f64,
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
