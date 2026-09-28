use crate::algebra::{
    Commutative, Group, Idempotent, Monoid, Ring, Semigroup, Semiring, SkewField,
};

macro_rules! forward {
    ($name:ident) => {
        impl<M: Semigroup> Semigroup for $name<M> {
            type Value = M::Value;
            fn op(&self, a: &M::Value, b: &M::Value) -> M::Value {
                self.0.op(a, b)
            }
        }
        impl<M: Monoid> Monoid for $name<M> {
            fn id(&self) -> M::Value {
                self.0.id()
            }
        }
        impl<G: Group> Group for $name<G> {
            fn inv(&self, a: &G::Value) -> G::Value {
                self.0.inv(a)
            }
        }
        impl<R: Semiring> Semiring for $name<R> {
            type Value = R::Value;
            fn zero(&self) -> R::Value {
                self.0.zero()
            }
            fn one(&self) -> R::Value {
                self.0.one()
            }
            fn add(&self, a: &R::Value, b: &R::Value) -> R::Value {
                self.0.add(a, b)
            }
            fn mul(&self, a: &R::Value, b: &R::Value) -> R::Value {
                self.0.mul(a, b)
            }
        }
        impl<R: Ring> Ring for $name<R> {
            fn neg(&self, a: &R::Value) -> R::Value {
                self.0.neg(a)
            }
        }
        impl<F: SkewField> SkewField for $name<F> {
            fn inv(&self, a: &F::Value) -> F::Value {
                self.0.inv(a)
            }
        }
    };
}

/// A structure asserted to be commutative.
///
/// # Definition
/// `self.0` with the same operations.
///
/// # Contract
/// `self.0` satisfies the contract of [`Commutative`].
#[derive(Clone, Copy, Default)]
pub struct AssertCommutative<M>(pub M);
forward!(AssertCommutative);
impl<M> Commutative for AssertCommutative<M> {}
impl<M: Idempotent> Idempotent for AssertCommutative<M> {}

/// A structure asserted to be idempotent.
///
/// # Definition
/// `self.0` with the same operations.
///
/// # Contract
/// `self.0` satisfies the contract of [`Idempotent`].
#[derive(Clone, Copy, Default)]
pub struct AssertIdempotent<M>(pub M);
forward!(AssertIdempotent);
impl<M> Idempotent for AssertIdempotent<M> {}
impl<M: Commutative> Commutative for AssertIdempotent<M> {}
