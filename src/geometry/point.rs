use crate::arithmetic::gcd::gcd;
use crate::geometry::vector::Vector;

/// A point `p = (p_x, p_y)` of `Z^2`.
///
/// # Definition
/// For points `p`, `q` and a [`Vector`] `v`, `p - q` is the vector from `q` to `p`, and `p + v` and
/// `p - v` are the points translated by `v` and `-v`. It is displayed as `p_x p_y`.
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
/// - [`Orientation::Counterclockwise`] if `p`, `q`, `r` make a left turn, that is, `r` lies to the
///   left of the line from `p` to `q`.
/// - [`Orientation::Clockwise`] if they make a right turn.
/// - [`Orientation::Collinear`] if they lie on a line.
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
/// # Complexity
/// - Time: O(1)
/// - Space: O(1)
pub fn squared_distance(p: Point, q: Point) -> i128 {
    (q - p).squared_length()
}

/// The kind of an angle in `[0, π]`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Angle {
    /// An angle in `[0, π/2)`.
    Acute,
    /// The angle `π/2`.
    Right,
    /// An angle in `(π/2, π]`.
    Obtuse,
}

/// The kind of the angle at `q` between the rays from `q` through `p` and through `r`.
///
/// # Complexity
/// - Time: O(1)
/// - Space: O(1)
///
/// # Panics
/// Panics if `p = q` or `r = q`.
pub fn angle(p: Point, q: Point, r: Point) -> Angle {
    assert!(p != q && r != q, "p and r must differ from q: q=({q})");
    match (p - q).dot(r - q).signum() {
        1 => Angle::Acute,
        0 => Angle::Right,
        _ => Angle::Obtuse,
    }
}

/// The midpoint of `p` and `q`.
///
/// # Complexity
/// - Time: O(1)
/// - Space: O(1)
pub fn midpoint(p: Point, q: Point) -> RationalPoint {
    let x = p.x as i128 + q.x as i128;
    let y = p.y as i128 + q.y as i128;
    if x % 2 == 0 && y % 2 == 0 {
        RationalPoint {
            x: x / 2,
            y: y / 2,
            w: 1,
        }
    } else {
        RationalPoint { x, y, w: 2 }
    }
}

/// The centroid `(p_0 + ... + p_{n-1}) / n` of the points `ps = [p_0, ..., p_{n-1}]`.
///
/// # Complexity
/// - Time: O(n)
/// - Space: O(1)
///
/// # Panics
/// Panics if `ps` is empty.
pub fn centroid(ps: &[Point]) -> RationalPoint {
    assert!(!ps.is_empty(), "ps must be nonempty");
    let n = ps.len() as i128;
    let x: i128 = ps.iter().map(|p| p.x as i128).sum();
    let y: i128 = ps.iter().map(|p| p.y as i128).sum();
    let g = gcd(gcd(x.rem_euclid(n), y.rem_euclid(n)), n);
    RationalPoint {
        x: x / g,
        y: y / g,
        w: n / g,
    }
}

/// The centroid of the region of the polygon with vertices `ps = [p_0, ..., p_{n-1}]` in order.
///
/// # Contract
/// The polygon does not intersect itself.
///
/// # Complexity
/// - Time: O(n + log(nC)), where `C` is the largest absolute value of a coordinate
/// - Space: O(1)
///
/// # Panics
/// Panics if the area of the polygon is `0`.
pub fn polygon_centroid(ps: &[Point]) -> RationalPoint {
    let n = ps.len();
    let (mut x, mut y, mut a) = (0, 0, 0);
    for i in 0..n {
        let (p, q) = (ps[i], ps[(i + 1) % n]);
        let c = (p - Point::ORIGIN).det(q - Point::ORIGIN);
        x += c * (p.x as i128 + q.x as i128);
        y += c * (p.y as i128 + q.y as i128);
        a += c;
    }
    assert!(a != 0, "the area of the polygon must be nonzero");
    RationalPoint::new(x, y, 3 * a)
}

/// The position of a point relative to a circle.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum CircleSide {
    /// Inside the circle.
    Inside,
    /// On the circle.
    On,
    /// Outside the circle.
    Outside,
}

/// The position of `s` relative to the circle through `p`, `q`, `r`.
///
/// # Complexity
/// - Time: O(1)
/// - Space: O(1)
///
/// # Panics
/// Panics if `p`, `q`, `r` lie on a line.
pub fn circumcircle_side(p: Point, q: Point, r: Point, s: Point) -> CircleSide {
    let o = (q - p).det(r - p).signum();
    assert!(o != 0, "p, q, r must not lie on a line");
    let (u, v, w) = (p - s, q - s, r - s);
    let d = u.squared_length() * v.det(w) - v.squared_length() * u.det(w)
        + w.squared_length() * u.det(v);
    match d.signum() * o {
        1 => CircleSide::Inside,
        0 => CircleSide::On,
        _ => CircleSide::Outside,
    }
}

/// The squared radius `n / d` of the circle through `p`, `q`, `r`, as the pair `(n, d)` with
/// `n = |q - p|^2 |p - r|^2 |r - q|^2`.
///
/// # Complexity
/// - Time: O(1)
/// - Space: O(1)
///
/// # Panics
/// Panics if `p`, `q`, `r` lie on a line.
pub fn squared_circumradius(p: Point, q: Point, r: Point) -> (i128, i128) {
    let det = (q - p).det(r - p);
    assert!(det != 0, "p, q, r must not lie on a line");
    let n = squared_distance(p, q) * squared_distance(q, r) * squared_distance(r, p);
    (n, 4 * det * det)
}

/// The circumcenter of `p`, `q`, `r`.
///
/// # Definition
/// The point at equal distance from `p`, `q` and `r`.
///
/// # Complexity
/// - Time: O(log max(|p_x|, |p_y|, |q_x|, |q_y|, |r_x|, |r_y|))
/// - Space: O(1)
///
/// # Panics
/// Panics if `p`, `q`, `r` lie on a line.
pub fn circumcenter(p: Point, q: Point, r: Point) -> RationalPoint {
    let (b, c) = (q - p, r - p);
    let d = 2 * b.det(c);
    assert!(d != 0, "p, q, r must not lie on a line");
    let (bb, cc) = (b.squared_length(), c.squared_length());
    RationalPoint::new(
        p.x as i128 * d + bb * c.y as i128 - cc * b.y as i128,
        p.y as i128 * d + cc * b.x as i128 - bb * c.x as i128,
        d,
    )
}

/// The orthocenter of `p`, `q`, `r`.
///
/// # Definition
/// The common point of the three altitudes of the triangle `p, q, r`.
///
/// # Complexity
/// - Time: O(log max(|p_x|, |p_y|, |q_x|, |q_y|, |r_x|, |r_y|))
/// - Space: O(1)
///
/// # Panics
/// Panics if `p`, `q`, `r` lie on a line.
pub fn orthocenter(p: Point, q: Point, r: Point) -> RationalPoint {
    let (x, y, w) = circumcenter(p, q, r).homogeneous();
    let sx = p.x as i128 + q.x as i128 + r.x as i128;
    let sy = p.y as i128 + q.y as i128 + r.y as i128;
    RationalPoint::new(sx * w - 2 * x, sy * w - 2 * y, w)
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
