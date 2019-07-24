use crate::point::*;
use crate::function::*;
use std::hash::{Hash, Hasher};
use statrs::distribution::{Normal, Continuous, Univariate};

/// represents a simplex
pub struct Simplex
{
   pub corners: Vec<Point>,
   pub center: Coordinates,
   ratio: f64
}

impl Simplex
{
   /// creates a new simplex
   fn new(corners: Vec<Point>, ratio: f64) -> Simplex
   {
      let center = Point::average_coordinate(&corners);
      Simplex { corners, center, ratio }
   }

   /// builds the initial unit simplex with one point per dimension plus an origin
   pub fn initial_simplex(f: &TargetFunction) -> Simplex
   {
      // builds one corner per dimension
      let min_coordinates = vec![0.; f.dimension]; // vector of zero
      let mut corners: Vec<Point> = (0..f.dimension).map(|i| {
                                                       let mut coordinates = min_coordinates.clone();
                                                       coordinates[i] = 1.;
                                                       let value = f.evaluate(&coordinates);
                                                       Point { coordinates, value }
                                                    })
                                                    .collect();

      // adds the corner corresponding to the origin
      let min_corner = Point { value: f.evaluate(&min_coordinates), coordinates: min_coordinates };
      corners.push(min_corner);

      // assemble the simplex
      Simplex::new(corners, 1.)
   }

   /// takes a simplex and splits it
   pub fn split(self, new_point: &Point) -> Vec<Simplex>
   {
      let mut result = vec![];
      let distances: Box<[f64]> = self.corners
                                      .iter()
                                      .map(|c| &c.coordinates)
                                      .map(|c| Point::distance(c, &new_point.coordinates))
                                      .collect();
      let total_distance: f64 = distances.iter().sum();
      for i in 0..self.corners.len()
      {
         // we refuse simplex reduced to a point
         if distances[i] > 1e-15
         {
            // builds the corners of the new simplex
            let mut corners = self.corners.clone();
            corners[i] = new_point.clone();

            // computes the ratio of the child
            // which is the ratio of its father multiplied by the fraction of its father occupied by the child
            let ratio = self.ratio * (distances[i] / total_distance);

            let simplex = Simplex::new(corners, ratio);
            result.push(simplex);
         }
      }
      result
   }

   /// returns a score for a simplex
   pub fn evaluate(&self, difference: f64, exploration_preference: f64) -> f64
   {
      // computes the distance from the center to each corner
      let inverse_distances: Vec<f64> =
         self.corners.iter().map(|c| 1. / Point::distance(&c.coordinates, &self.center)).collect();
      let total_inverse_distance: f64 = inverse_distances.iter().sum();

      // computes the value of the center, interpolated from the corners
      let interpolated_value =
         self.corners.iter().zip(inverse_distances.iter()).map(|(c, d)| c.value * d).sum::<f64>()
         / total_inverse_distance;

      // computes the number of split needed to reach the given ratio if we start from a regular simplex
      let dim = self.center.len() as f64;
      let split_number = self.ratio.log(dim + 1.).abs();

      interpolated_value - exploration_preference * difference * split_number
   }
}

//-----------------------------------------------------------------------------
// TRAITS

/// workaround since floats cannot be hashed
impl Hash for Simplex
{
   /// relies on a hash of the bit representation of the coordinates of the center of the simplex
   fn hash<H: Hasher>(&self, state: &mut H)
   {
      self.center.iter().map(|x| x.to_bits()).collect::<Vec<u64>>().hash(state);
   }
}

impl PartialEq for Simplex
{
   /// two Simplex are equal if they have the exact same center
   fn eq(&self, other: &Self) -> bool
   {
      self.center == other.center
   }
}

impl Eq for Simplex {}
