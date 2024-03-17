fn main() {
    init!(inp, _outp, IOConf::Stdio);

    inp.read::<usize>();
    let mut vals = inp.read_vec::<i64>();
}

#[allow(dead_code)]
fn lock_stdout() -> std::io::BufWriter<Box<dyn std::io::Write>> {
    let stdout = std::io::stdout().lock();
    std::io::BufWriter::new(Box::new(stdout))
}

#[allow(dead_code)]
fn get_file_buf(file: &str) -> std::io::BufWriter<Box<dyn std::io::Write>> {
    let fsin = std::fs::File::create(file).unwrap();
    std::io::BufWriter::new(Box::new(fsin))
}

#[allow(dead_code)]
struct OutpWriter<T: std::io::Write> {
    wr: std::io::BufWriter<T>,
}

#[allow(dead_code)]
impl<T: std::io::Write> OutpWriter<T> {
    fn flush(&mut self) {
        std::io::Write::flush(&mut self.wr).expect("flush failed")
    }

    fn log(&mut self, s: &str) {
        std::io::Write::write(&mut self.wr, s.as_bytes()).expect("error writing stdout");
        std::io::Write::write(&mut self.wr, "\n".as_bytes()).expect("error writing stdout");
    }
}
#[allow(dead_code)]
struct InpReader<T: std::io::Read> {
    lines: std::io::Lines<std::io::BufReader<T>>,
}
impl<T: std::io::Read> InpReader<T> {
    #[allow(dead_code)]
    fn read2<K: std::str::FromStr + Copy>(&mut self) -> [K; 2]
    where
        K::Err: std::fmt::Debug,
    {
        let v = self.read_vec();
        [v[0], v[1]]
    }

    #[allow(dead_code)]
    fn read3<K: std::str::FromStr + Copy>(&mut self) -> [K; 3]
    where
        K::Err: std::fmt::Debug,
    {
        let v = self.read_vec();
        [v[0], v[1], v[2]]
    }

    #[allow(dead_code)]
    fn read4<K: std::str::FromStr + Copy>(&mut self) -> [K; 4]
    where
        K::Err: std::fmt::Debug,
    {
        let v = self.read_vec();
        [v[0], v[1], v[2], v[3]]
    }

    #[allow(dead_code)]
    fn read_line(&mut self) -> String {
        let line = self.lines.next();
        let line = line.expect("no more lines");
        line.expect("failed to read line")
    }

    #[allow(dead_code)]
    fn read<K: std::str::FromStr>(&mut self) -> K
    where
        K::Err: std::fmt::Debug,
    {
        self.read_line()
            .trim()
            .parse::<K>()
            .expect("failed to read value")
    }

    #[allow(dead_code)]
    fn read_vec<K: std::str::FromStr>(&mut self) -> Vec<K>
    where
        K::Err: std::fmt::Debug,
    {
        self.read_line()
            .split_whitespace()
            .map(|x| x.parse::<K>().expect("failed to read values"))
            .collect()
    }
}
pub enum IOConf<'a> {
    Stdio,
    Files(&'a str, &'a str),
}
#[allow(dead_code)]
macro_rules! init {
    ($inp_name: ident, $outp_name: ident, $io_conf: expr) => {
        let x: std::io::BufReader<Box<dyn std::io::Read>> = match $io_conf {
            IOConf::Stdio => std::io::BufReader::new(Box::new(std::io::stdin().lock())),
            IOConf::Files(infile, _) => std::io::BufReader::new(Box::new(
                std::fs::File::open(infile).expect("input file not found"),
            )),
        };
        #[allow(unused_mut)]
        let mut $inp_name = InpReader {
            lines: std::io::BufRead::lines(x),
        };
        #[allow(unused_mut)]
        let mut $outp_name = OutpWriter {
            wr: match $io_conf {
                IOConf::Stdio => lock_stdout(),
                IOConf::Files(_, outfile) => get_file_buf(&outfile),
            },
        };
    };
}
#[allow(dead_code)]
fn it_to_string<T: ToString>(it: impl IntoIterator<Item = T>, sep: Option<&str>) -> String {
    let sl = it.into_iter();
    sl.map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join(sep.unwrap_or(""))
}
#[allow(unused_macros)]
macro_rules! cfor{{$a:ident,$r:expr,$incr:expr,$bl:block}=>{let mut $a = match std::ops::RangeBounds::start_bound(&$r){std::ops::Bound::Included(&x)=>x,std::ops::Bound::Excluded(&x)=>x-1,_=>panic!("unbounded range not allowed in for macro")};let mut b = match std::ops::RangeBounds::end_bound(&$r){std::ops::Bound::Included(&x)=>x,std::ops::Bound::Excluded(&x)=>x-1,_=>panic!("unbounded range not allowed in for macro")};if $incr<0 {[b,$a]=[$a,b]}while $incr>=0&&$a<=b || $incr<0&&$a>=b {$bl $a += $incr;}}}
