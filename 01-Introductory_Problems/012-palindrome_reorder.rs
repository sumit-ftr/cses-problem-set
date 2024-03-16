fn main() {
    let mut token = Tokenizer::new();
    let s: String = token.next();
    // hash taken for storing count of A-Z
    let mut a: [u32; 26] = [0; 26];
    for c in s.chars() {
        a[c as usize - 65] += 1;
    }

    // pre-computation for checking palindrome possiblility
    let mut odd_count: u8 = 0;
    for i in 0..26 {
        if a[i] & 1 == 1 {
            odd_count += 1;
        }
    }

    // checking reordering palindrome possiblity
    if odd_count > 1 {
        println!("NO SOLUTION");
    } else {
        let mut odd_index: usize = 26;
        for i in 0..26 {
            if a[i] & 1 == 0 {
                a[i] >>= 1;
                for _ in 0..a[i] {
                    print!("{}", char::from_u32(i as u32 + 65).unwrap());
                }
            } else {
                odd_index = i;
            }
        }
        if odd_index != 26 {
            for _ in 0..a[odd_index] {
                print!("{}", char::from_u32(odd_index as u32 + 65).unwrap());
            }
            a[odd_index] = 0;
        }
        for i in (0..26).rev() {
            for _ in 0..a[i] {
                print!("{}", char::from_u32(i as u32 + 65).unwrap());
            }
        }
        println!("");
    }
}

struct Tokenizer {
    buf: Vec<String>,
    i: usize,
}

impl Tokenizer {
    pub fn new() -> Self {
        return Tokenizer {
            buf: Vec::<String>::new(),
            i: 0,
        };
    }

    fn read_line(&mut self) {
        let mut s = String::new();
        std::io::stdin().read_line(&mut s).unwrap();
        self.buf = s.split_whitespace().map(str::to_string).collect();
        self.i = 0;
    }

    pub fn next<T: std::str::FromStr>(&mut self) -> T
    where
        T::Err: std::fmt::Debug,
    {
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
