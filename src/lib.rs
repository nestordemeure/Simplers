/*#![deny(missing_docs,
missing_debug_implementations,
missing_copy_implementations,
trivial_casts,
trivial_numeric_casts,
unsafe_code,
unstable_features,
unused_import_braces,
unused_qualifications)]*/

mod point;
mod simplex;
mod search_space;
pub mod algorithm;
pub use algorithm::Optimizer;

#[cfg(test)]
mod tests
{
   use crate::algorithm::Optimizer;
   use argmin_testfunctions::*;
   const ITER: usize = 100;

   #[test]
   fn test_pichety()
   {
      let input_interval = vec![(0., 1.), (0., 1.)];
      let (best_value, best_coordinates) = Optimizer::minimize(picheny, input_interval, ITER);
      let true_best_value = picheny(&[0.5, 0.25]);
      println!("best value : {} in [{}, {}] (target: {})",
               best_value, best_coordinates[0], best_coordinates[1], true_best_value);
   }

   #[test]
   fn test_ackley()
   {
      const DIM: usize = 5;
      let input_interval: Vec<(f64, f64)> = (1..=DIM).map(|_| (-32.768, 32.768)).collect();
      let (best_value, best_coordinates) = Optimizer::minimize(ackley, input_interval, ITER);
      let true_best_value = ackley(&[0.; DIM]);
      println!("best value : {} in [{}, {}] (target: {})",
               best_value, best_coordinates[0], best_coordinates[1], true_best_value);
   }

   #[test]
   fn test_goldsteinprice()
   {
      let input_interval: Vec<(f64, f64)> = vec![(-2., 2.), (-2., 2.)];
      let (best_value, best_coordinates) = Optimizer::minimize(goldsteinprice, input_interval, ITER);
      let true_best_value = goldsteinprice(&[0., -1.]);
      println!("best value : {} in [{}, {}] (target: {})",
               best_value, best_coordinates[0], best_coordinates[1], true_best_value);
   }

   #[test]
   fn test_himmelblau()
   {
      let input_interval: Vec<(f64, f64)> = vec![(-5., 5.), (-5., 5.)];
      let (best_value, best_coordinates) = Optimizer::minimize(himmelblau, input_interval, ITER);
      let true_best_value = himmelblau(&[3., 2.]);
      println!("best value : {} in [{}, {}] (target: {})",
               best_value, best_coordinates[0], best_coordinates[1], true_best_value);
   }

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
   }
}
