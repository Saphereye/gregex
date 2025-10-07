extern crate gregex;
use gregex::*;

fn main() {
    println!("=== API Methods Example ===\n");
    println!("This example demonstrates all available API methods.\n");

    let pattern = regex!("ab+");
    println!("Pattern: \"ab+\" (one 'a' followed by one or more 'b's)\n");

    // 1. matches_exact: Check if entire string matches
    println!("1. matches_exact(text) - Check if entire string matches:");
    assert!(pattern.matches_exact("ab"));
    println!("   matches_exact(\"ab\"): true");

    assert!(pattern.matches_exact("abbb"));
    println!("   matches_exact(\"abbb\"): true");

    assert!(!pattern.matches_exact("a"));
    println!("   matches_exact(\"a\"): false\n");

    // 2. is_match: Check if pattern appears anywhere in text
    println!("2. is_match(text) - Check if pattern appears anywhere:");
    assert!(pattern.is_match("ab"));
    println!("   is_match(\"ab\"): true");

    assert!(pattern.is_match("prefix_abb_suffix"));
    println!("   is_match(\"prefix_abb_suffix\"): true");

    assert!(!pattern.is_match("xyz"));
    println!("   is_match(\"xyz\"): false\n");

    // 3. find: Get first match position
    println!("3. find(text) - Get first match position (start, end):");
    let find_text = "xyzabbbxyz";
    match pattern.find(find_text) {
        Some((start, end)) => {
            let matched = &find_text[start..end];
            println!(
                "   find(\"{}\"): Some(({}, {})) -> \"{}\"",
                find_text, start, end, matched
            );
            assert_eq!(start, 3);
            // Note: The NFA matches greedily up to the found position
        }
        None => panic!("Should have found a match"),
    }

    match pattern.find("xyz") {
        Some(_) => panic!("Should not have found a match"),
        None => println!("   find(\"xyz\"): None\n"),
    }

    // 4. find_iter: Iterate over all matches
    println!("4. find_iter(text) - Iterator over all non-overlapping matches:");
    let text = "xabxabbxabbbx";
    let matches: Vec<(usize, usize)> = pattern.find_iter(text).collect();
    println!("   find_iter(\"{}\"): {:?}", text, matches);
    assert_eq!(matches.len(), 3);
    println!("   Found {} matches\n", matches.len());

    println!("All API methods work correctly!");
}
