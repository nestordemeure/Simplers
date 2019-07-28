use std::rc::Rc;
use ordered_float::OrderedFloat;

/// represents coordinates in space
pub type Coordinates = Box<[f64]>;

/// represents a evaluated coordinates in space
#[derive(Clone)]
pub struct Point
{
   pub coordinates: Coordinates,
   pub value: f64
}

impl Point
{
   /// computes the euclidian distance between two sets of coordinate
   pub fn distance_hypercube(p1: &Coordinates, p2: &Coordinates) -> f64
   {
      let sum1: f64 = p1.iter().sum();
      let max1 =
         *p1.iter().max_by_key(|&&c| OrderedFloat(c)).expect("You should have at least one coordinate.");
      let ratio1 = if max1 == 0. { 0. } else { max1 / sum1 };
      let sum2: f64 = p2.iter().sum();
      let max2 =
         *p2.iter().max_by_key(|&&c| OrderedFloat(c)).expect("You should have at least one coordinate.");
      let ratio2 = if max2 == 0. { 0. } else { max2 / sum2 };
      p1.iter().zip(p2.iter()).map(|(x1, x2)| (x1 * ratio1 - x2 * ratio2).powf(2.)).sum::<f64>().sqrt()
   }

   /// computes the euclidian distance between two sets of coordinate
   pub fn distance_simplex(p1: &Coordinates, p2: &Coordinates) -> f64
   {
      p1.iter().zip(p2.iter()).map(|(x, y)| (x - y).powf(2.)).sum::<f64>().sqrt()
   }

   /// adds the point to the coordinates and returns the coordinates
   fn add_to(&self, coordinates: Coordinates) -> Coordinates
   {
      coordinates.iter().zip(self.coordinates.iter()).map(|(x, y)| x + y).collect()
   }

   fn to_simplex(c: Coordinates) -> Coordinates
   {
      let sum: f64 = c.iter().sum();
      let max = c.iter()
                 .map(|&c| OrderedFloat(c))
                 .max()
                 .map(|c| *c)
                 .expect("You should have at least one coordinate.");
      let ratio = if sum == 0. { 0. } else { max / sum };
      c.into_iter().map(|x| x * ratio).collect()
   }

   fn to_hypercube(c: Coordinates) -> Coordinates
   {
      // gets the ratio to go from the unit hypercube to the unit simplex
      let sum: f64 = c.iter().sum();
      let max = c.iter()
                 .max_by_key(|&&c| OrderedFloat(c))
                 .map(|c| *c)
                 .expect("You should have at least one coordinate.");
      let ratio = if max == 0. { 0. } else { sum / max };
      // goes from the simplex to the target hypercube
      c.into_iter().map(|x| x * ratio).collect()
   }

   /// computes the average of the coordinates
   pub fn average_coordinate(points: &[Rc<Point>]) -> Coordinates
   {
      let length = points.len() as f64;
      let mut points = points.iter();
      let first = points.next().expect("You need at least one coordinate to average!").coordinates.clone();
      let sum = points.fold(first, |acc, x| x.add_to(acc));
      sum.iter().map(|sum| sum / length).collect()
      /*let length = points.len() as f64;
      let mut points = points.iter();
      let first = points.next().expect("You need at least one coordinate to average!").coordinates.clone();
      let sum = points.fold(Self::to_hypercube(first), |acc, x| x.add_to(Self::to_hypercube(acc.clone())));
      let result: Coordinates = sum.iter().map(|sum| sum / length).collect();
      Self::to_simplex(result)*/
   }
}
