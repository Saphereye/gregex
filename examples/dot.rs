extern crate gregex;
use gregex::*;

fn main() {
    let runner = regex!("abc");
    assert_eq!(runner.matches_exact("abc"), true);
    assert_eq!(runner.matches_exact("ab"), false);
    assert_eq!(runner.matches_exact("abcd"), false);
}
