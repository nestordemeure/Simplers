mod point;
mod simplex;
mod algorithm;
mod search_space;
use point::Coordinates;
use algorithm::Optimizer;

/// test function
#[allow(dead_code)]
fn g(v: &Coordinates) -> f64
{
   let x = v[0];
   let y = v[1];
   1. + 1. / ((x + 3.) * (x + 3.) + y * y).exp() + 0.5 / ((x + 1.).powf(2.) + (y + 2.).powf(2.)).exp()
}

/// test function defined as a lambda
#[allow(dead_code)]
static F: fn(&Coordinates) -> f64 = |v| -((v[0] - 0.2).powf(2.) + (v[1] - 0.3).powf(2.)).sqrt();

fn main()
{
   // test F
   let input_interval_f = vec![(0., 1.), (0., 1.)];
   let nb_iter = 30;
   let (best_value_f, best_coordinates_f) = Optimizer::optimize(F, input_interval_f, nb_iter);
   println!("best value F : {} in [{}, {}]", best_value_f, best_coordinates_f[0], best_coordinates_f[1]);

   // test g with iterator
   let input_interval_g = vec![(-10., 10.), (-10., 10.)];
   let optimizer = Optimizer::new(g, input_interval_g).set_exploration_depth(5);
   let (best_value_g, best_coordinates_g) = optimizer.skip(300).next().unwrap();
   println!("best value g : {} in [{}, {}]", best_value_g, best_coordinates_g[0], best_coordinates_g[1]);
}
