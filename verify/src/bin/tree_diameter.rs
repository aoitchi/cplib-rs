use std::io::{BufWriter, Read, Write, stdin, stdout};

use cplib::graph::tree_metric::TreeDiameter;

fn main() {
    let mut input = Vec::new();
    stdin().lock().read_to_end(&mut input).unwrap();
    let mut iter = input.split(|&b| b <= b' ').filter(|s| !s.is_empty());
    let mut stdout = BufWriter::new(stdout().lock());

    macro_rules! parse {
        ($t:ty) => {{
            let s = iter.next().unwrap();
            let mut x: $t = 0;
            for &b in s {
                x = x * 10 + (b - b'0') as $t;
            }
            x
        }};
    }

    let n = parse!(usize);

    let mut e = Vec::with_capacity(n - 1);
    let mut d = Vec::with_capacity(n - 1);
    for _ in 0..n - 1 {
        let a = parse!(usize);
        let b = parse!(usize);
        let c = parse!(u64);
        e.push((a, b));
        d.push(c);
    }

    let diameter = TreeDiameter::from_edges(n, &e, |i| d[i]);
    let (_, t) = diameter.endpoints();
    let path = diameter.ecc_path(t);
    writeln!(stdout, "{} {}", diameter.diameter(), path.len()).ok();
    for v in &path {
        write!(stdout, "{v} ").ok();
    }
    writeln!(stdout).ok();
}
