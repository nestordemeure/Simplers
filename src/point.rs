use std::rc::Rc;
use num_traits::Float;

/// Represents coordinates in space.
pub type Coordinates<CoordFloat> = Box<[CoordFloat]>;

/// Represents an evaluated coordinates in space.
#[derive(Clone, Debug)]
pub struct Point<CoordFloat: Float, ValueFloat: Float>
{
   pub coordinates: Coordinates<CoordFloat>,
   pub value: ValueFloat
}

impl<CoordFloat: Float, ValueFloat: Float> Point<CoordFloat, ValueFloat>
{
   /// computes the euclidian distance between two sets of coordinate
   pub fn distance(p1: &Coordinates<CoordFloat>, p2: &Coordinates<CoordFloat>) -> ValueFloat
   {
      p1.iter()
        .zip(p2.iter())
        .map(|(&x, &y)| (x - y).powi(2))
        .map(|x| ValueFloat::from(x).expect("Unable to convert from coordinate type to value type."))
        .fold(ValueFloat::zero(), ::std::ops::Add::add) // sum
        .sqrt()
   }

   /// adds the point to the coordinates and returns the coordinates
   fn add_to(&self, coordinates: Coordinates<CoordFloat>) -> Coordinates<CoordFloat>
   {
      coordinates.iter().zip(self.coordinates.iter()).map(|(&x, &y)| x + y).collect()
   }

   /// computes the average of the coordinates
   pub fn average_coordinate(points: &[Rc<Point<CoordFloat, ValueFloat>>]) -> Coordinates<CoordFloat>
   {
      let length = CoordFloat::from(points.len()).expect("Unable to convert from usize to value type.");
      let mut points = points.iter();
      let first = points.next().expect("You need at least one coordinate to average!").coordinates.clone();
      let sum = points.fold(first, |acc, x| x.add_to(acc));
      sum.iter().map(|sum| (*sum) / length).collect()
   }
}
