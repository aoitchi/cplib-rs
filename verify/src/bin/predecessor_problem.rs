use std::io::{BufWriter, Read, Write, stdin, stdout};

use cplib::collections::w_ary_tree::WAryTree;

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
    let q = parse!(u32);
    let t: &[u8] = iter.next().unwrap();
    let mut set = WAryTree::new(n);
    for (i, &t) in t.iter().enumerate() {
        if t == b'1' {
            set.insert(i);
        }
    }
    for _ in 0..q {
        let c = parse!(u8);
        let k = parse!(usize);
        match c {
            0 => {
                set.insert(k);
            }
            1 => {
                set.remove(k);
            }
            2 => {
                let ans = set.contains(k);
                writeln!(stdout, "{}", ans as u8).ok();
            }
            3 => {
                let ans = set.ceil(k).unwrap_or(!0);
                writeln!(stdout, "{}", ans as isize).ok();
            }
            4 => {
                let ans = set.floor(k).unwrap_or(!0);
                writeln!(stdout, "{}", ans as isize).ok();
            }
            _ => unreachable!(),
        }
    }
}
