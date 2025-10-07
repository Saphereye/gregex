# Gregex ![crates.io](https://img.shields.io/crates/v/gregex.svg)

Gregex is a powerful regular expression library that compiles regex patterns to Non-deterministic Finite Automata (NFA) at compile-time using Glushkov's construction algorithm. Write regex patterns as strings and let Rust's procedural macros do the rest!

## Features

- **String-based regex parsing**: Write natural regex syntax like `regex!("(a|b)+")`
- **Compile-time construction**: Zero runtime regex parsing overhead
- **Type-safe**: Leverages Rust's procedural macros for safety
- **NFA-based matching**: Uses Glushkov's construction for efficient matching
- **Rich operator support**: `*`, `+`, `?`, `|`, concatenation, and grouping

## Quick Start

Add gregex to your `Cargo.toml`:

```bash
cargo add --git https://github.com/Saphereye/gregex
```

### Simple Example

```rust
use gregex::*;

fn main() {
    // Natural regex syntax - parsed at compile time!
    let pattern = regex!("(a|b)+c");
    
    // Use standard regex API methods
    assert!(pattern.is_match("abc"));      // Find pattern anywhere
    assert!(pattern.is_match("prefix_abc_suffix"));
    assert_eq!(pattern.find("xabcy"), Some((1, 4)));  // Get match position
}
```

## API Methods

Gregex provides a standard regex API similar to Rust's `regex` crate:

| Method | Description | Example |
|--------|-------------|---------|
| `is_match(text)` | Check if pattern exists in text | `pattern.is_match("hello")` |
| `find(text)` | Get first match position | `pattern.find("text")` → `Some((start, end))` |
| `find_iter(text)` | Iterator over all matches | `pattern.find_iter("text").collect()` |

## Regex Syntax Reference

When using string-based syntax with `regex!("...")`, the following operators are supported:

| Syntax | Description | Example | Matches |
|--------|-------------|---------|---------|
| `a`, `b`, `c` | Literal characters | `regex!("abc")` | "abc" |
| `ab` | Concatenation (implicit) | `regex!("hello")` | "hello" |
| `a\|b` | Alternation (OR) | `regex!("a\|b")` | "a" or "b" |
| `a*` | Kleene star (zero or more) | `regex!("a*")` | "", "a", "aa", ... |
| `a+` | Plus (one or more) | `regex!("a+")` | "a", "aa", "aaa", ... |
| `a?` | Question (zero or one) | `regex!("a?")` | "" or "a" |
| `(...)` | Grouping for precedence | `regex!("(ab)+")` | "ab", "abab", ... |

### Wildcard Patterns

**Note**: The `.` wildcard (match any character) and `.*` patterns are not currently supported in the parser. However:
- Use `(a|b|c)*` to match specific character sets with repetition
- Use alternation `(a|b|c)+` for one-or-more of specific characters  
- The `is_match()` method finds patterns anywhere in text, so `pattern.is_match()` behaves similarly to `.*pattern.*` in standard regex

**Future Enhancement**: Full wildcard support (`.` and `\w`, `\d`, etc.) is planned for a future version.


## Usage Examples

### 1. String-Based Syntax (Recommended)

The most natural and recommended way to use Gregex:

```rust
use gregex::*;

// Simple patterns with new API
let pattern = regex!("a+@b+");
assert!(pattern.is_match("aaa@bbb"));
assert!(pattern.is_match("prefix_aa@bb_suffix"));

// Complex patterns with operators
let identifier = regex!("(a|b)(a|b|c)*");
assert!(identifier.is_match("abc"));
assert!(identifier.is_match("bca"));

// Find match positions
let pattern = regex!("a+b?c*");
if let Some((start, end)) = pattern.find("xyzaabccxyz") {
    println!("Found match from {} to {}", start, end);
}

// Nested grouping
let nested = regex!("((a|b)+c)*");
assert!(nested.is_match("acbc"));
```

## Examples

Run the included examples to see gregex in action:

### Basic Operator Examples

These examples demonstrate individual regex operators:

```bash
# Basic concatenation (matching "abc")
cargo run --example 01_basic_concatenation

# Alternation/OR operator (a|b|c)
cargo run --example 02_alternation

# Kleene star - zero or more (a*)
cargo run --example 03_kleene_star

# Plus operator - one or more (a+)
cargo run --example 04_plus_operator

# Question operator - zero or one (a?)
cargo run --example 05_question_operator

# Grouping and operator precedence
cargo run --example 06_grouping_and_precedence
```

### Advanced Examples

```bash
# Complete API methods demonstration
cargo run --example 07_api_methods

# Compile-time NFA construction verification
cargo run --example 08_compile_time_construction
```

### Use Case Examples

Real-world applications demonstrating practical pattern matching:

```bash
# Validate programming identifiers
cargo run --example usecase_identifier_validator

# Match URL-like paths
cargo run --example usecase_simple_url_matcher

# Search for patterns in text
cargo run --example usecase_text_search
```

## How It Works

Gregex uses Glushkov's construction algorithm to convert regular expressions into NFAs:

1. **Linearization**: Each symbol in the regex is assigned a unique index
2. **Set Construction**: Computes prefix, suffix, factors, and nullability sets
3. **NFA Generation**: Constructs the NFA based on these sets
4. **Simulation**: Runs the input string through the NFA to determine if it matches

This approach generates NFAs with states equal to the number of terminals plus one, making it efficient for pattern matching.

## Testing

Run the comprehensive test suite:

```bash
cargo test --all
```

## License

MIT License - see [LICENSE](LICENSE) for details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
