use crate::point::*;
use crate::simplex::*;

//-----------------------------------------------------------------------------
// PARAMETERS

/// defines the parameters of the algorithm
pub struct Parameters
{
   exploration_preference: f64,
   f: fn(&Coordinates) -> f64,
   points: Vec<Coordinates>
}

impl Parameters
{
   /// creates a set of parameters with default value
   /// f is the function to optimize
   /// input_interval is the hypercube in which the parameters are defined
   pub fn new(f: fn(&Coordinates) -> f64, input_interval: &[(f64, f64)]) -> Parameters
   {
      let exploration_preference = 0.15;
      let points = unimplemented!(); // TODO turn the input interval into a set of points
      Parameters { exploration_preference, f, points }
   }

   /// change the exploration preference and return the updated parameters
   pub fn set_exploration_preference(mut self, preference: f64) -> Self
   {
      assert!(preference >= 0.);
      self.exploration_preference = preference;
      self
   }

   /// probes a given set of coordinates in order to speed up the search
   pub fn probe_coordinates(mut self, coordinate: Coordinates) -> Self
   {
      self.points.push(coordinate);
      self
   }

   /// TODO runs the given function on all points in order to build the initial state of the optimizer
   pub fn initialise(self)
   {
      // turn points into evaluated simplexes
      // builds an algorithm out of that
      unimplemented!()
   }
}
