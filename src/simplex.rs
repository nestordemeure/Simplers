use crate::point::*;
use crate::search_space::*;
use std::hash::{Hash, Hasher};
use std::rc::Rc;
use num_traits::Float;

/// represents a simplex
#[derive(Clone)]
pub struct Simplex<CoordFloat: Float, ValueFloat: Float>
{
   /// the coordinate+evaluations of the corners of the simplex
   pub corners: Vec<Rc<Point<CoordFloat, ValueFloat>>>,
   /// the coordinates of the center of the simplex (which is where it is evaluated)
   pub center: Coordinates<CoordFloat>,
   /// what was the difference between the best value and the worst value when the simplex was last evaluated ?
   pub difference: ValueFloat,
   /// which fraction of the original simplex does this simplex represents ?
   ratio: ValueFloat
}

impl<CoordFloat: Float, ValueFloat: Float> Simplex<CoordFloat, ValueFloat>
{
   /// creates a new simplex
   fn new(corners: Vec<Rc<Point<CoordFloat, ValueFloat>>>, ratio: ValueFloat, difference: ValueFloat) -> Self
   {
      let center = Point::average_coordinate(&corners);
      Simplex { corners, center, ratio, difference }
   }

   /// builds the initial unit simplex with one point per axis plus an origin at zero
   pub fn initial_simplex(search_space: &SearchSpace<CoordFloat, ValueFloat>) -> Self
   {
      // origin, a vector of zero
      let origin = vec![CoordFloat::zero(); search_space.dimension].into_boxed_slice();

      // builds one corner per dimension
      let mut corners: Vec<_> = (0..search_space.dimension).map(|i| {
                                                                      let mut coordinates = origin.clone();
                                                                      coordinates[i] = CoordFloat::one();
                                                                      let value =
                                                                         search_space.evaluate(&coordinates);
                                                                      Rc::new(Point { coordinates, value })
                                                                   })
                                                                   .collect();

      // adds the corner corresponding to the origin
      let min_corner = Point { value: search_space.evaluate(&origin), coordinates: origin };
      corners.push(Rc::new(min_corner));

      // assemble the simplex
      Simplex::new(corners, ValueFloat::one(), ValueFloat::zero())
   }

   /// takes a simplex and splits it around a point
   /// difference is the best value so far minus the worst value so far
   pub fn split(self, new_point: Rc<Point<CoordFloat, ValueFloat>>, difference: ValueFloat) -> Vec<Self>
   {
      // computes the distance between the new point and each corners of the simplex
      let distances: Box<[ValueFloat]> = self.corners
                                      .iter()
                                      .map(|c| &c.coordinates)
                                      .map(|c| Point::distance(c, &new_point.coordinates))
                                      .collect();
      let total_distance: ValueFloat = distances.iter().copied().fold(ValueFloat::zero(), ::std::ops::Add::add);

      // computes each sub simplex
      let mut result = vec![];
      let min_distance = ValueFloat::from(10.).unwrap() * Float::epsilon();
      for i in 0..self.corners.len()
      {
         // we refuse simplex reduced to a point
         if distances[i] > min_distance
         {
            // builds the corners of the new simplex
            let mut corners = self.corners.clone();
            corners[i] = new_point.clone();

            // computes the ratio of the child
            // which is the ratio of its father multiplied by the fraction of its father occupied by the child
            let ratio = self.ratio * (distances[i] / total_distance);

            // builds the new simplex and adds it to the list
            let simplex = Simplex::new(corners, ratio, difference);
            result.push(simplex);
         }
      }
      result
   }

   /// returns a score for a simplex
   pub fn evaluate(&self, exploration_depth: ValueFloat) -> ValueFloat
   {
      // computes the inverse of the distance from the center to each corner
      let inverse_distances: Vec<ValueFloat> =
         self.corners.iter().map(|c| ValueFloat::one() / Point::distance(&c.coordinates, &self.center)).collect();
      let total_inverse_distance: ValueFloat = inverse_distances.iter().copied().fold(ValueFloat::zero(), ::std::ops::Add::add);

      // computes the value of the center, interpolated from the corners
      let interpolated_value =
         self.corners.iter().zip(inverse_distances.iter()).map(|(c, &d)| c.value * d).fold(ValueFloat::zero(), ::std::ops::Add::add)
         / total_inverse_distance;

      // computes the number of split needed to reach the given ratio if we start from a regular simplex
      let dim = ValueFloat::from(self.center.len()).unwrap();
      let split_number = self.ratio.log(dim + ValueFloat::one()).abs();

      // insures that the difference (which is positiv by construction) is non zero
      let difference = self.difference + Float::epsilon();

      interpolated_value - difference * (split_number / exploration_depth)
   }
}

//-----------------------------------------------------------------------------
// TRAITS FOR PRIORITY QUEUE

/// workaround since floats cannot be hashed
impl<CoordFloat: Float, ValueFloat: Float> Hash for Simplex<CoordFloat, ValueFloat>
{
   /// relies on a hash of the bit representation of the coordinates of the center of the simplex
   fn hash<H: Hasher>(&self, state: &mut H)
   {
      // TODO I will drop `.to_f64().unwrap()`
      // once the relevant [issue](https://github.com/rust-num/num-traits/issues/123) is resolved
      self.center.iter().map(|&x| x.to_f64().unwrap().to_bits()).collect::<Box<[u64]>>().hash(state);
   }
}

impl<CoordFloat: Float, ValueFloat: Float> PartialEq for Simplex<CoordFloat, ValueFloat>
{
   /// two Simplex are equal if they have the exact same center
   fn eq(&self, other: &Self) -> bool
   {
      self.center == other.center
   }
}

impl<CoordFloat: Float, ValueFloat: Float> Eq for Simplex<CoordFloat, ValueFloat> {}
