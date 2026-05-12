use crate::point::*;
use crate::search_space::*;
use std::hash::{Hash, Hasher};
use std::rc::Rc;
use num_traits::{Float, float::FloatCore};

//-----------------------------------------------------------------------------
// LINEAR ALGEBRA

/// Solves the linear system Ax = b in place using Gaussian elimination with partial pivoting.
/// Returns None if the matrix is singular.
#[allow(clippy::needless_range_loop)]
fn solve_linear_system<T: Float>(matrix: &mut [Vec<T>], rhs: &mut [T]) -> Option<Box<[T]>>
{
    let n = rhs.len();
    let tolerance = T::epsilon() * T::from(100.0).unwrap();

    // Forward elimination with partial pivoting
    for col in 0..n
    {
        // Find pivot row
        let mut max_row = col;
        let mut max_val = matrix[col][col].abs();
        for row in (col + 1)..n
        {
            let val = matrix[row][col].abs();
            if val > max_val
            {
                max_val = val;
                max_row = row;
            }
        }

        if max_val < tolerance
        {
            return None; // Singular matrix
        }

        // Swap rows
        if max_row != col
        {
            matrix.swap(col, max_row);
            rhs.swap(col, max_row);
        }

        // Eliminate below
        for row in (col + 1)..n
        {
            let factor = matrix[row][col] / matrix[col][col];
            for j in (col + 1)..n
            {
                matrix[row][j] = matrix[row][j] - factor * matrix[col][j];
            }
            rhs[row] = rhs[row] - factor * rhs[col];
        }
    }

    // Back substitution
    let mut solution = vec![T::zero(); n];
    for i in (0..n).rev()
    {
        let mut sum = rhs[i];
        for j in (i + 1)..n
        {
            sum = sum - matrix[i][j] * solution[j];
        }
        solution[i] = sum / matrix[i][i];
    }

    Some(solution.into_boxed_slice())
}

//-----------------------------------------------------------------------------
// SIMPLEX

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

impl<CoordFloat: Float + FloatCore, ValueFloat: Float> Simplex<CoordFloat, ValueFloat>
{
    /// creates a new simplex
    fn new(corners: Vec<Rc<Point<CoordFloat, ValueFloat>>>, ratio: ValueFloat, difference: ValueFloat)
           -> Self
    {
        let center = Point::average_coordinate(&corners);
        Simplex { corners, center, ratio, difference }
    }

    /// builds the initial unit simplex with one point per axis plus an origin at zero
    pub fn initial_simplex<F>(search_space: &mut SearchSpace<F, CoordFloat, ValueFloat>) -> Self
        where F: FnMut(&[CoordFloat]) -> ValueFloat
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
        let total_distance: ValueFloat =
            distances.iter().copied().fold(ValueFloat::zero(), ::std::ops::Add::add);

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

    /// Computes the barycentric coordinates of a point with respect to this simplex.
    /// Returns None if the simplex is degenerate (singular matrix).
    fn barycentric_coordinates(&self, point: &Coordinates<CoordFloat>) -> Option<Box<[CoordFloat]>>
    {
        let d = point.len();
        let v0 = &self.corners[0].coordinates;

        // Build the matrix: row i corresponds to dimension i, column j = corner[j+1] - corner[0]
        let mut matrix: Vec<Vec<CoordFloat>> =
            (0..d).map(|row| (0..d).map(|col| self.corners[col + 1].coordinates[row] - v0[row]).collect())
                  .collect();

        // RHS: point - corner[0]
        let mut rhs: Vec<CoordFloat> = point.iter().zip(v0.iter()).map(|(&p, &v)| p - v).collect();

        // Solve for lambda_1..lambda_d
        let lambdas = solve_linear_system(&mut matrix, &mut rhs)?;

        // lambda_0 = 1 - sum(lambda_1..lambda_d)
        let sum: CoordFloat = lambdas.iter().copied().fold(CoordFloat::zero(), ::std::ops::Add::add);
        let lambda_0 = CoordFloat::one() - sum;

        let mut result = Vec::with_capacity(d + 1);
        result.push(lambda_0);
        result.extend_from_slice(&lambdas);
        Some(result.into_boxed_slice())
    }

    /// Checks whether a point lies inside this simplex.
    pub fn contains_point(&self, point: &Coordinates<CoordFloat>) -> bool
    {
        let tolerance = -<CoordFloat as Float>::epsilon() * CoordFloat::from(100.0).unwrap();
        match self.barycentric_coordinates(point)
        {
            Some(coords) => coords.iter().all(|&c| c >= tolerance),
            None => false
        }
    }

    /// Computes the IDW-interpolated value at a point inside this simplex.
    /// Uses the same inverse-distance weighting as the internal evaluation.
    pub fn interpolate_at(&self, point: &Coordinates<CoordFloat>) -> ValueFloat
    {
        // If the point coincides with a corner, return that corner's value directly
        for corner in &self.corners
        {
            let dist: ValueFloat = Point::distance(&corner.coordinates, point);
            if dist < <ValueFloat as Float>::epsilon()
            {
                return corner.value;
            }
        }

        let inverse_distances: Vec<ValueFloat> =
            self.corners.iter().map(|c| ValueFloat::one() / Point::distance(&c.coordinates, point)).collect();
        let total: ValueFloat =
            inverse_distances.iter().copied().fold(ValueFloat::zero(), ::std::ops::Add::add);
        self.corners
            .iter()
            .zip(inverse_distances.iter())
            .map(|(c, &d)| c.value * d)
            .fold(ValueFloat::zero(), ::std::ops::Add::add)
        / total
    }

    /// returns a score for a simplex
    pub fn evaluate(&self, exploration_depth: ValueFloat) -> ValueFloat
    {
        // computes the inverse of the distance from the center to each corner
        let inverse_distances: Vec<ValueFloat> =
            self.corners
                .iter()
                .map(|c| ValueFloat::one() / Point::distance(&c.coordinates, &self.center))
                .collect();
        let total_inverse_distance: ValueFloat =
            inverse_distances.iter().copied().fold(ValueFloat::zero(), ::std::ops::Add::add);

        // computes the value of the center, interpolated from the corners
        let interpolated_value = self.corners
                                     .iter()
                                     .zip(inverse_distances.iter())
                                     .map(|(c, &d)| c.value * d)
                                     .fold(ValueFloat::zero(), ::std::ops::Add::add)
                                 / total_inverse_distance;

        // computes the number of split needed to reach the given ratio if we start from a regular simplex
        let dim = ValueFloat::from(self.center.len()).unwrap();
        let split_number = self.ratio.log(dim + ValueFloat::one()).abs();

        // ensures that the difference (which is positive by construction) is non zero
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
