mod point;
mod simplex;
mod algorithm;
mod search_space;
use algorithm::Optimizer;
use argmin_testfunctions::*;

fn main()
{
   let nb_iter = 3000;

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
      const DIM: usize = 50;
      let input_interval: Vec<(f64, f64)> = (1..=DIM).map(|_| (-32.768, 32.768)).collect();
      let (best_value, best_coordinates) = Optimizer::minimize(ackley, input_interval, nb_iter);
      let true_best_value = ackley(&[0.; DIM]);
      println!("best value : {} in [{}, {}] (target: {})",
               best_value, best_coordinates[0], best_coordinates[1], true_best_value);
   }
}
