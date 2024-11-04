use std::str::FromStr;

use num::Complex;

pub fn parse_pair<T: FromStr>(s: &str, separator: char) -> Option<(T, T)> {
    match s.find(separator) {
        None => None,
        Some(index) => match (T::from_str(&s[..index]), T::from_str(&s[index + 1..])) {
            (Ok(l), Ok(r)) => Some((l, r)),
            _ => None,
        },
    }
}
pub fn parse_complex(s: &str) -> Option<Complex<f64>> {
    match parse_pair(s, ',') {
        Some((re, im)) => Some(Complex { re, im }),
        None => None
    }
}
fn main() {

    // println!("Hello, world!");
}
#[test]
fn test_parse_pair() {
    assert_eq!(parse_pair::<i32>("4,4", ','), Some((4, 4)))
}
#[test]
fn test_parse_complex() {
    assert_eq!(parse_complex("11.1,54.5"), Some(Complex{re: 11.1, im: 54.5}))
}