use crate::arithmetic::gcd::gcd;
use crate::geometry::point::{Point, RationalPoint};
use crate::geometry::vector::Vector;

/// A line `l` of the plane with integer coefficients, without orientation.
///
/// # Definition
/// The set of points `(x, y)` with `a_l x + b_l y + c_l = 0`, for integers `a_l, b_l, c_l` with
/// `(a_l, b_l) != (0, 0)`. Two such equations give the same line exactly when their coefficients
/// are proportional, and the coefficients of `l` are the unique ones with `gcd(a_l, b_l, c_l) = 1`
/// and the first nonzero of `a_l, b_l` positive.
///
/// # Complexity
/// - Space: O(1)
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Line {
    a: i128,
    b: i128,
    c: i128,
}

/// The intersection of two lines.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Intersection {
    /// Parallel and distinct lines, which do not meet.
    Disjoint,
    /// Lines meeting at one point.
    Point(RationalPoint),
    /// The same line.
    Coincident,
}

impl Line {
    /// The line `ax + by + c = 0`.
    ///
    /// # Complexity
    /// - Time: O(log max(|a|, |b|, |c|))
    /// - Space: O(1)
    ///
    /// # Panics
    /// Panics if `a = b = 0`.
    pub fn new(a: i128, b: i128, c: i128) -> Self {
        assert!(a != 0 || b != 0, "a and b must not both be zero");
        let mut g = gcd(gcd(a.abs(), b.abs()), c.abs());
        if a < 0 || (a == 0 && b < 0) {
            g *= -1;
        }
        Self {
            a: a / g,
            b: b / g,
            c: c / g,
        }
    }

    /// The line through `p` and `q`.
    ///
    /// # Complexity
    /// - Time: O(log max(|p_x|, |p_y|, |q_x|, |q_y|))
    /// - Space: O(1)
    ///
    /// # Panics
    /// Panics if `p = q`.
    pub fn through(p: Point, q: Point) -> Self {
        assert!(p != q, "p and q must be distinct: p=({p})");
        let (px, py, qx, qy) = (p.x as i128, p.y as i128, q.x as i128, q.y as i128);
        Self::new(py - qy, qx - px, px * qy - py * qx)
    }

    /// The line through `p` in the direction of `v`.
    ///
    /// # Complexity
    /// - Time: O(log max(|p_x|, |p_y|, |v_x|, |v_y|))
    /// - Space: O(1)
    ///
    /// # Panics
    /// Panics if `v = 0`.
    pub fn along(p: Point, v: Vector) -> Self {
        assert!(v != Vector::ZERO, "v must be nonzero");
        Self::new(
            v.y as i128,
            -v.x as i128,
            v.x as i128 * p.y as i128 - v.y as i128 * p.x as i128,
        )
    }

    /// The perpendicular bisector of `p` and `q`.
    ///
    /// # Definition
    /// The set of points at equal distance from `p` and `q`.
    ///
    /// # Complexity
    /// - Time: O(log max(|p_x|, |p_y|, |q_x|, |q_y|))
    /// - Space: O(1)
    ///
    /// # Panics
    /// Panics if `p = q`.
    pub fn bisector(p: Point, q: Point) -> Self {
        assert!(p != q, "p and q must be distinct: p=({p})");
        let (px, py, qx, qy) = (p.x as i128, p.y as i128, q.x as i128, q.y as i128);
        let c = (qx * qx - px * px) + (qy * qy - py * py);
        Self::new(2 * (px - qx), 2 * (py - qy), c)
    }

    /// The line through `p` perpendicular to `l`.
    ///
    /// # Complexity
    /// - Time: O(log max(|a_l|, |b_l|, |p_x|, |p_y|))
    /// - Space: O(1)
    pub fn perpendicular(self, p: Point) -> Self {
        Self::new(self.b, -self.a, self.a * p.y as i128 - self.b * p.x as i128)
    }

    /// The line through `p` parallel to `l`.
    ///
    /// # Complexity
    /// - Time: O(log max(|a_l|, |b_l|, |p_x|, |p_y|))
    /// - Space: O(1)
    pub fn parallel(self, p: Point) -> Self {
        Self::new(
            self.a,
            self.b,
            -(self.a * p.x as i128 + self.b * p.y as i128),
        )
    }

