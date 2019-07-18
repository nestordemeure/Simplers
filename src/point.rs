
//-----------------------------------------------------------------------------
// POINT

/// represents coordinates in space
pub type Coordinates = Vec<f64>;

/// represents an already evaluated point in space
#[derive(Clone)]
pub struct Point
{
   pub coordinates: Coordinates,
   pub value: f64
}

impl Point
{
   /// computes the euclidian distance between two points
   pub fn distance(p1: &Coordinates, p2: &Coordinates) -> f64
   {
      p1.iter().zip(p2.iter()).map(|(x, y)| (x - y) * (x - y)).sum::<f64>().sqrt()
   }

   /// adds the point into the coordinates and returns the coordinates
   fn add_to(&self, coordinates: Coordinates) -> Coordinates
   {
      coordinates.iter().zip(self.coordinates.iter()).map(|(x, y)| x + y).collect()
   }

   /// computes the average of the coordinates
   pub fn average_coordinate(points: &[Point]) -> Coordinates
   {
      let length = points.len() as f64;
      let mut points = points.iter();
      let first = points.next().expect("Tried to average zero coordinates!").coordinates.clone();
      let sum = points.fold(first, |acc, x| x.add_to(acc));
      sum.iter().map(|sum| sum / length).collect()
   }
}