extern crate gregex;
use gregex::*;

fn main() {
    // Test Plus operator - one or more
    let runner_plus = regex!(plus!('a'));
    println!("Testing Plus operator (a+):");
    assert_eq!(runner_plus.run("a"), true, "a+ should match 'a'");
    assert_eq!(runner_plus.run("aa"), true, "a+ should match 'aa'");
    assert_eq!(runner_plus.run("aaa"), true, "a+ should match 'aaa'");
    assert_eq!(runner_plus.run(""), false, "a+ should NOT match ''");
    assert_eq!(runner_plus.run("b"), false, "a+ should NOT match 'b'");
    println!("✓ Plus operator tests passed!\n");

    // Test Question operator - zero or one
    let runner_question = regex!(question!('b'));
    println!("Testing Question operator (b?):");
    assert_eq!(runner_question.run(""), true, "b? should match ''");
    assert_eq!(runner_question.run("b"), true, "b? should match 'b'");
    assert_eq!(runner_question.run("bb"), false, "b? should NOT match 'bb'");
    assert_eq!(runner_question.run("a"), false, "b? should NOT match 'a'");
    println!("✓ Question operator tests passed!\n");

    // Test combination: a+b? (one or more 'a' followed by zero or one 'b')
    let runner_combo = regex!(dot!(plus!('a'), question!('b')));
    println!("Testing combination (a+b?):");
    assert_eq!(runner_combo.run("a"), true, "a+b? should match 'a'");
    assert_eq!(runner_combo.run("ab"), true, "a+b? should match 'ab'");
    assert_eq!(runner_combo.run("aa"), true, "a+b? should match 'aa'");
    assert_eq!(runner_combo.run("aab"), true, "a+b? should match 'aab'");
    assert_eq!(
        runner_combo.run("abb"),
        false,
        "a+b? should NOT match 'abb'"
    );
    assert_eq!(runner_combo.run(""), false, "a+b? should NOT match ''");
    assert_eq!(runner_combo.run("b"), false, "a+b? should NOT match 'b'");
    println!("✓ Combination tests passed!\n");

    println!("🎉 All Plus and Question operator tests passed!");
}
