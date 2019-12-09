mod point;
mod simplex;
mod search_space;
mod algorithm;
pub use algorithm::Optimizer;

fn main()
{
   {
      let f = |v: &[f64]| v[0] * v[1];
      let input_interval = vec![(-10., 10.), (-20., 20.)];
      let should_minimize = true;

      // runs the search for 30 iterations
      // then waits until we find a point good enough
      // finally stores the best value so far
      let (min_value, coordinates) =
         Optimizer::new(&f, &input_interval, should_minimize).skip(30)
                                                             .skip_while(|(value, coordinates)| *value > 1.)
                                                             .next()
                                                             .unwrap();

      println!("min value: {} found in [{}, {}]", min_value, coordinates[0], coordinates[1]);
   }

   {
      let f = |v: &[f64]| v[0] + v[1];
      let input_interval = vec![(-10., 10.), (-20., 20.)];
      let nb_iterations = 100;
      let (max_value, coordinates) = Optimizer::maximize(&f, &input_interval, nb_iterations);
      println!("max value: {} found in [{}, {}]", max_value, coordinates[0], coordinates[1]);
   }

   {
      let f = |v: &[f64]| v[0] * v[1];
      let input_interval = vec![(-10., 10.), (-20., 20.)];
      let should_minimize = true;

      // runs the search for 30 iterations
      // then waits until we find a point good enough
      // finally stores the best value so far
      let (min_value, coordinates) =
         Optimizer::new(&f, &input_interval, should_minimize).skip(30)
                                                             .skip_while(|(value, coordinates)| *value > 1.)
                                                             .next()
                                                             .unwrap();

      println!("min value: {} found in [{}, {}]", min_value, coordinates[0], coordinates[1]);
   }
}
