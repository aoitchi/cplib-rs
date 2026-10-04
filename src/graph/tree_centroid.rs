/// The centroid of a tree.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Centroid {
    /// A single vertex.
    Vertex(usize),
    /// Two adjacent vertices `u < v`.
    Edge(usize, usize),
}

/// The centroid of the tree given by `edges`.
///
/// # Definition
/// The tree is on `[0, n)`, and its `i`-th edge joins `u` and `v` for `edges[i] = (u, v)`. A
/// centroid is a vertex whose removal leaves only components `C` with `2|C| <= n`, and the centroid
/// is the set of centroids, which is a single vertex or two adjacent vertices.
///
/// # Complexity
/// - Time: O(n)
/// - Space: O(n)
///
/// # Panics
/// Panics if `n = 0` or `n >= 2^32`.
/// Panics if some `(u, v)` in `edges` satisfies `u >= n` or `v >= n`, or `edges` is not the edge
/// set of a tree on `[0, n)`.
pub fn tree_centroid(n: usize, edges: &[(usize, usize)]) -> Centroid {
    assert!(n > 0, "n must be positive");
    assert!(n < 1 << 32, "n must be less than 2^32: n={n}");
    assert_eq!(
        edges.len() + 1,
        n,
        "not a tree: number of edges={}, n={n}",
        edges.len()
    );
    let mut degree = vec![0u32; n];
    let mut xor = vec![0u32; n];
    for &(u, v) in edges {
        assert!(u < n, "vertex out of bounds: u={u}, n={n}");
        assert!(v < n, "vertex out of bounds: v={v}, n={n}");
        degree[u] += 1;
        degree[v] += 1;
        xor[u] ^= v as u32;
        xor[v] ^= u as u32;
    }

    let mut size = vec![1u32; n];
    let mut centroid: Option<usize> = None;
    let mut root = 0;
    let mut peeled = 0;
    for s in 0..n {
        let mut u = s;
        while degree[u] == 1 && peeled + 1 < n {
            let p = xor[u] as usize;
            degree[u] = 0;
            degree[p] -= 1;
            xor[p] ^= u as u32;
            size[p] += size[u];
            if (size[u] as usize) << 1 >= n && centroid.is_none_or(|c| size[u] < size[c]) {
                centroid = Some(u);
            }
            peeled += 1;
            root = p;
            u = p;
        }
    }
    assert_eq!(peeled + 1, n, "not a tree: peeled={peeled}, n={n}");

    let c = centroid.unwrap_or(root);
    if (size[c] as usize) << 1 == n {
        let p = xor[c] as usize;
        Centroid::Edge(c.min(p), c.max(p))
    } else {
        Centroid::Vertex(c)
    }
}
