extern crate gregex;
use gregex::*;

fn main() {
    let runner = regex!(star!('a'));
    assert_eq!(runner.run("a"), true);
    assert_eq!(runner.run("aa"), true);
    assert_eq!(runner.run(""), true);
}
