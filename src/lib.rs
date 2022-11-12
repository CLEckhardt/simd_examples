#[inline]
pub fn sum_up(n: u64) -> u64 {
    (0..=n).into_iter().fold(0, |a, x| sum(a, x))
}

fn sum(a: u64, x: u64) -> u64 {
    a + x
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(sum_up(5), 15);
    }
}
