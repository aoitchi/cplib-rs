use std::io::{BufWriter, Read, Write, stdin, stdout};

use cplib::arithmetic::quotient::for_each_quotient;

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

    let n = parse!(u64);
    let mut ans = vec![];
    for_each_quotient(n, |q, _, _| ans.push(q));
    ans.reverse();
    writeln!(stdout, "{}", ans.len()).ok();
    for q in ans {
        write!(stdout, "{q} ").ok();
    }
    writeln!(stdout).ok();
}
