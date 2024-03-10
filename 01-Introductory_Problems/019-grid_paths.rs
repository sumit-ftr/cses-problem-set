#[allow(non_snake_case)]
fn find_paths(v: &Vec<u8>, steps: usize, vis: &mut Vec<Vec<bool>>,
i: usize, j: usize, n: usize, paths: &mut usize) {
    let hitNdivide = ((1<=j && j<n-1 && !vis[i][j+1] && !vis[i][j-1]) && ((i==0 && vis[i+1][j]) || (i==n-1 && vis[i-1][j]))) 
                || ((1<=i && i<n-1 && !vis[i+1][j] && !vis[i-1][j]) && ((j==0 && vis[i][j+1]) || (j==n-1 && vis[i][j-1])))
                // this below two lines is when a path divides the board into two seperate closed areas
                || (1<=i && i<n-1 && 1<=j && j<n-1 && vis[i+1][j] && vis[i-1][j] && !vis[i][j+1] && !vis[i][j-1])
                || (1<=i && i<n-1 && 1<=j && j<n-1 && vis[i][j+1] && vis[i][j-1] && !vis[i+1][j] && !vis[i-1][j]);
    if hitNdivide { return; }

    if i==n-1 && j==0 {
        if steps == v.len() {
            *paths += 1;
        }
        return;
    }

    vis[i][j] = true;
    if v[steps] == '?' as u8 {
        if j+1<n && !vis[i][j+1] {
            find_paths(v, steps+1, vis, i, j+1, n, paths);
        }
        if i+1<n && !vis[i+1][j] {
            find_paths(v, steps+1, vis, i+1, j, n, paths);
        }
        if j>0 && !vis[i][j-1] {
            find_paths(v, steps+1, vis, i, j-1, n, paths);
        }
        if i>0 && !vis[i-1][j] {
            find_paths(v, steps+1, vis, i-1, j, n, paths);
        }
    } else if v[steps] == 'R' as u8 {
        if j+1<n && !vis[i][j+1] {
            find_paths(v, steps+1, vis, i, j+1, n, paths);
        }
    } else if v[steps] == 'D' as u8 {
        if i+1<n && !vis[i+1][j] {
            find_paths(v, steps+1, vis, i+1, j, n, paths);
        }
    } else if v[steps] == 'L' as u8 {
        if j>0 && !vis[i][j-1] {
            find_paths(v, steps+1, vis, i, j-1, n, paths);
        }
    } else if v[steps] == 'U' as u8 {
        if i>0 && !vis[i-1][j] {
            find_paths(v, steps+1, vis, i-1, j, n, paths);
        }
    }
    vis[i][j] = false;
    return;
}

fn main() {
    let mut token = Tokenizer::new();
    let n: usize = 7;
    let v: Vec<u8> = token.next::<String>().into();
    let mut vis: Vec<Vec<bool>> = vec![vec![false; n]; n];
    let mut paths: usize = 0;
    find_paths(&v, 0, &mut vis, 0, 0, n, &mut paths);
    println!("{paths}");
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