    /// The direction vector `(b_l, -a_l)` of `l`.
    ///
    /// # Complexity
    /// - Time: O(1)
    /// - Space: O(1)
    pub fn direction(self) -> Vector {
        Vector::new(self.b as i64, -self.a as i64)
    }

    /// The normal vector `(a_l, b_l)` of `l`.
    ///
    /// # Complexity
    /// - Time: O(1)
    /// - Space: O(1)
    pub fn normal(self) -> Vector {
        Vector::new(self.a as i64, self.b as i64)
    }

    /// Whether `l` is parallel to the `x`-axis.
    ///
    /// # Complexity
    /// - Time: O(1)
    /// - Space: O(1)
    pub fn is_horizontal(self) -> bool {
        self.a == 0
    }

    /// Whether `l` is parallel to the `y`-axis.
    ///
    /// # Complexity
    /// - Time: O(1)
    /// - Space: O(1)
    pub fn is_vertical(self) -> bool {
        self.b == 0
    }

    /// The homogeneous coordinates `(a_l, b_l, c_l)`.
    ///
    /// # Definition
    /// The unique ones with `gcd(a_l, b_l, c_l) = 1` and the first nonzero of `a_l, b_l` positive.
    ///
    /// # Complexity
    /// - Time: O(1)
    /// - Space: O(1)
    pub fn homogeneous(self) -> (i128, i128, i128) {
        (self.a, self.b, self.c)
    }

    /// The `y`-coordinate `n / d` of the point of `l` with `x`-coordinate `x`, as the pair `(n, d)`
    /// with `d = |b_l|`.
    ///
    /// # Complexity
    /// - Time: O(1)
    /// - Space: O(1)
    ///
    /// # Panics
    /// Panics if `l` is vertical.
    pub fn y_at_x(self, x: i64) -> (i128, i128) {
        assert!(self.b != 0, "l must not be vertical");
        let n = -(self.a * x as i128 + self.c);
        if self.b > 0 {
            (n, self.b)
        } else {
            (-n, -self.b)
        }
    }

    /// The `x`-coordinate `n / d` of the point of `l` with `y`-coordinate `y`, as the pair `(n, d)`
    /// with `d = |a_l|`.
    ///
    /// # Complexity
    /// - Time: O(1)
    /// - Space: O(1)
    ///
    /// # Panics
    /// Panics if `l` is horizontal.
    pub fn x_at_y(self, y: i64) -> (i128, i128) {
        assert!(self.a != 0, "l must not be horizontal");
        let n = -(self.b * y as i128 + self.c);
        if self.a > 0 {
            (n, self.a)
        } else {
            (-n, -self.a)
        }
    }

    /// Whether `l` contains `p`.
    ///
    /// # Complexity
    /// - Time: O(1)
    /// - Space: O(1)
    pub fn contains(self, p: Point) -> bool {
        self.a * p.x as i128 + self.b * p.y as i128 + self.c == 0
    }

    /// Whether `p` and `q` lie on the same side of `l`, neither of them on `l`.
    ///
    /// # Complexity
    /// - Time: O(1)
    /// - Space: O(1)
    pub fn same_side(self, p: Point, q: Point) -> bool {
        let s = self.a * p.x as i128 + self.b * p.y as i128 + self.c;
        let t = self.a * q.x as i128 + self.b * q.y as i128 + self.c;
        s != 0 && s.signum() == t.signum()
    }

    /// Whether `l` contains `r`.
    ///
    /// # Complexity
    /// - Time: O(1)
    /// - Space: O(1)
    pub fn contains_rational(self, r: RationalPoint) -> bool {
        let (x, y, w) = r.homogeneous();
        self.a * x + self.b * y + self.c * w == 0
    }

    /// Whether `r` and `s` lie on the same side of `l`, neither of them on `l`.
    ///
    /// # Complexity
    /// - Time: O(1)
    /// - Space: O(1)
    pub fn same_side_rational(self, r: RationalPoint, s: RationalPoint) -> bool {
        let ((rx, ry, rw), (sx, sy, sw)) = (r.homogeneous(), s.homogeneous());
        let u = self.a * rx + self.b * ry + self.c * rw;
        let v = self.a * sx + self.b * sy + self.c * sw;
        u != 0 && u.signum() == v.signum()
    }

