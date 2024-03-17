fn solve(mut sc: Scanner, op: &mut impl std::io::Write) {}

fn main() {
    let sc = Scanner::new(std::io::stdin());
    let mut op = std::io::BufWriter::new(std::io::stdout().lock());
    solve(sc, &mut op);
}

#[inline]
fn gcd<T>(a: T, b: T) -> T
where
    T: PartialEq + std::ops::Rem<Output = T> + Default + Copy,
{
    if b == T::default() {
        a
    } else {
        gcd(b, a % b)
    }
}

#[inline]
fn lcm<T>(a: T, b: T) -> T
where
    T: PartialEq
        + std::ops::Rem<Output = T>
        + std::ops::Mul<Output = T>
        + std::ops::Div<Output = T>
        + Default
        + Copy,
{
    a * b / gcd(a, b)
}
