fn nqueen_variant(
    v: &Vec<Vec<u8>>,
    row: usize,
    sum: &mut usize,
    c: &mut Vec<u8>,
    ld: &mut Vec<u8>,
    rd: &mut Vec<u8>,
) {
    if row == v.len() {
        *sum += 1;
        return;
    }

    for col in 0..v.len() {
        if c[col] == 0 && rd[row + col] == 0 && ld[v.len() - 1 + col - row] == 0 && v[row][col] == 0
        {
            c[col] = 1;
            rd[row + col] = 1;
            ld[v.len() - 1 + col - row] = 1;
            nqueen_variant(v, row + 1, sum, c, ld, rd);
            c[col] = 0;
            rd[row + col] = 0;
            ld[v.len() - 1 + col - row] = 0;
        }
    }
    return;
}

fn main() {
    let mut token = Tokenizer::new();
    let n: usize = 8;
    let mut v: Vec<Vec<u8>> = vec![Vec::with_capacity(n); n];
    let mut c: Vec<u8> = vec![0; n];
    let mut ld: Vec<u8> = vec![0; 2 * n - 1];
    let mut rd: Vec<u8> = vec![0; 2 * n - 1];
    let mut sum: usize = 0;
    // 0 for free , 1 for queens , 2 for reserved
    for i in 0..n {
        let s: String = token.next();
        for ch in s.chars() {
            if ch == '.' {
                v[i].push(0);
            } else {
                v[i].push(2);
            }
        }
    }

    nqueen_variant(&v, 0, &mut sum, &mut c, &mut ld, &mut rd);
    println!("{sum}");
}

/*
fn is_safe(v: &Vec<Vec<u8>>, i: usize, j: usize) -> bool {
    let m = usize::min(i, j);
    for k in 0..v.len() {
        // horizontal and vertical check
        if v[k][j] == 2 || v[i][k] == 2 {
            return false;
        }
        // left diagonal (\) check
        if i-m+k < v.len() && j-m+k < v.len() {
            if v[i-m+k][j-m+k] == 2 {
                return false;
            }
        }
        // right diagonal (/) check
        if i > k && j+k < v.len() {
            if v[i-k][j+k] == 2 {
                return false;
            }
        }
    }
    return true;
}
*/

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
