extern crate gregex;
use gregex::*;

fn main() {
    let runner = regex!("a*");
    assert_eq!(runner.matches_exact("a"), true);
    assert_eq!(runner.matches_exact("aa"), true);
    assert_eq!(runner.matches_exact(""), true);
}
