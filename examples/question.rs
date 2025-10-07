extern crate gregex;
use gregex::*;

fn main() {
    let runner = regex!(question!('a'));
    assert_eq!(runner.matches_exact("a"), true);
    assert_eq!(runner.matches_exact("aa"), false);
    assert_eq!(runner.matches_exact(""), true); // a? should match empty string
}
