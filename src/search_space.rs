use crate::point::*;
use ordered_float::OrderedFloat;
use num_traits::Float;

/// encapsulate a function and its domain of definition
pub struct SearchSpace<'f_lifetime, CoordFloat: Float, ValueFloat: Float>
{
   f: &'f_lifetime dyn Fn(&[CoordFloat]) -> ValueFloat,
   hypercube: Vec<(CoordFloat, CoordFloat)>,
   pub minimize: bool,
   pub dimension: usize
}

impl<'f_lifetime, CoordFloat: Float, ValueFloat: Float> SearchSpace<'f_lifetime, CoordFloat, ValueFloat>
{
   /// builds a new search space that encapsulate both the function to evaluate and its domain of definition
   pub fn new(f: &'f_lifetime impl Fn(&[CoordFloat]) -> ValueFloat,
              hypercube: &[(CoordFloat, CoordFloat)],
              minimize: bool)
              -> Self
   {
      let dimension = hypercube.len();
      SearchSpace { f, hypercube: hypercube.to_vec(), minimize, dimension }
   }

   /// Converts coordinates from the hypercube to the unit simplex
   /// This fucntion is useful when one wants to suggest a point to the algorithm
   /// for the formula used, see: https://math.stackexchange.com/a/385071/495073
   #[allow(dead_code)]
   pub fn to_simplex(&self, c: Coordinates<CoordFloat>) -> Coordinates<CoordFloat>
   {
      // goes to the unit hypercube
      let c: Coordinates<CoordFloat> =
         c.into_iter().zip(self.hypercube.iter()).map(|(&x, &(inf, sup))| (x - inf) / (sup - inf)).collect();
      // goes to the unit simplex
      let sum = c.iter().map(|&c| c).fold(CoordFloat::zero(), ::std::ops::Add::add); // sum
      let max = c.iter()
                 .map(|&c| c)
                 .max_by_key(|&c| OrderedFloat(c))
                 .expect("You should have at least one coordinate.");
      let ratio = if sum.is_zero() { CoordFloat::zero() } else { max / sum };
      c.into_iter().map(|&x| x * ratio).collect()
   }

   /// converts coordinates from the unit simplex to the hypercube
   /// formula deduced from: https://math.stackexchange.com/a/385071/495073
   pub fn to_hypercube(&self, c: Coordinates<CoordFloat>) -> Coordinates<CoordFloat>
   {
      // gets the ratio to go from the unit hypercube to the unit simplex
      let sum = c.iter().map(|&c| c).fold(CoordFloat::zero(), ::std::ops::Add::add); // sum
      let max = c.iter()
                 .map(|&c| c)
                 .max_by_key(|&c| OrderedFloat(c))
                 .expect("You should have at least one coordinate.");
      let ratio = if max.is_zero() { CoordFloat::zero() } else { sum / max };
      // goes from the simplex to the target hypercube
      c.into_iter()
       .zip(self.hypercube.iter())
       .map(|(&x, &(inf, sup))| inf + x * ratio * (sup - inf))
       .collect()
   }

   /// takes coordinates in the unit simplex and evaluate them with the function
   pub fn evaluate(&self, c: &Coordinates<CoordFloat>) -> ValueFloat
   {
      let c_hypercube = self.to_hypercube(c.clone());
      let evaluation = (self.f)(&c_hypercube);
      if self.minimize
      {
         -evaluation
      }
      else
      {
         evaluation
      }
   }
}