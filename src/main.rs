mod point;
mod simplex;
mod algorithm;
mod search_space;
use algorithm::Optimizer;
use argmin_testfunctions::*;

fn main()
{
   let nb_iter = 300;

   {
      // pichety
      let input_interval = vec![(0., 1.), (0., 1.)];
      let (best_value, best_coordinates) = Optimizer::minimize(picheny, input_interval, nb_iter);
      let true_best_value = picheny(&[0.5, 0.25]);
      println!("best value : {} in [{}, {}] (target: {})",
               best_value, best_coordinates[0], best_coordinates[1], true_best_value);
   }

   {
      // Ackley (the optimization is fairly bad on this function full of local minimums)
      const DIM: usize = 5;
      let input_interval: Vec<(f64, f64)> = (1..=DIM).map(|_| (-32.768, 32.768)).collect();
      let (best_value, best_coordinates) = Optimizer::minimize(ackley, input_interval, nb_iter);
      let true_best_value = ackley(&[0.; DIM]);
      println!("best value : {} in [{}, {}] (target: {})",
               best_value, best_coordinates[0], best_coordinates[1], true_best_value);
   }

   {
      // goldsteinprice
      let input_interval: Vec<(f64, f64)> = vec![(-2., 2.), (-2., 2.)];
      let (best_value, best_coordinates) = Optimizer::minimize(goldsteinprice, input_interval, nb_iter);
      let true_best_value = goldsteinprice(&[0., -1.]);
      println!("best value : {} in [{}, {}] (target: {})",
               best_value, best_coordinates[0], best_coordinates[1], true_best_value);
   }

   {
      // himmelblau
      let input_interval: Vec<(f64, f64)> = vec![(-5., 5.), (-5., 5.)];
      let (best_value, best_coordinates) = Optimizer::minimize(himmelblau, input_interval, nb_iter);
      let true_best_value = himmelblau(&[3., 2.]);
      println!("best value : {} in [{}, {}] (target: {})",
               best_value, best_coordinates[0], best_coordinates[1], true_best_value);
   }

   {
      // styblinski_tang
      const DIM: usize = 5;
      let input_interval: Vec<(f64, f64)> = (1..=DIM).map(|_| (-5., 5.)).collect();
      let (best_value, best_coordinates) = Optimizer::minimize(styblinski_tang, input_interval, nb_iter);
      let true_best_value = styblinski_tang(&[-2.903534; DIM]);
      println!("best value : {} in [{}, {}] (target: {})",
               best_value, best_coordinates[0], best_coordinates[1], true_best_value);
   }
}