    /// Whether `l` and `m` are parallel.
    ///
    /// # Complexity
    /// - Time: O(1)
    /// - Space: O(1)
    pub fn is_parallel(self, m: Self) -> bool {
        self.a * m.b == m.a * self.b
    }

    /// Whether `l` and `m` are orthogonal.
    ///
    /// # Complexity
    /// - Time: O(1)
    /// - Space: O(1)
    pub fn is_orthogonal(self, m: Self) -> bool {
        self.a * m.a + self.b * m.b == 0
    }

    /// The intersection of `l` and `m`.
    ///
    /// # Complexity
    /// - Time: O(log max(|a_l|, |b_l|, |c_l|, |a_m|, |b_m|, |c_m|))
    /// - Space: O(1)
    pub fn intersection(self, m: Self) -> Intersection {
        let w = self.a * m.b - m.a * self.b;
        if w != 0 {
            let x = self.b * m.c - m.b * self.c;
            let y = self.c * m.a - m.c * self.a;
            Intersection::Point(RationalPoint::new(x, y, w))
        } else if self == m {
            Intersection::Coincident
        } else {
            Intersection::Disjoint
        }
    }

    /// The foot of the perpendicular from `p` to `l`.
    ///
    /// # Complexity
    /// - Time: O(log max(|a_l|, |b_l|, |c_l|, |p_x|, |p_y|))
    /// - Space: O(1)
    pub fn projection(self, p: Point) -> RationalPoint {
        let (px, py) = (p.x as i128, p.y as i128);
        let w = self.a * self.a + self.b * self.b;
        let v = self.a * px + self.b * py + self.c;
        RationalPoint::new(px * w - self.a * v, py * w - self.b * v, w)
    }

    /// The reflection of `p` in `l`.
    ///
    /// # Definition
    /// The point `p'` such that `l` is the perpendicular bisector of `p` and `p'`, or `p` itself if
    /// `l` contains `p`.
    ///
    /// # Complexity
    /// - Time: O(log max(|a_l|, |b_l|, |c_l|, |p_x|, |p_y|))
    /// - Space: O(1)
    pub fn reflection(self, p: Point) -> RationalPoint {
        let (px, py) = (p.x as i128, p.y as i128);
        let w = self.a * self.a + self.b * self.b;
        let v = self.a * px + self.b * py + self.c;
        RationalPoint::new(px * w - 2 * self.a * v, py * w - 2 * self.b * v, w)
    }

    /// The reflection of `m` in `l`.
    ///
    /// # Definition
    /// The line of the reflections in `l` of the points of `m`.
    ///
    /// # Complexity
    /// - Time: O(log max(|a_l|, |b_l|, |c_l|, |a_m|, |b_m|, |c_m|))
    /// - Space: O(1)
    pub fn reflection_of_line(self, m: Self) -> Self {
        let w = self.a * self.a + self.b * self.b;
        let k = 2 * (self.a * m.a + self.b * m.b);
        Self::new(
            w * m.a - k * self.a,
            w * m.b - k * self.b,
            w * m.c - k * self.c,
        )
    }

    /// The squared distance `n / d` from `p` to `l`, as the pair `(n, d)` with `d = a_l^2 + b_l^2`.
    ///
    /// # Complexity
    /// - Time: O(1)
    /// - Space: O(1)
    pub fn squared_distance(self, p: Point) -> (i128, i128) {
        let v = self.a * p.x as i128 + self.b * p.y as i128 + self.c;
        (v * v, self.a * self.a + self.b * self.b)
    }

    /// The squared distance `n / d` between the parallel lines `l` and `m`, as the pair `(n, d)`
    /// with `d = gcd(a_m, b_m)^2 (a_l^2 + b_l^2)`.
    ///
    /// # Complexity
    /// - Time: O(log max(|a_l|, |b_l|, |a_m|, |b_m|))
    /// - Space: O(1)
    ///
    /// # Panics
    /// Panics if `l` and `m` are not parallel.
    pub fn squared_distance_to_line(self, m: Self) -> (i128, i128) {
        assert!(self.is_parallel(m), "l and m must be parallel");
        let (g, h) = (gcd(self.a.abs(), self.b.abs()), gcd(m.a.abs(), m.b.abs()));
        let v = self.c * h - m.c * g;
        (v * v, h * h * (self.a * self.a + self.b * self.b))
    }
}
