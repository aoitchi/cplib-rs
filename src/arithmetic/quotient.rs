/// Calls `f` once for each block of `x` with the same quotient `floor(n / x)`.
///
/// # Definition
/// For `x` in `[1, n]`, `q(x) = floor(n / x)` is non-increasing in `x`, so `[1, n]` splits into
/// maximal intervals `[l, r)` on which `q` is constant. `f(q, l, r)` is called once for each such
/// interval, in increasing order of `l`, with `q = q(l) = ... = q(r - 1)`. For `n = 0` nothing is
/// called.
///
/// # Complexity
/// - Time: O(√n), and O(√n) calls of `f`
/// - Space: O(1)
///
/// # Panics
/// Panics if `n = u64::MAX`.
pub fn for_each_quotient(n: u64, mut f: impl FnMut(u64, u64, u64)) {
    assert!(n < u64::MAX, "n must be less than u64::MAX");
    let mut l = 1;
    while l <= n {
        let q = n / l;
        let r = n / q + 1;
        f(q, l, r);
        l = r;
    }
}
