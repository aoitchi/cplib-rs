/// A vector `v = (v_x, v_y)` of `Z^2`.
///
/// # Definition
/// The vector with components `(x, y)`, `+`, `-`, unary `-` and `*` by an `i64` are the operations
/// of the `Z`-module `Z^2`. It is displayed as `v_x v_y`.
///
/// # Contract
/// Every result of `+`, `-`, unary `-` and `*` has components that fit in `i64`.
///
/// # Complexity
/// - Space: O(1)
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct Vector {
    pub x: i64,
    pub y: i64,
}

impl Vector {
    /// The zero vector `(0, 0)`.
    pub const ZERO: Self = Self { x: 0, y: 0 };

    /// The vector `(x, y)`.
    ///
    /// # Complexity
    /// - Time: O(1)
    /// - Space: O(1)
    pub const fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }

    /// The inner product `<v, w> = v_x * w_x + v_y * w_y`.
    ///
    /// # Contract
    /// `v` and `w` are not both `(i64::MIN, i64::MIN)`.
    ///
    /// # Complexity
    /// - Time: O(1)
    /// - Space: O(1)
    pub fn dot(self, w: Self) -> i128 {
        self.x as i128 * w.x as i128 + self.y as i128 * w.y as i128
    }

    /// The determinant `det(v, w) = v_x w_y - v_y w_x` of the matrix with columns `v` and `w`.
    ///
    /// # Complexity
    /// - Time: O(1)
    /// - Space: O(1)
    pub fn det(self, w: Self) -> i128 {
        self.x as i128 * w.y as i128 - self.y as i128 * w.x as i128
    }

    /// The squared length `|v|^2 = v_x^2 + v_y^2`.
    ///
    /// # Contract
    /// `v != (i64::MIN, i64::MIN)`.
    ///
    /// # Complexity
    /// - Time: O(1)
    /// - Space: O(1)
    pub fn squared_length(self) -> i128 {
        self.dot(self)
    }

    /// The rotation `(-v_y, v_x)` of `v` by a quarter turn counterclockwise.
    ///
    /// # Contract
    /// `v_y != i64::MIN`.
    ///
    /// # Complexity
    /// - Time: O(1)
    /// - Space: O(1)
    pub fn rot_quarter(self) -> Self {
        Self::new(-self.y, self.x)
    }

    /// Whether `v` and `w` are parallel, that is `det(v, w) = 0`.
    ///
    /// # Complexity
    /// - Time: O(1)
    /// - Space: O(1)
    pub fn is_parallel(self, w: Self) -> bool {
        self.det(w) == 0
    }

    /// Whether `v` and `w` are orthogonal, that is `<v, w> = 0`.
    ///
    /// # Complexity
    /// - Time: O(1)
    /// - Space: O(1)
    pub fn is_orthogonal(self, w: Self) -> bool {
        self.x as i128 * w.x as i128 == -(self.y as i128 * w.y as i128)
    }
}

impl From<(i64, i64)> for Vector {
    fn from(value: (i64, i64)) -> Self {
        Self::new(value.0, value.1)
    }
}

impl std::fmt::Display for Vector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.x, self.y)
    }
}

impl std::ops::Neg for Vector {
    type Output = Self;
    fn neg(mut self) -> Self {
        self.x = -self.x;
        self.y = -self.y;
        self
    }
}
impl std::ops::Neg for &Vector {
    type Output = Vector;
    fn neg(self) -> Vector {
        -*self
    }
}

impl std::ops::Add for Vector {
    type Output = Self;
    #[inline]
    fn add(mut self, rhs: Self) -> Self {
        self.x += rhs.x;
        self.y += rhs.y;
        self
    }
}
impl std::ops::Sub for Vector {
    type Output = Self;
    #[inline]
    fn sub(mut self, rhs: Self) -> Self {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self
    }
}
macro_rules! forward_ref_binop {
    ($($trait:ident, $method:ident);* $(;)?) => {
        $(
            impl std::ops::$trait<&Self> for Vector {
                type Output = Self;
                #[inline]
                fn $method(self, rhs: &Self) -> Self {
                    std::ops::$trait::$method(self, *rhs)
                }
            }

            impl std::ops::$trait<Vector> for &Vector {
                type Output = Vector;
                #[inline]
                fn $method(self, rhs: Vector) -> Vector {
                    std::ops::$trait::$method(*self, rhs)
                }
            }

            impl std::ops::$trait for &Vector {
                type Output = Vector;
                #[inline]
                fn $method(self, rhs: Self) -> Vector {
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
impl std::ops::Mul<i64> for Vector {
    type Output = Self;
    #[inline]
    fn mul(mut self, rhs: i64) -> Self {
        self.x *= rhs;
        self.y *= rhs;
        self
    }
}
impl std::ops::Mul<&i64> for Vector {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: &i64) -> Self {
        std::ops::Mul::mul(self, *rhs)
    }
}
impl std::ops::Mul<i64> for &Vector {
    type Output = Vector;
    #[inline]
    fn mul(self, rhs: i64) -> Vector {
        std::ops::Mul::mul(*self, rhs)
    }
}
impl std::ops::Mul<&i64> for &Vector {
    type Output = Vector;
    #[inline]
    fn mul(self, rhs: &i64) -> Vector {
        std::ops::Mul::mul(*self, *rhs)
    }
}

impl std::ops::AddAssign for Vector {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}
impl std::ops::SubAssign for Vector {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}
macro_rules! forward_ref_op_assign {
    ($($trait:ident, $method:ident);* $(;)?) => {
        $(
            impl std::ops::$trait<&Self> for Vector {
                #[inline]
                fn $method(&mut self, rhs: &Self) {
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
impl std::ops::MulAssign<i64> for Vector {
    #[inline]
    fn mul_assign(&mut self, rhs: i64) {
        self.x *= rhs;
        self.y *= rhs;
    }
}
impl std::ops::MulAssign<&i64> for Vector {
    #[inline]
    fn mul_assign(&mut self, rhs: &i64) {
        std::ops::MulAssign::mul_assign(self, *rhs);
    }
}
