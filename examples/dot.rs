extern crate gregex;
use gregex::*;

fn main() {
    let runner = regex!(dot!('a', 'b', 'c'));
    assert_eq!(runner.run("abc"), true);
    assert_eq!(runner.run("ab"), false);
    assert_eq!(runner.run("abcd"), false);
}
