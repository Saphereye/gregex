# Gregex ![crates.io](https://img.shields.io/crates/v/gregex.svg) ![Build Passing](https://github.com/Saphereye/gregex/actions/workflows/ci.yml/badge.svg)

![](https://github.com/Saphereye/gregex/raw/master/assets/gregex_workflow.excalidraw.svg)

Gregex is a regular expression solver which utilizes Non-deterministic Finite Automata (NFA) to simulate the input strings using Glushkov's construction algorithm.

## Features

- **NFA-based matching**: Uses Glushkov's construction for efficient regex matching
- **Macro-based API**: Intuitive macro interface for building regex patterns
- **Multiple operators**: Support for concatenation, alternation, repetition (Kleene star, plus, question)
- **Type-safe**: Compile-time regex construction with Rust's procedural macros

## Installation

Add gregex to your `Cargo.toml`:

```toml
[dependencies]
gregex = "0.7.2"
```

## Supported Operators

| Operator | Macro | Description | Example | Matches |
|----------|-------|-------------|---------|---------|
| Concatenation | `dot!(...)` | Matches sequences | `dot!('a', 'b')` | "ab" |
| Alternation | `or!(...)` | Matches alternatives | `or!('a', 'b')` | "a" or "b" |
| Kleene Star | `star!(...)` | Zero or more | `star!('a')` | "", "a", "aa", ... |
| Plus | `plus!(...)` | One or more | `plus!('a')` | "a", "aa", "aaa", ... |
| Question | `question!(...)` | Zero or one | `question!('a')` | "" or "a" |

## Usage

### Basic Example

```rust
use gregex::*;

fn main() {
    // Match the pattern "ab"
    let runner = regex!(dot!('a', 'b'));
    assert_eq!(runner.run("ab"), true);
    assert_eq!(runner.run("ba"), false);
}
```

### Operators

#### Concatenation (`dot!`)

```rust
let runner = regex!(dot!('a', 'b', 'c'));
assert_eq!(runner.run("abc"), true);
```

#### Alternation (`or!`)

```rust
let runner = regex!(or!('a', 'b', 'c'));
assert_eq!(runner.run("a"), true);
assert_eq!(runner.run("b"), true);
assert_eq!(runner.run("ab"), false);
```

#### Kleene Star (`star!`) - Zero or More

```rust
let runner = regex!(star!('a'));
assert_eq!(runner.run(""), true);
assert_eq!(runner.run("a"), true);
assert_eq!(runner.run("aaa"), true);
```

#### Plus (`plus!`) - One or More

```rust
let runner = regex!(plus!('a'));
assert_eq!(runner.run("a"), true);
assert_eq!(runner.run("aa"), true);
assert_eq!(runner.run(""), false);  // Requires at least one
```

#### Question (`question!`) - Zero or One

```rust
let runner = regex!(question!('a'));
assert_eq!(runner.run(""), true);
assert_eq!(runner.run("a"), true);
assert_eq!(runner.run("aa"), false);  // At most one
```

### Complex Patterns

Operators can be nested and combined:

```rust
// Pattern: a+b? (one or more 'a' followed by optional 'b')
let runner = regex!(dot!(plus!('a'), question!('b')));
assert_eq!(runner.run("a"), true);
assert_eq!(runner.run("ab"), true);
assert_eq!(runner.run("aab"), true);
assert_eq!(runner.run("abb"), false);

// Pattern: (a|b)* (zero or more of 'a' or 'b')
let runner = regex!(star!(or!('a', 'b')));
assert_eq!(runner.run(""), true);
assert_eq!(runner.run("ab"), true);
assert_eq!(runner.run("baba"), true);
```

## Examples

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
