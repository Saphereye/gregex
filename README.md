# Gregex ![crates.io](https://img.shields.io/crates/v/gregex.svg) ![Build Passing](https://github.com/Saphereye/gregex/actions/workflows/ci.yml/badge.svg)

Gregex is a regular expression solver which utilizes Non-deterministic Finite Automata (NFA) to simulate the input strings.

## Usage

```rust
extern crate gregex;
use gregex::*;
fn main() {
    let tree = dot!(star!('a'), 'b', 'c');
    let regex = regex(&tree);
    assert!(regex.simulate("abc"));
    assert!(!regex.simulate("a"));
    assert!(regex.simulate("aaabc"));
}
```

## Theory
The project uses [Glushkov's construction algorithm](https://en.wikipedia.org/wiki/Glushkov%27s_construction_algorithm) for creating the NFA.

The pipeline can be summarised as below
![](https://github.com/Saphereye/gregex/blob/master/assets/gregex_workflow.excalidraw.svg)