use std::io::Write;

struct Node {
    pos: u32,
    prev: i32,
    next: i32,
}

fn main() {
    let mut token = Scanner::new(std::io::stdin().lock());
    let mut out = std::io::BufWriter::new(std::io::stdout().lock());
    let x = token.next::<u32>();
    let n = token.next::<usize>();
    let mut lights = Vec::<(u32, u32)>::with_capacity(n);
    let mut mapped = Vec::<u32>::with_capacity(n);
    let mut lt = Vec::<Node>::with_capacity(n + 2);
    let mut gaps = Vec::<u32>::with_capacity(n);
    let mut max_gap = u32::MIN;

    for i in 0..n {
        lights.push((token.next::<u32>(), i as u32));
        mapped.push(0);
        gaps.push(0);
    }
    lights.sort();

    lt.push(Node {
        pos: 0,
        prev: -1,
        next: 1,
    });
    for i in 0..n {
        mapped[lights[i].1 as usize] = i as u32;
        lt.push(Node {
            pos: lights[i].0,
            prev: i as i32,
            next: i as i32 + 2,
        });
        max_gap = u32::max(max_gap, lt[i + 1].pos - lt[i].pos);
    }
    lt.push(Node {
        pos: x,
        prev: n as i32,
        next: -1,
    });
    max_gap = u32::max(max_gap, lt[n + 1].pos - lt[n].pos);
    gaps[n - 1] = max_gap;

    for i in (1..n).rev() {
        let t1 = lt[mapped[i] as usize + 1].prev as usize;
        let t2 = lt[mapped[i] as usize + 1].next as usize;
        lt[t1].next = t2 as i32;
        lt[t2].prev = t1 as i32;
        max_gap = u32::max(max_gap, lt[t2].pos - lt[t1].pos);
        gaps[i - 1] = max_gap;
    }

    for i in 0..n {
        write!(out, "{} ", gaps[i]).unwrap();
    }
}

// use std::collections::{BTreeMap, BTreeSet};
// #[allow(dropping_references)]
// fn main() {
//     let mut token = Scanner::new(std::io::stdin().lock());
//     let mut out = std::io::BufWriter::new(std::io::stdout().lock());
//     let x = token.next::<u32>();
//     let n = token.next::<usize>();
//     let mut pos = BTreeSet::<u32>::from([0, x]);
//     let mut max = BTreeMap::<u32, u32>::from([(x, 1)]);
//     for _ in 0..n {
//         let p = token.next::<u32>();
//         pos.insert(p);
//         let before = pos.range(..p).next_back().unwrap().clone();
//         let after = pos.range(p + 1..).next().unwrap().clone();
//         if let Some((k, v)) = max.range_mut(after - before..).next() {
//             if *v == 1 {
//                 let key = k.clone();
//                 drop(k);
//                 drop(v);
//                 max.remove(&key);
//             } else {
//                 *v -= 1;
//             }
//             max.entry(after - p).and_modify(|x| *x += 1).or_insert(1);
//             max.entry(p - before).and_modify(|x| *x += 1).or_insert(1);
//         }
//         write!(out, "{} ", max.last_entry().unwrap().key()).unwrap();
//     }
// }

pub struct Scanner<R> {
    reader: R,
    buf_str: Vec<u8>,
    buf_iter: std::str::SplitAsciiWhitespace<'static>,
}

impl<R: std::io::BufRead> Scanner<R> {
    pub fn new(reader: R) -> Self {
        Self {
            reader,
            buf_str: vec![],
            buf_iter: "".split_ascii_whitespace(),
        }
    }

    pub fn next<T: std::str::FromStr>(&mut self) -> T {
        loop {
            if let Some(token) = self.buf_iter.next() {
                return token.parse().ok().expect("Failed parse");
            }
            self.buf_str.clear();
            self.reader
                .read_until(b'\n', &mut self.buf_str)
                .expect("Failed read");
            self.buf_iter = unsafe {
                let slice = std::str::from_utf8_unchecked(&self.buf_str);
                std::mem::transmute(slice.split_ascii_whitespace())
            }
        }
    }
}
