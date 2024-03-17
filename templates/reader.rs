fn main() {
    // let v = read::<T>().unwrap();
    // let mut _lines: Vec<Vec<i64>> = read_lines::<i64>();
}

#[allow(unused)]
fn read_line() -> String {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line
}

#[allow(unused)]
fn read<T: std::str::FromStr>() -> Result<T, T::Err> {
    read_line().trim().parse::<T>()
}

#[allow(unused)]
fn read_vec<T: std::str::FromStr>() -> Result<Vec<T>, T::Err> {
    read_line()
        .split_whitespace()
        .map(|x| x.parse::<T>())
        .collect()
}

#[allow(unused)]
fn read_lines<T: std::str::FromStr + std::fmt::Debug>() -> Vec<Vec<T>>
where
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    std::io::stdin()
        .lines()
        .map(|l| {
            l.unwrap()
                .split(" ")
                .map(|x| x.parse::<T>().unwrap())
                .collect()
        })
        .collect::<Vec<Vec<T>>>()
}
