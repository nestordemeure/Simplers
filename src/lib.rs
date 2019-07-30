//!A Rust implementation of the [Simple(x)](https://github.com/chrisstroemel/Simple) global optimization algorithm.
//!
//!This algorithm, which should not be confused with the [simplex algorithm](https://en.wikipedia.org/wiki/Simplex_algorithm), is closest to [bayesian optimization](https://en.wikipedia.org/wiki/Bayesian_optimization).
//!Its strengths compared to bayesian optimization would be the ability to deal with a large number of sample and high dimension gracefully.
//!
//!There are two ways to use the algorithm, either use one of the `Optimizer::minimize` / `Optimizer::maximize` functions :
//!
//!```rust
//!# fn main() {
//!let f = |v| v[0] + v[1] * v[2];
//!let input_interval = vec![(-10., 10.), (-20., 20.), (0., 5.)];
//!let nb_iterations = 100;
//!
//!let (max_value, coordinates) = Optimizer::maximize(f, input_interval, nb_iterations);
//!println!("max value: {} found in [{}, {}, {}]", max_value, coordinates[0], coordinates[1], coordinates[2]);
//!# }
//!```
//!
//!Or use an iterator if you want to set `exploration_depth` to an exotic value or to have fine grained control on the stopping criteria :
//!
//!```rust
//!# fn main() {
//!let f = |v| v[0] * v[1];
//!let input_interval = vec![(-10., 10.), (-20., 20.)];
//!let should_minimize = true;
//!
//!// sets `exploration_depth` to be greedy
//!// runs the search for 30 iterations
//!// then waits until we find a point good enough
//!// finally stores the best value so far
//!let (min_value, coordinates) = Optimizer::new(f, input_interval, should_minimize)
//!                                       .set_exploration_depth(10)
//!                                       .skip(30)
//!                                       .take_while(|(value,coordinates)| value > 1. )
//!                                       .next().unwrap();
//!
//!println!("min value: {} found in [{}, {}]", min_value, coordinates[0], coordinates[1]);
//!# }
//!```
#![deny(missing_docs,
        //missing_debug_implementations,
        missing_copy_implementations,
        trivial_casts,
        trivial_numeric_casts,
        unsafe_code,
        unstable_features,
        unused_import_braces,
        unused_qualifications)]

mod point;
mod simplex;
mod search_space;
mod algorithm;
pub use algorithm::Optimizer;

#[cfg(test)]
mod tests
{
   /*use crate::algorithm::Optimizer;
   use argmin_testfunctions::*;
   const ITER: usize = 100;

   #[test]
   fn test_styblinski_tang()
   {
      const DIM: usize = 5;
      let input_interval: Vec<(f64, f64)> = (1..=DIM).map(|_| (-5., 5.)).collect();
      let (best_value, best_coordinates) = Optimizer::minimize(styblinski_tang, input_interval, ITER);
      let true_best_value = styblinski_tang(&[-2.903534; DIM]);
      println!("best value : {} in [{}, {}] (target: {})",
               best_value, best_coordinates[0], best_coordinates[1], true_best_value);
   }

   #[test]
   fn test_himmelblau_iterator()
   {
      let input_interval: Vec<(f64, f64)> = vec![(-5., 5.), (-5., 5.)];
      Optimizer::new(himmelblau, input_interval, true)
         .take(ITER)
         .enumerate()
         .for_each(|(i,(v,c))| println!("iter {}: {} in {:?}", i, v, c));
   }*/
}
