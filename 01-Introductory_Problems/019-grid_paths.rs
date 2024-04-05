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

#[allow(non_upper_case_globals)]
fn main() {
    let mut token = Scanner::new(std::io::stdin().lock());
    const n: usize = 7;
    let v: Vec<u8> = token.next::<String>().into();
    let mut vis = vec![vec![false; n]; n];
    let mut paths = 0usize;
    find_paths(&v, 0, &mut vis, 0, 0, n, &mut paths);
    println!("{paths}");
}

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
                return unsafe { token.parse().unwrap_unchecked() };
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
