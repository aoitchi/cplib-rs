use crate::algebra::Zero;

/// The diameter of a tree, with the eccentricities of its vertices.
///
/// # Definition
/// `T` is the tree on `[0, n)` whose `i`-th edge joins `u` and `v` with weight `weight(i)` for
/// `edges[i] = (u, v)`, given to [`TreeDiameter::from_edges`], and `d(x, y)` is the total weight of
/// the path joining `x` and `y`. The diameter is `D = max d(x, y)` over all pairs of vertices, and
/// `(s, t)` is a pair attaining it; which one is unspecified when several exist. The eccentricity
/// of `v` is `ecc(v) = max_x d(v, x)`.
///
/// # Invariants
/// - `d(s, t) = D`.
/// - `dist_s[v] = d(s, v)`, and `parent_s[v]` is the neighbor of `v` on the path from `v` to `s`,
///   with `parent_s[s] = s`; likewise `dist_t` and `parent_t` for `t`.
///
/// # Complexity
/// - Space: O(n)
pub struct TreeDiameter<W> {
    s: usize,
    t: usize,
    dist_s: Box<[W]>,
    dist_t: Box<[W]>,
    parent_s: Box<[usize]>,
    parent_t: Box<[usize]>,
}

impl<W: Copy + Ord + std::ops::Add<Output = W> + Zero> TreeDiameter<W> {
    /// The diameter of the tree given by `edges` and `weight`.
    ///
    /// # Definition
    /// `weight(i)` is called exactly once for each `i`.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(n)
    ///
    /// # Panics
    /// Panics if `n = 0`.
    /// Panics if some `(u, v)` in `edges` satisfies `u >= n` or `v >= n`, some `weight(i)` is less
    /// than `W::zero()`, or `edges` is not the edge set of a tree on `[0, n)`.
    pub fn from_edges(
        n: usize,
        edges: &[(usize, usize)],
        mut weight: impl FnMut(usize) -> W,
    ) -> Self {
        assert!(n > 0, "n must be positive");
        assert_eq!(
            edges.len(),
            n - 1,
            "not a tree: number of edges={}, n={n}",
            edges.len()
        );
        let mut adjacency = vec![vec![]; n];
        for (i, &(u, v)) in edges.iter().enumerate() {
            assert!(u < n, "vertex out of bounds: u={u}, n={n}");
            assert!(v < n, "vertex out of bounds: v={v}, n={n}");
            let w = weight(i);
            assert!(w >= W::zero(), "weight must be non-negative");
            adjacency[u].push((v, w));
            adjacency[v].push((u, w));
        }
        let mut dist = vec![W::zero(); n];
        let mut seen = vec![false; n];
        seen[0] = true;
        let mut reached = 1;
        let mut stack = vec![0];
        let mut s = 0;
        let mut mx_dist = W::zero();
        while let Some(u) = stack.pop() {
            for &(v, w) in &adjacency[u] {
                if !seen[v] {
                    seen[v] = true;
                    reached += 1;
                    dist[v] = dist[u] + w;
                    if mx_dist < dist[v] {
                        mx_dist = dist[v];
                        s = v;
                    }
                    stack.push(v);
                }
            }
        }
        assert_eq!(reached, n, "not a tree: reachable={reached}, n={n}");

        let mut dist_s = vec![W::zero(); n];
        let mut parent_s = vec![s; n];
        seen.fill(false);
        seen[s] = true;
        stack.push(s);
        let mut t = s;
        let mut mx_dist = W::zero();
        while let Some(u) = stack.pop() {
            for &(v, w) in &adjacency[u] {
                if !seen[v] {
                    seen[v] = true;
                    dist_s[v] = dist_s[u] + w;
                    parent_s[v] = u;
                    if mx_dist < dist_s[v] {
                        mx_dist = dist_s[v];
                        t = v;
                    }
                    stack.push(v);
                }
            }
        }

        let mut dist_t = vec![W::zero(); n];
        let mut parent_t = vec![t; n];
        seen.fill(false);
        seen[t] = true;
        stack.push(t);
        while let Some(u) = stack.pop() {
            for &(v, w) in &adjacency[u] {
                if !seen[v] {
                    seen[v] = true;
                    dist_t[v] = dist_t[u] + w;
                    parent_t[v] = u;
                    stack.push(v);
                }
            }
        }
        Self {
            s,
            t,
            dist_s: dist_s.into(),
            dist_t: dist_t.into(),
            parent_s: parent_s.into(),
            parent_t: parent_t.into(),
        }
    }

    /// The length `D`.
    ///
    /// # Complexity
    /// - Time: O(1)
    /// - Space: O(1)
    pub fn diameter(&self) -> W {
        self.dist_s[self.t]
    }

