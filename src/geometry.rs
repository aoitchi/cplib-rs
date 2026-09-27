//! Geometry of the plane with integer coordinates.
//!
//! `Z^2` is the plane of the points `(x, y)` with integer coordinates, with the `x`-axis to the
//! right and the `y`-axis upward. Coordinates are `i64`, and quantities of degree two in them, such
//! an inner products and determinants, are `i128`.
//!
//! For a point or a vector `v`, `v_x` and `v_y` denote its fields `x` and `y`. In the documentation
//! of a method, `self` is written `v` for a vector and `p` for a point.

pub mod point;
pub mod vector;
