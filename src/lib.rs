//! This crate provides a [regular expression](https://en.wikipedia.org/wiki/Regular_expression) engine that uses a [Nondeterministic finite automaton](https://en.wikipedia.org/wiki/Nondeterministic_finite_automaton) to simulate the regular expression.
//!  Here is a short example on how to use this crate
//!
//! ```rust
//! extern crate gregex;
//! use gregex::*;
//!
//! fn main() {
//!     let tree = dot!(star!(term('a')), term('b'), term('c'));
//!     let regex = regex(&tree);
//!     assert!(regex.simulate("abc"));
//!     assert!(!regex.simulate("a"));
//!     assert!(regex.simulate("aaabc"));
//! }
//! ```
//!
//! The regex function uses the regular expression string to create a NFA that can be used to simulate the regular expression.
//! The program uses the [Glushkov's construction algorithm](https://en.wikipedia.org/wiki/Glushkov%27s_construction_algorithm) to create its NFA.
//! The NFA is then later simulated to check if the input string matches the regular expression.
//!
//! A brief overview of the pipeline:
//! [![](https://github.com/Saphereye/gregex/blob/master/assets/gregex_workflow.excalidraw.svg)
//! 

pub mod nfa;
pub mod translation;

use nfa::*;
use std::sync::atomic::{AtomicU32, Ordering};
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
static TERMINAL_COUNT: AtomicU32 = AtomicU32::new(0);

/// Represents the `concatenation` action in regex. Can dot multiple nodes.
/// 
/// Regex: ab
/// Gregex: dot!(term('a'), term('b'))
#[macro_export]
macro_rules! dot {
    ($($node:expr),+ $(,)?) => {
        {
            let nodes = vec![$($node),+];
            nodes.into_iter().reduce(|left, right| {
                $crate::translation::node::Node::Operation($crate::translation::operator::Operator::Concat, Box::new(left), Some(Box::new(right)))
            }).expect("Cannot dot an empty Vec<Node>")
        }
    };
}

/// Represents a `term` in regex. This is a single character.
/// 
/// Regex: a
/// Gregex: term('a')
pub fn term(symbol: char) -> Node {
    let count = TERMINAL_COUNT.fetch_add(1, Ordering::SeqCst);
    Node::Terminal(symbol, count)
}

/// Represents the `or`` action in regex. Can 'or' multiple nodes.
/// 
/// Regex: a|b
/// Gregex: or!(term('a'), term('b'))
#[macro_export]
macro_rules! or {
    ($($node:expr),+ $(,)?) => {
        {
            let nodes = vec![$($node),+];
            nodes.into_iter().reduce(|left, right| {
                $crate::translation::node::Node::Operation($crate::translation::operator::Operator::Or, Box::new(left), Some(Box::new(right)))
            }).expect("Cannot or an empty Vec<Node>")
        }
    };
}

/// Represents the `star` action in regex. This is a single node.
/// 
/// Regex: a*
/// Gregex: star!(term('a'))
#[macro_export]
macro_rules! star {
    ($child:expr) => {
        $crate::translation::node::Node::Operation(
            $crate::translation::operator::Operator::Production,
            Box::new($child),
            None,
        )
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_regex() {
        let tree = dot!(star!(term('a')), term('b'), term('c'));
        let regex = regex(&tree);
        assert!(regex.simulate("abc"));
        assert!(!regex.simulate("a"));
        assert!(regex.simulate("aaabc"));
    }
}
