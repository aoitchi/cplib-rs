use crate::algebra::canonical::{Additive, Canonical};
use crate::collections::fenwick_tree::FenwickTree;

/// The number of inversions of `a`.
///
/// # Definition
/// For a sequence `a` of length `n`, an inversion of `a` is a pair `(i, j)` with `0 <= i < j < n`
/// and `a[i] > a[j]`. The number of inversions is the minimum number of swaps of two adjacent
/// elements that sort `a` in non-decreasing order.
///
/// # Complexity
/// - Time: O(n log n)
/// - Space: O(n)
pub fn count_inversions<T: Ord>(a: &[T]) -> usize {
    let mut order: Vec<(&T, usize)> = a.iter().zip(0..).collect();
    order.sort_unstable();
    let mut fenwick_tree = FenwickTree::new(Additive(Canonical::<usize>::new()), a.len());
    let mut count = 0;
    for (k, &(_, i)) in order.iter().enumerate() {
        count += k - fenwick_tree.prefix_fold(i);
        fenwick_tree.op_assign(i, &1);
    }
    count
}

/// The numbers of inversions of the rotations of `a`.
///
/// # Definition
/// For a sequence `a` of length `n`, `c[k]` is the number of inversions, as defined in
/// [`count_inversions`], of `(a[k], ..., a[n - 1], a[0], ..., a[k - 1])` for `k` in `[0, n)`.
///
/// # Complexity
/// - Time: O(n log n)
/// - Space: O(n)
pub fn rotation_inversions<T: Ord>(a: &[T]) -> Vec<usize> {
    let n = a.len();
    let mut order: Vec<(&T, usize)> = a.iter().zip(0..).collect();
    order.sort_unstable();
    let mut fenwick_tree = FenwickTree::new(Additive(Canonical::<usize>::new()), n);
    let mut count = 0;
    let mut c = vec![0; n];
    let mut l = 0;
    while l < n {
        let mut r = l + 1;
        while r < n && order[r].0 == order[l].0 {
            r += 1;
        }
        for (k, &(_, i)) in (l..r).zip(&order[l..r]) {
            count += k - fenwick_tree.prefix_fold(i);
            fenwick_tree.op_assign(i, &1);
            c[i] = (n - r).wrapping_sub(l);
        }
        l = r;
    }
    for x in &mut c {
        let d = *x;
        *x = count;
        count = count.wrapping_add(d);
    }
    c
}

/// The numbers of inversions of the ranges of `a`.
///
/// # Definition
/// For a sequence `a` of length `n`, `t[l][r]` is the number of inversions, as defined in
/// [`count_inversions`], of `a[l..r]` for `0 <= l <= r <= n`, and `t[l][r] = 0` for `l > r`.
///
/// # Complexity
/// - Time: O(n^2)
/// - Space: O(n^2)
pub fn range_inversions<T: Ord>(a: &[T]) -> Vec<Vec<usize>> {
    let n = a.len();
    let mut t = vec![vec![0; n + 1]; n + 1];
    for l in (0..n).rev() {
        let (upper, lower) = t.split_at_mut(l + 1);
        let (row, next) = (&mut upper[l], &lower[0]);
        for r in l + 2..=n {
            row[r] = row[r - 1] + next[r] - next[r - 1] + usize::from(a[l] > a[r - 1]);
        }
    }
    t
}

/// The adjacent swap distance from `a` to `b`.
///
/// # Definition
/// The minimum number of swaps of two adjacent elements that turn `a` into `b`, or `None` if `b` is
/// not a rearrangement of `a`.
///
/// # Complexity
/// - Time: O(n log n), where `n = min(a.len(), b.len())`
/// - Space: O(n)
pub fn adjacent_swap_distance<T: Ord>(a: &[T], b: &[T]) -> Option<usize> {
    if a.len() != b.len() {
        return None;
    }
    let n = a.len();
    let mut x: Vec<(&T, usize)> = a.iter().zip(0..).collect();
    let mut y: Vec<(&T, usize)> = b.iter().zip(0..).collect();
    x.sort_unstable();
    y.sort_unstable();
    let mut p = vec![0; n];
    for (&(u, i), &(v, j)) in x.iter().zip(&y) {
        if u != v {
            return None;
        }
        p[i] = j;
    }
    let mut fenwick_tree = FenwickTree::new(Additive(Canonical::<usize>::new()), n);
    let mut count = 0;
    for (k, &j) in p.iter().enumerate() {
        count += k - fenwick_tree.prefix_fold(j);
        fenwick_tree.op_assign(j, &1);
    }
    Some(count)
}
