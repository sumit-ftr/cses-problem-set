use std::io::Write;

fn main() -> std::io::Result<()> {
    let mut sc = Scanner::new(std::io::stdin().lock());
    let mut op = std::io::BufWriter::new(std::io::stdout().lock());

    Ok(())
}

struct Scanner<R> {
    reader: R,
    buffer: Vec<String>,
}

impl<R: std::io::BufRead> Scanner<R> {
    fn new(reader: R) -> Self {
        Self {
            reader,
            buffer: vec![],
        }
    }

    pub(crate) fn next<T: std::str::FromStr>(&mut self) -> T {
        loop {
            if let Some(token) = self.buffer.pop() {
                return token.parse().ok().expect("Failed parse");
            }
            let mut input = String::new();
            self.reader.read_line(&mut input).expect("Failed read");
            self.buffer = input.split_whitespace().rev().map(String::from).collect();
        }
    }
}

/// For local tests
#[allow(unused)]
fn from_file() -> (
    Scanner<std::io::BufReader<std::fs::File>>,
    std::io::BufWriter<std::fs::File>,
) {
    let input = std::fs::File::open("./src/input.txt").expect("Not found");
    let output = std::fs::File::create("./src/output.txt").expect("Not found");

    (
        Scanner::new(std::io::BufReader::new(input)),
        std::io::BufWriter::new(output),
    )
}
