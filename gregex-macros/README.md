# Gregex Macros

Procedural macros for compile-time regular expression parsing and NFA construction.

## Overview

`gregex-macros` provides the `regex!` macro that parses regex pattern strings at compile time and generates optimized NFA construction code. This eliminates runtime parsing overhead and enables compile-time validation of regex patterns.

## The `regex!` Macro

### Basic Usage

```rust,ignore
use gregex::regex;

let pattern = regex!("a+b*");
```

### Compile-Time Construction

The macro parses the regex string during compilation and directly embeds the resulting NFA data structure. For example, `regex!("abc")` expands to:

```rust,ignore
NFA::from_raw(
    vec![2, 3, 4],      // States
    vec![4],            // Accept states
    vec![               // Transitions
        ((0, 'a'), vec![2]),
        ((2, 'b'), vec![3]),
        ((3, 'c'), vec![4]),
    ]
)
```

This means zero runtime overhead for pattern compilation.

## Parser Implementation

The macro uses a Pratt parser (recursive descent with operator precedence) to handle regex syntax:

### Supported Syntax

- **Literals**: `a`, `b`, `c`, etc.
- **Concatenation**: `ab` (implicit)
- **Alternation**: `a|b` (OR operator)
- **Kleene Star**: `a*` (zero or more)
- **Plus**: `a+` (one or more)
- **Question**: `a?` (zero or one)
- **Grouping**: `(...)` for precedence control

### Operator Precedence

From highest to lowest:
1. Postfix operators: `*`, `+`, `?`
2. Concatenation (implicit)
3. Alternation: `|`

### Examples

```rust,ignore
regex!("(a|b)+")      // One or more of 'a' or 'b'
regex!("a+b?c*")      // At least one 'a', optional 'b', zero or more 'c'
regex!("(ab|cd)*")    // Zero or more repetitions of "ab" or "cd"
```

## Implementation Details

### Lexical Analysis

The parser tokenizes the input string into:
- Character literals
- Operators (`*`, `+`, `?`, `|`)
- Parentheses (`(`, `)`)
- End-of-file marker

### Syntax Tree Generation

Tokens are parsed into an abstract syntax tree (AST) using the `Node` type from `gregex-logic`:

```rust,ignore
pub enum Node {
    Terminal(char, u32),
    Operation(Operator, Box<Node>, Option<Box<Node>>),
}
```

### NFA Generation

The AST is processed using Glushkov's algorithm to compute:
1. **Nullability set**: Whether the pattern matches empty string
2. **Prefix set**: Initial characters that can start a match
3. **Suffix set**: Final characters that can end a match
4. **Factors set**: Valid character-to-character transitions

These sets are used to construct the final NFA.

## Error Handling

The parser provides compile-time error messages for invalid syntax:

```rust,ignore
regex!("(abc")   // Error: Unmatched parenthesis
regex!("a**")    // Error: Unexpected operator
```

## Performance Characteristics

- **Compile time**: O(n) where n is pattern length
- **Generated code**: Direct NFA data structure (no runtime parsing)
- **Type safety**: All errors caught at compile time

## Limitations

Current limitations of the parser:

- No escape sequences (e.g., `\n`, `\t`)
- No character classes (e.g., `[a-z]`, `\d`, `\w`)
- No wildcards (`.`)
- No quantifiers (e.g., `{n,m}`)
- No anchors (e.g., `^`, `$`)

These are planned for future versions.

## Integration with gregex-logic

This crate depends on `gregex-logic` for:
- AST node types (`Node`, `Operator`)
- Set computation functions (`nullability_set`, `prefix_set`, etc.)
- NFA construction logic

The macro acts as a compile-time bridge, converting string patterns into executable NFA data structures.

## License

MIT - See LICENSE file in the repository root.
