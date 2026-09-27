use crate::arithmetic::gcd::gcd;
use crate::geometry::point::Point;

/// A point of the plane with rational coordinates.
///
/// # Definition
/// The point `(x/w, y/w)` given by homogeneous coordinates `(x : y : w)` with `w != 0`, where
/// `(x : y : w)` and `(λx : λy : λw)` are the same point for every nonzero `λ`. Its coordinates are
/// the unique integers `(x, y, w)` with `gcd(x, y, w) = 1` and `w > 0`.
///
/// # Complexity
/// - Space: O(1)
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct RationalPoint {
    x: i128,
    y: i128,
    w: i128,
}

impl RationalPoint {
    /// The point `(x/w, y/w)`.
    ///
    /// # Complexity
    /// - Time: O(log max(|x|, |y|, |w|))
    /// - Space: O(1)
    ///
    /// # Panics
    /// Panics if `w = 0`.
    pub fn new(x: i128, y: i128, w: i128) -> Self {
        assert!(w != 0, "w must be nonzero");
        let mut g = gcd(gcd(x.abs(), y.abs()), w.abs());
        if w < 0 {
            g *= -1;
        }
        Self {
            x: x / g,
            y: y / g,
            w: w / g,
        }
    }

    /// The midpoint of `p` and `q`.
    ///
    /// # Complexity
    /// - Time: O(1)
    /// - Space: O(1)
    pub fn midpoint(p: Point, q: Point) -> Self {
        let x = p.x as i128 + q.x as i128;
        let y = p.y as i128 + q.y as i128;
        if x % 2 == 0 && y % 2 == 0 {
            Self {
                x: x / 2,
                y: y / 2,
                w: 1,
            }
        } else {
            Self { x, y, w: 2 }
        }
    }

    /// The homogeneous coordinates `(x, y, w)`.
    ///
    /// # Definition
    /// The unique ones with `gcd(x, y, w) = 1` and `w > 0`.
    ///
    /// # Complexity
    /// - Time: O(1)
    /// - Space: O(1)
    pub fn homogeneous(self) -> (i128, i128, i128) {
        (self.x, self.y, self.w)
    }
}

impl From<Point> for RationalPoint {
    fn from(p: Point) -> Self {
        Self {
            x: p.x as i128,
            y: p.y as i128,
            w: 1,
        }
    }
}
