extern crate gregex;
use gregex::*;

fn main() {
    let runner = regex!(or!('a', 'b', 'c'));
    assert_eq!(runner.run("a"), true);
    assert_eq!(runner.run("b"), true);
    assert_eq!(runner.run("c"), true);
}
