use crate::point::*;
use crate::search_space::*;
use std::hash::{Hash, Hasher};

/// represents a simplex
pub struct Simplex
{
   /// the coordinate+evaluations of the corners of the simplex
   pub corners: Vec<Point>,
   /// the coordinates of the center of the simplex (which is where it is evaluated)
   pub center: Coordinates,
   /// what was the difference between the best value and the worst value when the simplex was last evaluated ?
   pub difference: f64,
   /// which fraction of the original simplex does this simplex represents ?
   ratio: f64
}

impl Simplex
{
   /// creates a new simplex
   fn new(corners: Vec<Point>, ratio: f64, difference: f64) -> Simplex
   {
      let center = Point::average_coordinate(&corners);
      Simplex { corners, center, ratio, difference }
   }

   /// builds the initial unit simplex with one point per axis plus an origin at zero
   pub fn initial_simplex(search_space: &SearchSpace) -> Simplex
   {
      // origin, a vector of zero
      let origin = vec![0.; search_space.dimension].into_boxed_slice();

      // builds one corner per dimension
      let mut corners: Vec<Point> =
         (0..search_space.dimension).map(|i| {
                                       let mut coordinates = origin.clone();
                                       coordinates[i] = 1.;
                                       let value = search_space.evaluate(&coordinates);
                                       Point { coordinates, value }
                                    })
                                    .collect();

      // adds the corner corresponding to the origin
      let min_corner = Point { value: search_space.evaluate(&origin), coordinates: origin };
      corners.push(min_corner);

      // assemble the simplex
      Simplex::new(corners, 1., 0.)
   }

   /// takes a simplex and splits it around a point
   /// difference is the best value so far minus the worst value so far
   pub fn split(self, new_point: &Point, difference: f64) -> Vec<Simplex>
   {
      // computes the distance between the new point and each corners of the simplex
      let distances: Box<[f64]> = self.corners
                                      .iter()
                                      .map(|c| &c.coordinates)
                                      .map(|c| Point::distance(c, &new_point.coordinates))
                                      .collect();
      let total_distance: f64 = distances.iter().sum();

      // computes each sub simplex
      let mut result = vec![];
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

            // builds the new simplex and adds it to the list
            let simplex = Simplex::new(corners, ratio, difference);
            result.push(simplex);
         }
      }
      result
   }

   /// returns a score for a simplex
   pub fn evaluate(&self, exploration_depth: f64) -> f64
   {
      // computes the inverse of the distance from the center to each corner
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

      // insures that the difference (which is positiv by construction) is non zero
      let difference = self.difference + std::f64::EPSILON;

      interpolated_value - difference * (split_number / exploration_depth)
   }
}

//-----------------------------------------------------------------------------
// TRAITS FOR PRIORITY QUEUE

/// workaround since floats cannot be hashed
impl Hash for Simplex
{
   /// relies on a hash of the bit representation of the coordinates of the center of the simplex
   fn hash<H: Hasher>(&self, state: &mut H)
   {
      self.center.iter().map(|x| x.to_bits()).collect::<Box<[u64]>>().hash(state);
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
