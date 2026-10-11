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
/// Panics if `n == 0`.
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
