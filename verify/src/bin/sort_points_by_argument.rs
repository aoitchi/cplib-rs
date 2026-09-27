use std::io::{BufWriter, Read, Write, stdin, stdout};

use cplib::geometry::point::Point;
use cplib::geometry::vector::Vector;

fn main() {
    let mut input = Vec::new();
    stdin().lock().read_to_end(&mut input).unwrap();
    let mut iter = input.split(|&b| b <= b' ').filter(|s| !s.is_empty());
    let mut stdout = BufWriter::new(stdout().lock());

    macro_rules! parse {
        ($t:ty) => {{
            let s = iter.next().unwrap();
            let (neg, digits) = match s[0] {
                b'-' => (true, &s[1..]),
                _ => (false, s),
            };
            let mut x: $t = 0;
            for &b in digits {
                x = x * 10 + (b - b'0') as $t;
            }
            if neg { 0 - x } else { x }
        }};
    }

    let n = parse!(usize);
    let mut xy: Vec<Point> = (0..n)
        .map(|_| Point::new(parse!(i64), parse!(i64)))
        .collect();
    let key = |q: &Point| {
        let v = *q - Point::ORIGIN;
        if v == Vector::ZERO {
            Vector::new(1, 0)
        } else {
            v
        }
    };
    xy.sort_unstable_by(|a, b| key(a).cmp_arg(key(b)));
    for p in xy {
        writeln!(stdout, "{p}").ok();
    }
}
