use crate::algebra::{Ring, Semiring};
use crate::arithmetic::prime::factorize;

/// The divisors of `n`.
///
/// # Definition
/// Every positive divisor of `n` exactly once, in increasing order.
///
/// # Complexity
/// - Time: O(√n)
/// - Space: O(d(n)), where d(n) is the number of divisors of `n`
///
/// # Panics
/// Panics if `n = 0`.
pub fn divisors(n: u64) -> Vec<u64> {
    assert!(n > 0, "n must be positive");
    let (mut small, mut large) = (Vec::new(), Vec::new());
    let mut d = 1;
    while d <= n / d {
        if n % d == 0 {
            small.push(d);
            if d != n / d {
                large.push(n / d);
            }
        }
        d += 1;
    }

    small.extend(large.into_iter().rev());
    small
}

/// The lattice of positive divisors of `n`, ordered by divisibility.
///
/// # Definition
/// `D(n) = {d >= 1: d|n}`, with meet `gcd` and join `lcm`.
///
/// # Invariants
/// - `factors` is the prime factorization `n = Π p_i^e_i`, in increasing order of `p_i`.
/// - `divisors` contains `Π p_i^a_i` at index `Σ a_i Π_{j<i} (e_j + 1)`, for `0 <= a_i <= e_i`.
/// - A function on `D(n)` is stored with `f(divisors[i])` at index `i`.
///
/// # Complexity
/// - Space: O(d(n)), where d(n) is the number of divisors of `n`.
pub struct DivisorLattice {
    factors: Box<[(u64, u32)]>,
    divisors: Box<[u64]>,
}

impl DivisorLattice {
    /// The divisor lattice of `n`.
    ///
    /// # Complexity
    /// - Time: O(√n + d(n))
    /// - Space: O(d(n))
    ///
    /// # Panics
    /// Panics if `n = 0`.
    pub fn new(n: u64) -> Self {
        assert!(n > 0, "n must be positive");
        let factors = factorize(n);
        let mut divisors = vec![1u64];
        for &(p, e) in &factors {
            let len = divisors.len();
            let mut power = 1;
            for _ in 0..e {
                power *= p;
                for i in 0..len {
                    divisors.push(divisors[i] * power);
                }
            }
        }
        Self {
            factors: factors.into(),
            divisors: divisors.into(),
        }
    }

    /// The divisors in mixed-radix order.
    ///
    /// # Complexity
    /// - Time: O(1)
    /// - Space: O(1)
    pub fn divisors(&self) -> &[u64] {
        &self.divisors
    }

    /// The index of `d`, or `None` if `d` does not divide `n`.
    ///
    /// # Complexity
    /// - Time: O(log n)
    /// - Space: O(1)
    pub fn index(&self, mut d: u64) -> Option<usize> {
        if d == 0 {
            return None;
        }
        let mut index = 0;
        let mut stride = 1;
        for &(p, e) in &self.factors {
            let mut a = 0;
            while a < e && d % p == 0 {
                d /= p;
                a += 1;
            }
            if d % p == 0 {
                return None;
            }
            index += a as usize * stride;
            stride *= e as usize + 1;
        }
        (d == 1).then_some(index)
    }

    /// The multiple zeta transform of `f`, in place.
    ///
    /// # Definition
    /// `(Zf)(d) = Σ_{d|x|n} f(x)`.
    ///
    /// # Complexity
    /// - Time: O(d(n)ω(n))
    /// - Space: O(1)
    ///
    /// # Panics
    /// Panics if `f.len() != d(n)`.
    pub fn multiple_zeta<R: Semiring>(&self, ring: &R, f: &mut [R::Value]) {
        assert_eq!(
            f.len(),
            self.divisors.len(),
            "length mismatch: len={}, number of divisors={}",
            f.len(),
            self.divisors.len(),
        );
        let mut stride = 1;
        for &(_, e) in &self.factors {
            let e = e as usize;
            let width = stride * (e + 1);
            for block in f.chunks_exact_mut(width) {
                for a in (0..e).rev() {
                    let (lo, hi) = block.split_at_mut((a + 1) * stride);
                    for (x, y) in lo[a * stride..].iter_mut().zip(&hi[..stride]) {
                        *x = ring.add(x, y);
                    }
                }
            }
            stride = width;
        }
    }

    /// The multiple Mobius transform of `f`, in place, inverting [`Self::multiple_zeta`].
    ///
    /// # Complexity
    /// - Time: O(d(n)ω(n))
    /// - Space: O(1)
    ///
    /// # Panics
    /// Panics if `f.len() != d(n)`.
    pub fn multiple_mobius<R: Ring>(&self, ring: &R, f: &mut [R::Value]) {
        assert_eq!(
            f.len(),
            self.divisors.len(),
            "length mismatch: len={}, number of divisors={}",
            f.len(),
            self.divisors.len(),
        );
        let mut stride = 1;
        for &(_, e) in &self.factors {
            let e = e as usize;
            let width = stride * (e + 1);
            for block in f.chunks_exact_mut(width) {
                for a in 0..e {
                    let (lo, hi) = block.split_at_mut((a + 1) * stride);
                    for (x, y) in lo[a * stride..].iter_mut().zip(&hi[..stride]) {
                        *x = ring.add(x, &ring.neg(y));
                    }
                }
            }
            stride = width;
        }
    }

    /// The divisor zeta transform of `f`, in place.
    ///
    /// # Definition
    /// `(Zf)(d) = Σ_{x|d} f(x)`.
    ///
    /// # Complexity
    /// - Time: O(d(n)ω(n))
    /// - Space: O(1)
    ///
    /// # Panics
    /// Panics if `f.len() != d(n)`.
    pub fn divisor_zeta<R: Semiring>(&self, ring: &R, f: &mut [R::Value]) {
        assert_eq!(
            f.len(),
            self.divisors.len(),
            "length mismatch: len={}, number of divisors={}",
            f.len(),
            self.divisors.len(),
        );
        let mut stride = 1;
        for &(_, e) in &self.factors {
            let e = e as usize;
            let width = stride * (e + 1);
            for block in f.chunks_exact_mut(width) {
                for a in 1..=e {
                    let (lo, hi) = block.split_at_mut(a * stride);
                    for (x, y) in hi[..stride].iter_mut().zip(&lo[(a - 1) * stride..]) {
                        *x = ring.add(x, y);
                    }
                }
            }
            stride = width;
        }
    }

    /// The divisor Mobius transform of `f`, in place, inverting [`Self::divisor_zeta`].
    ///
    /// # Complexity
    /// - Time: O(d(n)ω(n))
    /// - Space: O(1)
    ///
    /// # Panics
    /// Panics if `f.len() != d(n)`.
    pub fn divisor_mobius<R: Ring>(&self, ring: &R, f: &mut [R::Value]) {
        assert_eq!(
            f.len(),
            self.divisors.len(),
            "length mismatch: len={}, number of divisors={}",
            f.len(),
            self.divisors.len(),
        );
        let mut stride = 1;
        for &(_, e) in &self.factors {
            let e = e as usize;
            let width = stride * (e + 1);
            for block in f.chunks_exact_mut(width) {
                for a in (1..=e).rev() {
                    let (lo, hi) = block.split_at_mut(a * stride);
                    for (x, y) in hi[..stride].iter_mut().zip(&lo[(a - 1) * stride..]) {
                        *x = ring.add(x, &ring.neg(y));
                    }
                }
            }
            stride = width;
        }
    }
}