    /// The pair `(s, t)`.
    ///
    /// # Complexity
    /// - Time: O(1)
    /// - Space: O(1)
    pub fn endpoints(&self) -> (usize, usize) {
        (self.s, self.t)
    }

    /// The eccentricity `ecc(v)`.
    ///
    /// # Complexity
    /// - Time: O(1)
    /// - Space: O(1)
    ///
    /// # Panics
    /// Panics if `v >= n`.
    pub fn eccentricity(&self, v: usize) -> W {
        let n = self.dist_s.len();
        assert!(v < n, "vertex out of bounds: v={v}, n={n}");
        self.dist_s[v].max(self.dist_t[v])
    }

    /// The farther of `s` and `t` from `v`, or `s` if they are equally far, which is a vertex `x`
    /// with `d(v, x) = ecc(v)`.
    ///
    /// # Complexity
    /// - Time: O(1)
    /// - Space: O(1)
    ///
    /// # Panics
    /// Panics if `v >= n`.
    pub fn farthest(&self, v: usize) -> usize {
        let n = self.dist_s.len();
        assert!(v < n, "vertex out of bounds: v={v}, n={n}");
        if self.dist_s[v] < self.dist_t[v] {
            self.t
        } else {
            self.s
        }
    }

    /// The vertices of the path from `v` to `farthest(v)`, in order from `v`.
    ///
    /// # Complexity
    /// - Time: O(k), where `k` is the number of vertices on the path
    /// - Space: O(k)
    ///
    /// # Panics
    /// Panics if `v >= n`.
    pub fn ecc_path(&self, mut v: usize) -> Vec<usize> {
        let x = self.farthest(v);
        let parent = if x == self.t {
            &self.parent_t
        } else {
            &self.parent_s
        };
        let mut path = vec![v];
        while v != x {
            v = parent[v];
            path.push(v);
        }
        path
    }
}

/// The center of a tree.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Center {
    /// A single vertex.
    Vertex(usize),
    /// Two adjacent vertices `u < v`.
    Edge(usize, usize),
}

/// The center of the tree given by `edges`.
///
/// # Definition
/// `T` is the tree on `[0, n)` whose `i`-th edge joins `u` and `v` for `edges[i] = (u, v)`, and
/// `d(x, y)` is the number of edges on the path joining `x` and `y`. The eccentricity of `v` is
/// `ecc(v) = max_x d(v, x)`, and the center is the set of vertices minimizing `ecc`, which is a
/// single vertex or two adjacent vertices.
///
/// # Complexity
/// - Time: O(n)
/// - Space: O(n)
///
/// # Panics
/// Panics if `n = 0`.
/// Panics if `n >= 2^32`.
/// Panics if some `(u, v)` in `edges` satisfies `u >= n` or `v >= n`, or `edges` is not the edge
/// set of a tree on `[0, n)`.
pub fn tree_center(n: usize, edges: &[(usize, usize)]) -> Center {
    assert!(n > 0, "n must be positive");
    assert!(n < 1 << 32, "n must be less than 2^32: n={n}");
    assert_eq!(
        edges.len(),
        n - 1,
        "not a tree: number of edges={}, n={n}",
        edges.len()
    );
    let mut adjacency = vec![vec![]; n];
    for &(u, v) in edges {
        assert!(u < n, "vertex out of bounds: u={u}, n={n}");
        assert!(v < n, "vertex out of bounds: v={v}, n={n}");
        adjacency[u].push(v);
        adjacency[v].push(u);
    }
    let mut dist = vec![0u32; n];
    let mut seen = vec![false; n];
    seen[0] = true;
    let mut reached = 1;
    let mut stack = vec![0];
    let mut s = 0;
    while let Some(u) = stack.pop() {
        for &v in &adjacency[u] {
            if !seen[v] {
                seen[v] = true;
                reached += 1;
                dist[v] = dist[u] + 1;
                if dist[s] < dist[v] {
                    s = v;
                }
                stack.push(v);
            }
        }
    }
    assert_eq!(reached, n, "not a tree: reachable={reached}, n={n}");

    let mut parent = vec![s; n];
    dist.fill(0);
    seen.fill(false);
    seen[s] = true;
    stack.push(s);
    let mut t = s;
    while let Some(u) = stack.pop() {
        for &v in &adjacency[u] {
            if !seen[v] {
                seen[v] = true;
                dist[v] = dist[u] + 1;
                parent[v] = u;
                if dist[t] < dist[v] {
                    t = v;
                }
                stack.push(v);
            }
        }
    }

    let diameter = dist[t];
    let mut c = t;
    for _ in 0..diameter / 2 {
        c = parent[c];
    }
    if diameter & 1 == 0 {
        Center::Vertex(c)
    } else {
        let p = parent[c];
        Center::Edge(c.min(p), c.max(p))
    }
}
