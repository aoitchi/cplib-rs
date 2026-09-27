use std::io::{BufWriter, Read, Write, stdin, stdout};

use cplib::algebra::assert::AssertCommutative;
use cplib::algebra::closures::FnMonoid;
use cplib::graph::rerooting::Rerooting;
use cplib::num::fp::{Fp, fp};

const P: u32 = 998_244_353;

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
    let a: Vec<Fp<P>> = (0..n).map(|_| fp!(parse!(u32), mod P)).collect();
    let mut e = Vec::with_capacity(n - 1);
    let mut bc = Vec::with_capacity(n - 1);
    for _ in 0..n - 1 {
        let u = parse!(usize);
        let v = parse!(usize);
        let b = parse!(u32);
        let c = parse!(u32);
        e.push((u, v));
        bc.push((fp!(b, mod P), fp!(c, mod P)));
    }

    let monoid = AssertCommutative(FnMonoid {
        id: (fp!(0), fp!(0)),
        op: |x: &(Fp<P>, Fp<P>), y: &(Fp<P>, Fp<P>)| (x.0 + y.0, x.1 + y.1),
    });
    let rerooting = Rerooting::from_edges(
        n,
        &e,
        &monoid,
        |x: &(Fp<P>, Fp<P>), _: usize, _: usize, i: usize| {
            let (b, c) = bc[i];
            (x.0, b * x.1 + c * x.0)
        },
        |y: &(Fp<P>, Fp<P>), v: usize| (y.0 + fp!(1), y.1 + a[v]),
    );
    for i in 0..n {
        write!(stdout, "{} ", rerooting.fold(i, i).1).ok();
    }
    writeln!(stdout).ok();
}
