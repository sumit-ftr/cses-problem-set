use std::collections::BTreeMap;

fn main() {
    let mut token = Tokenizer::new();
    let n: usize = token.next();
    let m: usize = token.next();
    let mut tickets: BTreeMap<u32, u32> = BTreeMap::new();
    for _ in 0..n {
        let x: u32 = token.next();
        tickets.entry(x).and_modify(|v| *v += 1).or_insert(1);
    }
    for _ in 0..m {
        let price: u32 = token.next();
        let mut flag: Option<u32> = None;
        if let Some((k, v)) = tickets.range_mut(..=price).rev().next() {
            if *v > 0 {
                println!("{}", k);
                *v -= 1;
            }
            if *v == 0 {
                flag = Some(k.clone());
            }
        } else {
            println!("-1");
        }
        if let Some(k) = flag {
            tickets.remove(&k);
        }
    }
}

struct Tokenizer {
    buf: Vec<String>,
    i: usize
}

impl Tokenizer {
    pub fn new() -> Self {
        return Tokenizer { buf: Vec::<String>::new(), i: 0 };
    }

    fn read_line(&mut self) {
        let mut s = String::new();
        std::io::stdin().read_line(&mut s).unwrap();
        self.buf = s.split_whitespace().map(str::to_string).collect();
        self.i = 0;
    }

    pub fn next<T : std::str::FromStr>(&mut self) -> T
    where T::Err : std::fmt::Debug {
        while self.i == self.buf.len() {
            self.read_line();
        }
        let t = self.buf[self.i].parse().unwrap();
        self.i += 1;
        return t;
    }

    #[allow(dead_code)]
    pub fn next_line(&self) -> String {
        let mut s = String::new();
        std::io::stdin().read_line(&mut s).unwrap();
        return s;
    }
}

