extern crate gregex;
use gregex::*;

fn main() {
    println!("=== Use Case: Text Search ===\n");
    println!("This example demonstrates finding patterns in text documents.\n");

    let pattern = regex!("(a|b)+c");
    println!("Pattern: \"(a|b)+c\"");
    println!("Meaning: One or more 'a' or 'b', followed by 'c'\n");

    // Example 1: Finding pattern in text
    println!("Example 1: Finding single occurrence");
    let text1 = "The pattern abc appears here";
    println!("Text: \"{}\"", text1);

    if let Some((start, end)) = pattern.find(text1) {
        let matched = &text1[start..end];
        println!("Found: \"{}\" at position {}-{}", matched, start, end);
        assert_eq!(matched, "abc");
    } else {
        println!("No match found");
    }

    // Example 2: Finding multiple occurrences
    println!("\nExample 2: Finding multiple occurrences");
    let text2 = "Patterns: abc, bac, aabc, and bbbac appear here";
    println!("Text: \"{}\"", text2);
    println!("Matches found:");

    let matches: Vec<(usize, usize)> = pattern.find_iter(text2).collect();
    for (start, end) in &matches {
        let matched = &text2[*start..*end];
        println!("  \"{}\" at position {}-{}", matched, start, end);
    }
    assert_eq!(matches.len(), 4);
    println!("Total matches: {}", matches.len());

    // Example 3: Checking if pattern exists anywhere
    println!("\nExample 3: Quick existence check");
    let test_cases = vec![
        ("This has abc in it", true),
        ("This has bbbac too", true),
        ("This has abd but not our pattern", false),
        ("No match here", false),
    ];

    for (text, expected) in test_cases {
        let found = pattern.is_match(text);
        let status = if found == expected { "PASS" } else { "FAIL" };
        println!("[{}] \"{}\" -> {}", status, text, found);
        assert_eq!(found, expected);
    }

    println!("\nAll text search tests passed!");
}
