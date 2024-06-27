//! This crate provides a [regular expression](https://en.wikipedia.org/wiki/Regular_expression) engine that uses a [Nondeterministic finite automaton](https://en.wikipedia.org/wiki/Nondeterministic_finite_automaton) to simulate the regular expression.
//!  Here is a short example on how to use this crate
//!
//! ```rust
//! extern crate gregex;
//! use gregex::*;
//!
//! fn main() {
//!     let tree = dot!(star!('a'), 'b', 'c');
//!     let regex = regex(&tree);
//!     assert!(regex.run("abc"));
//!     assert!(!regex.run("a"));
//!     assert!(regex.run("aaabc"));
//! }
//! ```
//!
//! The regex function uses the regular expression string to create a NFA that can be used to simulate the regular expression.
//! The program uses the [Glushkov's construction algorithm](https://en.wikipedia.org/wiki/Glushkov%27s_construction_algorithm) to create its NFA.
//! The NFA is then later simulated to check if the input string matches the regular expression.
//!
//! A brief overview of the pipeline:
//! [![](https://github.com/Saphereye/gregex/blob/master/assets/gregex_workflow.excalidraw.svg)]
//!

pub mod nfa;
pub mod translation;

use nfa::*;
use std::sync::atomic::AtomicU32;
use translation::node::*;

type Regex = NFA;

/// Translates a regular expression tree to a NFA. This NFA can then be called to simulate inputs.
pub fn regex(regex_tree: &Node) -> Regex {
    let prefix_set = &prefix_set(regex_tree);
    let suffix_set = &suffix_set(regex_tree);
    let factors_set = &factors_set(regex_tree);
    NFA::set_to_nfa(prefix_set, suffix_set, factors_set)
}

/// Keeps count of the terminals created. This is used to create unique terminals.
pub static TERMINAL_COUNT: AtomicU32 = AtomicU32::new(0);

/// Represents the `concatenation` action in regex. Can dot multiple nodes.
///
/// Regex: `ab`
/// Gregex: `dot!('a', 'b')`
#[macro_export]
macro_rules! dot {
    ($($node:expr),+ $(,)?) => {
        {
            let nodes = vec![$(helper!($node)),+];
            nodes.into_iter().reduce(|left, right| {
                $crate::translation::node::Node::Operation($crate::translation::operator::Operator::Concat, Box::new(left), Some(Box::new(right)))
            }).expect("The input is empty")
        }
    };
}

/// Represents the `or` action in regex. Can 'or' multiple nodes.
///
/// Regex: `a|b`
/// Gregex: `or!('a', 'b')`
#[macro_export]
macro_rules! or {
    ($($node:expr),+ $(,)?) => {
        {
            let nodes = vec![$(helper!($node)),+];
            nodes.into_iter().reduce(|left, right| {
                $crate::translation::node::Node::Operation($crate::translation::operator::Operator::Or, Box::new(left), Some(Box::new(right)))
            }).expect("The input is empty")
        }
    };
}

/// Helper function to handle literals and expressions inside the [or], [star] and [dot].
#[macro_export]
macro_rules! helper {
    ($node:literal) => {{
        {
            let count = $crate::TERMINAL_COUNT.fetch_add(1, core::sync::atomic::Ordering::SeqCst);
            $crate::translation::node::Node::Terminal($node, count)
        }
    }};
    ($node:expr) => {
        $node
    };
}

/// Represents the `production` action in regex. This is a single node.
///
/// Regex: `a*`
/// Gregex: `star!('a')`
#[macro_export]
macro_rules! star {
    ($child:expr) => {
        $crate::translation::node::Node::Operation(
            $crate::translation::operator::Operator::Production,
            Box::new(helper!($child)),
            None,
        )
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_regex() {
        let tree = dot!(star!('a'), 'b', 'c');
        let regex = regex(&tree);
        assert!(regex.run("abc"));
        assert!(!regex.run("a"));
        assert!(regex.run("aaabc"));
    }
}
