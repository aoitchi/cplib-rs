use crate::geometry::vector::Vector;

/// A point `p = (p_x, p_y)` of `Z^2`.
///
/// # Definition
/// For points `p`, `q` and a [`Vector`] `v`, `p - q` is the vector from `q` to `p`, and `p + v` and
/// `p - v` are the points translated by `v` and `-v`. `p - v` arethe points translated by `v` and
/// `-v`. It is displayed as `p_x p_y`.
///
/// # Contract
/// Every result of `p - q`, `p + v` and `p - v` has coordinates that fit in `i64`.
///
/// # Complexity
/// - Space: O(1)
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Point {
    pub x: i64,
    pub y: i64,
}

impl Point {
    /// The origin `(0, 0)`.
    pub const ORIGIN: Self = Self { x: 0, y: 0 };

    /// The point `(x, y)`.
    ///
    /// # Complexity
    /// - Time: O(1)
    /// - Space: O(1)
    pub const fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }
}

/// The orientation of three points.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Orientation {
    /// Turning right.
    Clockwise,
    /// Lying on a line.
    Collinear,
    /// Turning left.
    Counterclockwise,
}

/// The orientation of `p`, `q`, `r`.
///
/// # Definition
/// The sign of `det(q - p, r - p)`:
/// - [`Orientation::Counterclockwise`] if positive.
/// - [`Orientation::Collinear`] if zero.
/// - [`Orientation::Clockwise`] if negative.
///
/// # Contract
/// `q - p` and `r - p` have coordinates that fit in `i64`.
///
/// # Complexity
/// - Time: O(1)
/// - Space: O(1)
pub fn orientation(p: Point, q: Point, r: Point) -> Orientation {
    match (q - p).det(r - p).signum() {
        1 => Orientation::Counterclockwise,
        0 => Orientation::Collinear,
        _ => Orientation::Clockwise,
    }
}

/// The squared distance `|q - p|^2` between `p` and `q`.
///
/// # Contract
/// `q - p` has coordinates that fit in `i64`, and `q - p != (i64::MIN, i64::MIN)`.
///
/// # Complexity
/// - Time: O(1)
/// - Space: O(1)
pub fn squared_distance(p: Point, q: Point) -> i128 {
    (q - p).squared_length()
}

impl From<(i64, i64)> for Point {
    fn from(value: (i64, i64)) -> Self {
        Self::new(value.0, value.1)
    }
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.x, self.y)
    }
}

impl std::ops::Sub for Point {
    type Output = Vector;
    #[inline]
    fn sub(self, rhs: Self) -> Vector {
        Vector::new(self.x - rhs.x, self.y - rhs.y)
    }
}
impl std::ops::Sub<&Self> for Point {
    type Output = Vector;
    #[inline]
    fn sub(self, rhs: &Self) -> Vector {
        std::ops::Sub::sub(self, *rhs)
    }
}
impl std::ops::Sub<Point> for &Point {
    type Output = Vector;
    #[inline]
    fn sub(self, rhs: Point) -> Vector {
        std::ops::Sub::sub(*self, rhs)
    }
}
impl std::ops::Sub<Self> for &Point {
    type Output = Vector;
    #[inline]
    fn sub(self, rhs: Self) -> Vector {
        std::ops::Sub::sub(*self, *rhs)
    }
}

impl std::ops::Add<Vector> for Point {
    type Output = Self;
    #[inline]
    fn add(mut self, rhs: Vector) -> Self {
        self.x += rhs.x;
        self.y += rhs.y;
        self
    }
}
impl std::ops::Sub<Vector> for Point {
    type Output = Self;
    #[inline]
    fn sub(mut self, rhs: Vector) -> Self {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self
    }
}
macro_rules! forward_ref_binop {
    ($($trait:ident, $method:ident);* $(;)?) => {
        $(
            impl std::ops::$trait<&Vector> for Point {
                type Output = Self;
                #[inline]
                fn $method(self, rhs: &Vector) -> Self {
                    std::ops::$trait::$method(self, *rhs)
                }
            }

            impl std::ops::$trait<Vector> for &Point {
                type Output = Point;
                #[inline]
                fn $method(self, rhs: Vector) -> Point {
                    std::ops::$trait::$method(*self, rhs)
                }
            }

            impl std::ops::$trait<&Vector> for &Point {
                type Output = Point;
                #[inline]
                fn $method(self, rhs: &Vector) -> Point {
                    std::ops::$trait::$method(*self, *rhs)
                }
            }
        )*
    };
}
forward_ref_binop! {
    Add, add;
    Sub, sub;
}

impl std::ops::AddAssign<Vector> for Point {
    #[inline]
    fn add_assign(&mut self, rhs: Vector) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}
impl std::ops::SubAssign<Vector> for Point {
    #[inline]
    fn sub_assign(&mut self, rhs: Vector) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}
macro_rules! forward_ref_op_assign {
    ($($trait:ident, $method:ident);* $(;)?) => {
        $(
            impl std::ops::$trait<&Vector> for Point {
                #[inline]
                fn $method(&mut self, rhs: &Vector) {
                    std::ops::$trait::$method(self, *rhs);
                }
            }
        )*
    };
}
forward_ref_op_assign! {
    AddAssign, add_assign;
    SubAssign, sub_assign;
}
