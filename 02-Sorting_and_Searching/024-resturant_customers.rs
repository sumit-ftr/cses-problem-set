use std::collections::BTreeMap;
#[allow(unused_variables)]

fn main() {
    let mut token = Tokenizer::new();
    let n: usize = token.next();
    let mut map: BTreeMap<i32, i32> = BTreeMap::new();
    for _ in 0..n {
        let x: i32 = token.next();
        let y: i32 = token.next();
        map.entry(x).and_modify(|k| *k+=1).or_insert(1);
        map.entry(y).and_modify(|k| *k-=1).or_insert(-1);
    }
    let mut mc: i32 = 0;
    let mut cc: i32 = 0;
    for (i, count) in &mut map {
        cc += *count;
        *count = cc;
        if mc < *count {
            mc = *count;
        }
    }
    println!("{}", mc);
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
