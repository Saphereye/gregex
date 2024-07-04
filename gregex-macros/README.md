# Gregex Macros
Contains the macro interface for all the gregex function.

Without these, users would have to rely on function that generate the Node tree. To explain this we can first look at an example.

Let's take the regex `a*`.

The Node tree in our case would be,
```rust
Node::Operation(
    Operator::Production,
    Box::new(Node::Terminal('a', 0u32)),
    None,
)
```

Although we can wrap this in a function or a `macro_rules!` macro, the generated code is quite bloated. We can do the hard work during compilation, i.e. converting our regex to the end NFA.

Currently converting to NFA is not possible, but this crate can convert it to the interstitial form of the Node Tree.