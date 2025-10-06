extern crate gregex;
use gregex::*;

fn main() {
    let runner = regex!(question!('a'));
    assert_eq!(runner.run("a"), true);
    assert_eq!(runner.run("aa"), false);
    assert_eq!(runner.run(""), true); // a? should match empty string
}
