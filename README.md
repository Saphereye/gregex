# Gregex ![crates.io](https://img.shields.io/crates/v/gregex.svg) ![Build Passing](https://github.com/Saphereye/gregex/actions/workflows/ci.yml/badge.svg)

![](https://github.com/Saphereye/gregex/raw/master/assets/gregex_workflow.excalidraw.svg)

Gregex is a powerful regular expression library that compiles regex patterns to Non-deterministic Finite Automata (NFA) at compile-time using Glushkov's construction algorithm. Write regex patterns as strings and let Rust's procedural macros do the rest!

## ✨ Features

- 🎯 **String-based regex parsing**: Write natural regex syntax like `regex!("(a|b)+")`
- ⚡ **Compile-time construction**: Zero runtime regex parsing overhead
- 🔒 **Type-safe**: Leverages Rust's procedural macros for safety
- 🧩 **NFA-based matching**: Uses Glushkov's construction for efficient matching
- 📦 **Multiple API styles**: String parsing, operator macros, or character literals
- 🎨 **Rich operator support**: `*`, `+`, `?`, `|`, concatenation, and grouping

## 🚀 Quick Start

Add gregex to your `Cargo.toml`:

```toml
[dependencies]
gregex = "0.8.0"
```

### Simple Example (Recommended: String Syntax)

```rust
use gregex::*;

fn main() {
    // Natural regex syntax - parsed at compile time!
    let runner = regex!("(a|b)+c");
    
    assert_eq!(runner.run("abc"), true);
    assert_eq!(runner.run("bbbac"), true);
    assert_eq!(runner.run("c"), false);
}
```

## 📖 Regex Syntax Reference

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

## 💡 Usage Examples

### 1. String-Based Syntax (Recommended)

The most natural and recommended way to use Gregex:

```rust
use gregex::*;

// Simple patterns
let email_checker = regex!("a+@b+");
assert_eq!(email_checker.run("user@domain"), true);

// Complex patterns with operators
let identifier = regex!("(a|b)(a|b|c)*");
assert_eq!(identifier.run("abc"), true);
assert_eq!(identifier.run("bca"), true);

// Multiple operators combined
let pattern = regex!("a+b?c*");
assert_eq!(pattern.run("aabcc"), true);
assert_eq!(pattern.run("a"), true);

// Nested grouping
let nested = regex!("((a|b)+c)*");
assert_eq!(nested.run("acbc"), true);
```

### 2. Operator Macros (Alternative API)

Use explicit operator macros for more control:

```rust
use gregex::*;

// Concatenation with strings
let runner = regex!(dot!("hello", " ", "world"));
assert_eq!(runner.run("hello world"), true);

// Operators work with strings too
let runner = regex!(star!("ab"));
assert_eq!(runner.run("ababab"), true);

let runner = regex!(plus!("hello"));
assert_eq!(runner.run("hellohello"), true);
```

### 3. Combining Operators

Both string syntax and macros can be mixed and nested:

```rust
use gregex::*;

// Nested macros
let runner = regex!(dot!(plus!('a'), question!('b')));
assert_eq!(runner.run("aab"), true);

// String syntax is usually clearer for the same pattern
let runner = regex!("a+b?");
assert_eq!(runner.run("aab"), true);
```

## 📦 Examples

Run the included examples to see gregex in action:

```bash
# Basic concatenation
cargo run --example dot

# Alternation (OR)
cargo run --example or

# Kleene star (zero or more)
cargo run --example star

# Plus operator (one or more)
cargo run --example plus

# Question operator (zero or one)
cargo run --example question

# Real-world pattern matching
cargo run --example real_world_patterns

# String literal support in macros
cargo run --example string_support

# Regex string parsing
cargo run --example regex_string_parsing
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
