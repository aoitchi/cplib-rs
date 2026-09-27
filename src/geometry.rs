//! Geometry of the plane with integer coordinates.
//!
//! `Z^2` is the plane of the points `(x, y)` with integer coordinates, with the `x`-axis to the
//! right and the `y`-axis upward. Coordinates are `i64`, and quantities of degree two in them, such
//! an inner products and determinants, are `i128`.

pub mod point;
pub mod vector;
