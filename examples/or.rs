extern crate gregex;
use gregex::*;

fn main() {
    let runner = regex!("a|b|c");
    assert_eq!(runner.matches_exact("a"), true);
    assert_eq!(runner.matches_exact("b"), true);
    assert_eq!(runner.matches_exact("c"), true);
}
