use crate::algebra::{Commutative, Monoid};
use crate::graph::tree::Tree;

/// The values of the subtrees of a tree for every choice of the root (rerooting).
///
/// # Definition
/// `n`, `edges`, `op`, `id`, `lift` and `wrap` are those given to [`Rerooting::from_edges`]. The
/// tree is on `[0, n)`, and its `i`-th edge joins `u` and `v` for `edges[i] = (u, v)`.
///
/// Root the tree at `r`. For a vertex `v`, let `c_1, ..., c_k` be its children and `e_j` the index
/// of the edge joining `c_j` and `v`. The value of the subtree of `v` is defined from the leaves up
/// by
///
/// `fold(v, r) = wrap(lift(fold(c_1, r), c_1, v, e_1) ... lift(fold(c_k, r), c_k, v, e_k), v)`,
///
/// where the product inside `wrap` is under `op`, which is `id()` for `k = 0`. That is, the values
/// of the children are carried over their edges by `lift`, combined by `op`, and `v` is added by
/// `wrap`. The product does not depend on the order of the children, as `op` is commutative.
///
/// # Invariants
/// - `tree` is the tree rooted at `0`.
/// - For `v != 0` with `k = tree.index(v)`, `down[k - 1] = fold(v, 0)` and
///   `up[k - 1] = fold(tree.parent(v), v)`.
/// - `all[tree.index(v)] = fold(v, v)` for every `v`.
///
/// # Complexity
/// - Space: O(n)
pub struct Rerooting<T> {
    tree: Tree,
    down: Box<[T]>,
    up: Box<[T]>,
    all: Box<[T]>,
}

impl<T> Rerooting<T> {
    /// The values for the tree given by `edges`, the commutative monoid `monoid`, and the maps
    /// `lift` and `wrap`.
    ///
    /// # Definition
    /// The arguments of `lift` and `wrap` are those in the definition of `fold`. In `lift(x, c, v, i)`,
    /// `c` and `v` are joined by the `i`-th edge, and `x = fold(c, v)`, the value of the subtree of
    /// `c` when the tree is rooted at `v`. In `wrap(y, v)`, `y` is the product under `op` of
    /// `lift(fold(c, v), c, v, i)` over all the neighbors `c` of `v`, or over all but one of them,
    /// where `i` is the index of the edge joining `c` and `v`.
    ///
    /// # Contract
    /// `lift` and `wrap` depend only on their arguments.
    ///
    /// # Complexity
    /// - Time: O(n), and O(n) calls of `op`, `id`, `lift` and `wrap`
    /// - Space: O(n)
    ///
    /// # Panics
    /// Panics if `n = 0`, some `(u, v)` in `edges` satisfies `u >= n` or `v >= n`, or `edges` is
    /// not the edge set of a tree on `[0, n)`.
    pub fn from_edges<M: Monoid + Commutative>(
        n: usize,
        edges: &[(usize, usize)],
        monoid: &M,
        mut lift: impl FnMut(&T, usize, usize, usize) -> M::Value,
        mut wrap: impl FnMut(&M::Value, usize) -> T,
    ) -> Self {
        let mut adjacency = vec![vec![]; n];
        for &(u, v) in edges {
            assert!(u < n, "vertex out of bounds: u={u}, n={n}");
            assert!(v < n, "vertex out of bounds: v={v}, n={n}");
            adjacency[u].push(v);
            adjacency[v].push(u);
        }
        let tree = Tree::from_adjacency(&adjacency, 0);
        let mut edge = vec![0; n];
        for (i, &(u, v)) in edges.iter().enumerate() {
            if tree.parent(u) == v {
                edge[u] = i;
            } else {
                edge[v] = i;
            }
        }

        let mut acc: Vec<M::Value> = (0..n).map(|_| monoid.id()).collect();
        let mut down = Vec::with_capacity(n - 1);
        let mut lifted = Vec::with_capacity(n - 1);
        let mut prefix = Vec::with_capacity(n - 1);
        for i in (1..n).rev() {
            let v = tree.vertex(i);
            let p = tree.parent(v);
            let below = wrap(&acc[v], v);
            let lifted_v = lift(&below, v, p, edge[v]);
            let next = monoid.op(&acc[p], &lifted_v);
            prefix.push(std::mem::replace(&mut acc[p], next));
            lifted.push(lifted_v);
            down.push(below);
        }

        let mut up = Vec::with_capacity(n - 1);
        let mut all = Vec::with_capacity(n);
        all.push(wrap(&acc[0], 0));
        acc[0] = monoid.id();
        for i in 1..n {
            let v = tree.vertex(i);
            let p = tree.parent(v);
            let j = n - 1 - i;
            let above = wrap(&monoid.op(&prefix[j], &acc[p]), p);
            acc[p] = monoid.op(&acc[p], &lifted[j]);
            let side = lift(&above, p, v, edge[v]);
            all.push(wrap(&monoid.op(&side, &acc[v]), v));
            acc[v] = side;
            up.push(above);
        }
        down.reverse();
        Self {
            tree,
            down: down.into(),
            up: up.into(),
            all: all.into(),
        }
    }

    /// The value `fold(v, root)` of the subtree of `v` when the tree is rooted at `root`, which is
    /// the value of the whole tree for `v = root`.
    ///
    /// # Complexity
    /// - Time: O(log n)
    /// - Space: O(1)
    ///
    /// # Panics
    /// Panics if `v >= n` or `root >= n`.
    pub fn fold(&self, v: usize, root: usize) -> &T {
        let n = self.all.len();
        assert!(v < n, "index out of bounds: v={v}, len={n}");
        assert!(root < n, "index out of bounds: root={root}, len={n}");
        if v == root {
            &self.all[self.tree.index(v)]
        } else if self.tree.is_ancestor(v, root) {
            let c = self.tree.jump(v, root, 1).unwrap();
            &self.up[self.tree.index(c) - 1]
        } else {
            &self.down[self.tree.index(v) - 1]
        }
    }
}
