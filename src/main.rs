mod point;
mod simplex;
mod queue;
mod algorithm;
use point::Coordinates;
use algorithm::simple_optimizer;

//-----------------------------------------------------------------------------
// TEST

fn main()
{
   //let f: fn(&Coordinates) -> f64 = |v| -((v[0] - 0.2).powf(2.) + (v[1] - 0.3).powf(2.)).sqrt();
   let f: fn(&Coordinates) -> f64 = |v| {
      let x = v[0];
      let y = v[1];
      1. + 1. / ((x + 3.) * (x + 3.) + y * y).exp() + 0.5 / ((x + 1.).powf(2.) + (y + 2.).powf(2.)).exp()
   };
   let input_interval = vec![(-10., 10.), (-10., 10.)];
   let nb_iter = 300;
   let exploration_preference = 0.05;
   let best_value = simple_optimizer(f, &input_interval, nb_iter, exploration_preference);
   println!("best value : {}", best_value);
}
