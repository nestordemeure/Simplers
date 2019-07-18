mod point;
mod simplex;
mod algorithm;
mod function;
use point::Coordinates;
use algorithm::simple_optimizer;

/// test function
fn g(v: &Coordinates) -> f64
{
   let x = v[0];
   let y = v[1];
   1. + 1. / ((x + 3.) * (x + 3.) + y * y).exp() + 0.5 / ((x + 1.).powf(2.) + (y + 2.).powf(2.)).exp()
}

/// test function defined as a lambda
static F: fn(&Coordinates) -> f64 = |v| -((v[0] - 0.2).powf(2.) + (v[1] - 0.3).powf(2.)).sqrt();

fn main()
{
   //let input_interval = vec![(-10., 10.), (-10., 10.)];
   let input_interval = vec![(0., 1.), (0., 1.)];
   let nb_iter = 30;
   let (best_value, best_coordinates) = simple_optimizer(F, input_interval, nb_iter);
   println!("best value : {} in [{}, {}]", best_value, best_coordinates[0], best_coordinates[1]);
}
