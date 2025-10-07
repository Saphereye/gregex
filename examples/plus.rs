extern crate gregex;
use gregex::*;

fn main() {
    let runner = regex!("a+");
    println!("Testing 'a': {}", runner.matches_exact("a"));
    println!("Testing 'aa': {}", runner.matches_exact("aa"));
    println!("Testing 'aaa': {}", runner.matches_exact("aaa"));
    println!("Testing '': {}", runner.matches_exact(""));
    println!("NFA: {:?}", runner);

    assert_eq!(runner.matches_exact("a"), true);
    assert_eq!(runner.matches_exact("aa"), true);
    assert_eq!(runner.matches_exact("aaa"), true);
    assert_eq!(runner.matches_exact(""), false);
}
