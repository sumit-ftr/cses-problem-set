use std::io::Write;

trait UpperBound {
    type Item;
    fn upper_bound(&mut self, value: Self::Item) -> Option<&mut Self::Item>;
}

impl<T: std::cmp::PartialOrd> UpperBound for Vec<T> {
    type Item = T;
    fn upper_bound(&mut self, value: T) -> Option<&mut Self::Item> {
        let (mut low, mut high) = (0isize, self.len() as isize - 1);
        let mut res = self.len();
        if res == 0 || self[high as usize] <= value {
            return None;
        }
        while low <= high {
            let mid = (low + high) / 2;
            if self[mid as usize] > value {
                res = mid as usize;
                high = mid - 1;
            } else {
                low = mid + 1;
            }
        }
        Some(&mut self[res])
    }
}

fn main() {
    let mut token = Scanner::new(std::io::stdin().lock());
    let mut out = std::io::BufWriter::new(std::io::stdout().lock());
    let n = token.next::<usize>();
    let mut cubes = Vec::<u32>::with_capacity(n);
    for _ in 0..n {
        let x = token.next::<u32>();
        if let Some(value) = cubes.upper_bound(x) {
            *value = x;
        } else {
            cubes.push(x);
        }
    }
    writeln!(out, "{}", cubes.len()).unwrap();
}

// let mut set = BTreeMap::<u32, u32>::new();
// for _ in 0..n {
//     let x = token.next::<u32>();
//     if let Some((&value, count)) = set.range_mut(x + 1..).next() {
//         if *count == 1 {
//             set.remove(&value);
//         } else {
//             *count -= 1;
//         }
//     } else {
//         sum += 1;
//     }
//     set.entry(x).and_modify(|v| *v += 1).or_insert(1);
// }
// writeln!(out, "{sum}").unwrap();

pub struct Scanner<R> {
    reader: R,
    buffer: Vec<u8>,
    iter: std::str::SplitAsciiWhitespace<'static>,
}

impl<R: std::io::BufRead> Scanner<R> {
    pub fn new(reader: R) -> Self {
        Self {
            reader,
            buffer: vec![],
            iter: "".split_ascii_whitespace(),
        }
    }

    pub fn next<T: std::str::FromStr>(&mut self) -> T {
        loop {
            if let Some(token) = self.iter.next() {
                return token.parse().ok().unwrap();
            }
            self.buffer.clear();
            self.reader.read_until(b'\n', &mut self.buffer).unwrap();
            self.iter = unsafe {
                let slice = std::str::from_utf8_unchecked(&self.buffer);
                std::mem::transmute(slice.split_ascii_whitespace())
            }
        }
    }
}
