//!A Rust implementation of the [Simple(x)](https://github.com/chrisstroemel/Simple) global optimization algorithm.
//!
//!This algorithm, which should not be confused with the [simplex algorithm](https://en.wikipedia.org/wiki/Simplex_algorithm), is closest to [bayesian optimization](https://en.wikipedia.org/wiki/Bayesian_optimization).
//!Its strengths compared to bayesian optimization are the ability to deal with a large number of sample and high dimension efficiently.
//!
//!There are two ways to use the algorithm, either use one of the `Optimizer::minimize` / `Optimizer::maximize` functions :
//!
//!```rust
//!# use simplers_optimization::Optimizer;
//!# fn main() {
//!let f = |v:&[f64]| v[0] + v[1] * v[2];
//!let input_interval = vec![(-10., 10.), (-20., 20.), (0., 5.)];
//!let nb_iterations = 100;
//!
//!let (max_value, coordinates) = Optimizer::maximize(f, &input_interval, nb_iterations);
//!println!("max value: {} found in [{}, {}, {}]", max_value, coordinates[0], coordinates[1], coordinates[2]);
//!# }
//!```
//!
//!Or use an iterator if you want to set `exploration_depth` to an exotic value or to have fine grained control on the stopping criteria :
//!
//!```rust
//!# use simplers_optimization::Optimizer;
//!# fn main() {
//!let f = |v:&[f64]| v[0] * v[1];
//!let input_interval = vec![(-10., 10.), (-20., 20.)];
//!let should_minimize = true;
//!
//!// sets `exploration_depth` to be greedy
//!// runs the search for 30 iterations
//!// then waits until we find a point good enough
//!// finally stores the best value so far
//!let (min_value, coordinates) = Optimizer::new(f, &input_interval, should_minimize)
//!                                       .set_exploration_depth(10)
//!                                       .skip(30)
//!                                       .skip_while(|(value,coordinates)| *value > 1. )
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
    use crate::algorithm::Optimizer;

    #[test]
    fn test_predict_during_optimization()
    {
        let f = |v: &[f64]| -(v[0] * v[0] + v[1] * v[1]); // minimum at origin
        let input_interval = vec![(-5., 5.), (-5., 5.)];

        let mut optimizer = Optimizer::new(f, &input_interval, true);

        // Run some iterations
        for _ in 0..50
        {
            optimizer.next();
        }

        // Predict at all previously evaluated points — should always return Some
        for (coords, _actual_value) in optimizer.evaluated_points()
        {
            let prediction = optimizer.predict(coords);
            assert!(prediction.is_some(),
                    "predict returned None for previously evaluated point {:?}",
                    coords);
        }

        // Predict at the known minimum (origin) — should return Some
        let prediction_at_origin = optimizer.predict(&[0.0, 0.0]);
        assert!(prediction_at_origin.is_some(), "predict returned None at origin");

        // Predict at corners of the search space — should return Some
        let corners = vec![[-5.0, -5.0], [-5.0, 5.0], [5.0, -5.0], [5.0, 5.0], [0.0, 0.0], [2.5, -1.0]];
        for corner in &corners
        {
            let prediction = optimizer.predict(corner);
            assert!(prediction.is_some(), "predict returned None at {:?}", corner);
        }
    }

    #[test]
    fn test_predict_improves_with_iterations()
    {
        let f = |v: &[f64]| v[0] * v[0] + v[1] * v[1]; // minimum at origin = 0
        let input_interval = vec![(-5., 5.), (-5., 5.)];

        // Run a short optimization
        let mut optimizer_short = Optimizer::new(f, &input_interval, true);
        for _ in 0..10
        {
            optimizer_short.next();
        }

        // Run a longer optimization
        let mut optimizer_long = Optimizer::new(f, &input_interval, true);
        for _ in 0..100
        {
            optimizer_long.next();
        }

        // The longer run should predict closer to 0 at the origin
        let pred_short = optimizer_short.predict(&[0.0, 0.0]).unwrap();
        let pred_long = optimizer_long.predict(&[0.0, 0.0]).unwrap();
        assert!(pred_long.abs() <= pred_short.abs(),
                "longer optimization should predict better at minimum: short={}, long={}",
                pred_short,
                pred_long);
    }
}
