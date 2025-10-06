extern crate gregex;
use gregex::*;

fn main() {
    let runner = regex!(plus!('a'));
    println!("Testing 'a': {}", runner.run("a"));
    println!("Testing 'aa': {}", runner.run("aa"));
    println!("Testing 'aaa': {}", runner.run("aaa"));
    println!("Testing '': {}", runner.run(""));
    println!("NFA: {:?}", runner);

    assert_eq!(runner.run("a"), true);
    assert_eq!(runner.run("aa"), true);
    assert_eq!(runner.run("aaa"), true);
    assert_eq!(runner.run(""), false);
}
