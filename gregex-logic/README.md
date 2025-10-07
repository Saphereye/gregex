# Gregex Logic

Core logic library for the Gregex regular expression engine.

## Overview

`gregex-logic` implements the fundamental algorithms and data structures for regular expression matching using Non-deterministic Finite Automata (NFA). This crate provides the runtime engine that powers the `gregex` library's compile-time regex capabilities.

## Architecture

### Glushkov's Construction Algorithm

This library uses [Glushkov's construction algorithm](https://en.wikipedia.org/wiki/Glushkov%27s_construction_algorithm) to convert regular expressions into NFAs. The advantage over Thompson's construction is that the generated NFA has exactly `n+1` states for a regex with `n` terminals, making it more compact and efficient.

### Key Components

1. **NFA Module** (`nfa.rs`)
   - Non-deterministic Finite Automaton implementation
   - State transitions and acceptance logic
   - Matching algorithms for substring and exact matching
   - Iterator support for finding all matches

2. **Translation Module** (`translation/`)
   - **Node** (`node.rs`): Abstract syntax tree representation
   - **Operator** (`operator.rs`): Regex operator definitions
   - **SetTerminal** (`setterminal.rs`): Terminal symbol handling
   - Set computation functions: nullability, prefix, suffix, and factors

## Supported Operators

- **Concatenation**: Implicit sequencing of characters
- **Alternation** (`|`): Match either left or right expression
- **Kleene Star** (`*`): Zero or more repetitions
- **Plus** (`+`): One or more repetitions
- **Question** (`?`): Zero or one occurrence

## API Methods

The NFA struct provides several matching methods:

- `matches_exact(text)`: Check if entire text matches the pattern
- `is_match(text)`: Check if pattern appears anywhere in text
- `find(text)`: Find first match position
- `find_iter(text)`: Iterator over all non-overlapping matches

## Usage

This crate is designed to be used through the `gregex` main crate, which provides the `regex!` macro for compile-time pattern compilation. Direct usage of `gregex-logic` is possible but requires manual NFA construction:

```rust,ignore
use gregex_logic::nfa::NFA;

// Manual NFA construction
let mut nfa = NFA::new();
nfa.add_state(1);
nfa.add_accept_state(1);
nfa.add_transition(0, 'a', 1);

assert!(nfa.matches_exact("a"));
```

## Performance

- **Compile-time construction**: When used through `gregex`, NFAs are built at compile time
- **Linear matching**: O(n*m) time complexity where n is text length and m is NFA states
- **No backtracking**: NFA-based approach avoids exponential backtracking issues

## Future Enhancements

- Capture group support
- Wildcard patterns (`.`, `\w`, `\d`, etc.)
- NFA optimization and minimization
- Unicode support improvements

## License

MIT - See LICENSE file in the repository root.
