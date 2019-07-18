use crate::point::*;
use std::hash::{Hash, Hasher};

//-----------------------------------------------------------------------------
// SIMPLEX

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
      //println!("center: x{} y{}", center[0], center[1]);
      Simplex { corners, center, ratio }
   }

   /// takes an hypercube and produces a simplex that contains the given hypercube
   /// NOTE: the ratio is set to 1. and the difference to 0.
   /// WARNING: this will create coordinates that are out of the given hypercube
   pub fn from_hypercube(hypercube: &[(f64, f64)], f: fn(&Coordinates) -> f64) -> Simplex
   {
      // builds all the corners of the simplex
      let min_coordinates: Vec<f64> = hypercube.iter().map(|(inf, _sup)| *inf).collect();
      let mut corners = vec![];
      for i in 0..hypercube.len()
      {
         let mut coordinates = min_coordinates.clone();
         let (inf_i, sup_i) = hypercube[i];
         coordinates[i] = sup_i + (sup_i - inf_i);

         let value = f(&coordinates);

         let corner = Point { coordinates, value };
         corners.push(corner);
      }

      // adds the corner that corresponds to the min coordinates
      let value_at_min = f(&min_coordinates);
      let corner_min = Point { coordinates: min_coordinates, value: value_at_min };
      corners.push(corner_min);

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
         // builds the corners of the new simplex
         let mut corners = self.corners.clone();
         corners[i] = new_point.clone();

         // computes the ratio of the child
         // which is the ratio of its father multiplied by the fraction of its father occupied by the child
         let ratio = self.ratio * (distances[i] / total_distance);

         let simplex = Simplex::new(corners, ratio);
         result.push(simplex);
      }
      result
   }

   /// returns a score for a simplex
   pub fn evaluate(&self, exploration_preference: f64, difference: f64) -> f64
   {
      // computes the value interpolated from the corners
      let mut total_inverse_distance = 0.;
      let interpolated_value = self.corners
                                   .iter()
                                   .map(|c| {
                                      let d = 1. / Point::distance(&c.coordinates, &self.center);
                                      total_inverse_distance += d;
                                      c.value * d
                                   })
                                   .sum::<f64>()
                               / total_inverse_distance;

      // computes the number of split needed to reach the given ratio if we start from a regular simplex
      let dim = self.center.len() as f64;
      let split_number = self.ratio.log(dim + 1.).abs();

      if difference == 0.
      {
         -split_number
      }
      else
      {
         //println!("{} - {} * {} * {}", interpolated_value, exploration_preference, difference, split_number);
         interpolated_value - exploration_preference * difference * split_number
      }
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
