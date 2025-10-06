extern crate gregex;
use gregex::*;

fn main() {
    println!("=== Testing New API Methods ===\n");

    // Test is_match (replaces run)
    println!("1. is_match() - Find pattern anywhere in text:");
    let pattern = regex!("abc");
    assert!(pattern.is_match("abc"));
    assert!(pattern.is_match("xabcy")); // Matches in middle
    assert!(pattern.is_match("___abc")); // Matches at end
    assert!(!pattern.is_match("xyz"));
    println!("✓ is_match works\n");

    // Test find
    println!("2. find() - Get first match position:");
    let pattern = regex!("ab");
    assert_eq!(pattern.find("xabcy"), Some((1, 3)));
    assert_eq!(pattern.find("ab"), Some((0, 2)));
    assert_eq!(pattern.find("xyz"), None);
    println!("✓ find works\n");

    // Test find_iter
    println!("3. find_iter() - Find all matches:");
    let pattern = regex!("ab");
    let matches: Vec<_> = pattern.find_iter("abxabxab").collect();
    println!("   Matches in 'abxabxab': {:?}", matches);
    assert_eq!(matches.len(), 3);
    println!("✓ find_iter works\n");

    // Test .* pattern (any character, zero or more times)
    println!("4. Testing .* patterns:");
    // Note: Current parser doesn't support '.' as wildcard
    // But we can use star on characters
    let any_a = regex!("a*"); // Zero or more 'a'
    assert!(any_a.is_match(""));
    assert!(any_a.is_match("aaa"));
    assert!(any_a.is_match("bbb")); // Matches empty string at start
    println!("✓ Star patterns work (matches zero or more)\n");

    // Test captures (placeholder)
    println!("5. captures() - Currently not implemented:");
    let pattern = regex!("(a+)");
    assert_eq!(pattern.captures("aaa"), None);
    println!("✓ Returns None as expected (future feature)\n");

    // Complex pattern matching
    println!("6. Complex patterns:");
    let email_like = regex!("a+@b+");
    assert!(email_like.is_match("a@b"));
    assert!(email_like.is_match("aaa@bbb"));
    assert!(email_like.is_match("prefix_aaa@bbb_suffix"));
    println!("✓ Complex patterns work with is_match\n");

    println!("🎉 All new API methods work correctly!");
    println!("\nNote: Use `is_match()` instead of deprecated `run()` method.");
}
