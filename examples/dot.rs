extern crate gregex;
use gregex::*;

fn main() {
    let runner = regex!(dot!('a', 'b', 'c'));
    assert_eq!(runner.matches_exact("abc"), true);
    assert_eq!(runner.matches_exact("ab"), false);
    assert_eq!(runner.matches_exact("abcd"), false);
}
