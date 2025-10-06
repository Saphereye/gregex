//! Has the implementation of a non-deterministic finite automaton (NFA).

use crate::translation::setterminal::SetTerminal;
use core::panic;
use std::collections::{HashMap, HashSet};

/// The `NFA` struct represents a non-deterministic finite automaton.
#[derive(Debug, Default)]
pub struct NFA {
    /// Set of all possible states of the NFA.
    states: HashSet<u32>,
    /// Set of all accepting states. If the NFA ends at any one if these the simulation is succesful.
    accept: HashSet<u32>,
    /// The transition function is a map from a pair of a state and a character to a set of states.
    transition_function: HashMap<(u32, char), HashSet<u32>>,
}

impl NFA {
    /// Simulates the NFA with the given input.
    pub fn run(&self, input: &str) -> bool {
        let mut current_states = HashSet::new();
        current_states.insert(0);
        for c in input.chars() {
            let mut next_states = HashSet::new();
            for state in current_states {
                if let Some(states) = self.transition_function.get(&(state, c)) {
                    next_states.extend(states);
                }
            }
            current_states = next_states;
        }
        !current_states.is_disjoint(&self.accept)
    }

    /// Converts the prefix, suffix and factors sets to a NFA.
    pub fn set_to_nfa(
        prefix_set: &HashSet<SetTerminal>,
        suffix_set: &HashSet<SetTerminal>,
        factors_set: &HashSet<SetTerminal>,
        nullability_set: &HashSet<SetTerminal>,
    ) -> Self {
        let mut nfa = Self::default();

        // If the regex is nullable (accepts empty string), add initial state to accept states
        if nullability_set.contains(&SetTerminal::Epsilon) {
            nfa.accept.insert(0);
        }

        for i in prefix_set {
            match *i {
                SetTerminal::SingleElement(symbol, index) => {
                    nfa.states.insert(index);
                    nfa.transition_function
                        .insert((0, symbol), vec![index].into_iter().collect());
                }
                SetTerminal::DoubleElement(_, _, _, _) => {
                    panic!("DoubleElement not supported")
                }
                _ => {}
            }
        }

        for i in suffix_set {
            match *i {
                SetTerminal::SingleElement(_, index) => {
                    nfa.states.insert(index);
                    nfa.accept.insert(index);
                }
                SetTerminal::DoubleElement(_, _, _, _) => {
                    panic!("DoubleElement not supported")
                }
                _ => {}
            }
        }

        for i in factors_set {
            match *i {
                SetTerminal::DoubleElement(_, index1, symbol2, index2) => {
                    nfa.states.insert(index1);
                    nfa.states.insert(index2);
                    nfa.transition_function
                        .entry((index1, symbol2))
                        .or_insert_with(HashSet::new)
                        .insert(index2);
                }
                SetTerminal::SingleElement(_, _) => {
                    panic!("SingleElement not supported")
                }
                _ => {}
            }
        }

        nfa
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simulate_nfa_simple_test() {
        let nfa = NFA {
            states: vec![0, 1, 2].into_iter().collect(),
            accept: vec![2].into_iter().collect(),
            transition_function: vec![
                ((0, 'a'), vec![0, 1].into_iter().collect()),
                ((1, 'b'), vec![2].into_iter().collect()),
            ]
            .into_iter()
            .collect(),
        };
        assert!(nfa.run("ab"));
    }

    #[test]
    fn set_to_nfa_simple_test() {
        use crate::translation::setterminal::SetTerminal;
        let prefix_set = vec![SetTerminal::SingleElement('a', 1)]
            .into_iter()
            .collect();
        let suffix_set = vec![SetTerminal::SingleElement('b', 2)]
            .into_iter()
            .collect();
        let factors_set = vec![SetTerminal::DoubleElement('a', 1, 'b', 2)]
            .into_iter()
            .collect();
        let nullability_set = vec![SetTerminal::Empty].into_iter().collect();
        let nfa = NFA::set_to_nfa(&prefix_set, &suffix_set, &factors_set, &nullability_set);
        assert!(nfa.run("ab"));
    }

    #[test]
    fn set_to_nfa_plus_test() {
        // Test for a+ (one or more 'a')
        use crate::translation::setterminal::SetTerminal;
        let prefix_set = vec![SetTerminal::SingleElement('a', 1)]
            .into_iter()
            .collect();
        let suffix_set = vec![SetTerminal::SingleElement('a', 1)]
            .into_iter()
            .collect();
        let factors_set = vec![SetTerminal::DoubleElement('a', 1, 'a', 1)]
            .into_iter()
            .collect();
        let nullability_set = vec![SetTerminal::Empty].into_iter().collect();
        let nfa = NFA::set_to_nfa(&prefix_set, &suffix_set, &factors_set, &nullability_set);

        assert!(nfa.run("a"));
        assert!(nfa.run("aa"));
        assert!(nfa.run("aaa"));
        assert!(!nfa.run(""));
        assert!(!nfa.run("b"));
    }

    #[test]
    fn set_to_nfa_question_test() {
        // Test for a? (zero or one 'a')
        // Question operator should match empty string (epsilon in suffix)
        use crate::translation::node::{
            factors_set, nullability_set, prefix_set, suffix_set, Node,
        };
        use crate::translation::operator::Operator;

        let tree = Node::Operation(Operator::Question, Box::new(Node::Terminal('a', 1)), None);
        let prefix = prefix_set(&tree);
        let suffix = suffix_set(&tree);
        let factors = factors_set(&tree);
        let nullability = nullability_set(&tree);

        let nfa = NFA::set_to_nfa(&prefix, &suffix, &factors, &nullability);

        // For a?, we expect to match 'a' and empty string
        assert!(nfa.run("a"));
        assert!(nfa.run(""));
        assert!(!nfa.run("aa"));
    }

    #[test]
    fn set_to_nfa_plus_complex_test() {
        // Test for (ab)+ pattern
        use crate::translation::node::{
            factors_set, nullability_set, prefix_set, suffix_set, Node,
        };
        use crate::translation::operator::Operator;

        let tree = Node::Operation(
            Operator::Plus,
            Box::new(Node::Operation(
                Operator::Concat,
                Box::new(Node::Terminal('a', 1)),
                Some(Box::new(Node::Terminal('b', 2))),
            )),
            None,
        );

        let prefix = prefix_set(&tree);
        let suffix = suffix_set(&tree);
        let factors = factors_set(&tree);
        let nullability = nullability_set(&tree);

        let nfa = NFA::set_to_nfa(&prefix, &suffix, &factors, &nullability);

        assert!(nfa.run("ab"));
        assert!(nfa.run("abab"));
        assert!(nfa.run("ababab"));
        assert!(!nfa.run(""));
        assert!(!nfa.run("a"));
        assert!(!nfa.run("b"));
        assert!(!nfa.run("ba"));
    }

    #[test]
    fn test_operator_combinations_plus_question() {
        // Test a+b? (one or more 'a' followed by zero or one 'b')
        use crate::translation::node::{
            factors_set, nullability_set, prefix_set, suffix_set, Node,
        };
        use crate::translation::operator::Operator;

        let tree = Node::Operation(
            Operator::Concat,
            Box::new(Node::Operation(
                Operator::Plus,
                Box::new(Node::Terminal('a', 1)),
                None,
            )),
            Some(Box::new(Node::Operation(
                Operator::Question,
                Box::new(Node::Terminal('b', 2)),
                None,
            ))),
        );

        let prefix = prefix_set(&tree);
        let suffix = suffix_set(&tree);
        let factors = factors_set(&tree);
        let nullability = nullability_set(&tree);

        let nfa = NFA::set_to_nfa(&prefix, &suffix, &factors, &nullability);

        assert!(nfa.run("a"));
        assert!(nfa.run("ab"));
        assert!(nfa.run("aa"));
        assert!(nfa.run("aab"));
        assert!(!nfa.run("abb"));
        assert!(!nfa.run(""));
        assert!(!nfa.run("b"));
    }

    #[test]
    fn test_operator_combinations_star_plus() {
        // Test a*b+ (zero or more 'a' followed by one or more 'b')
        use crate::translation::node::{
            factors_set, nullability_set, prefix_set, suffix_set, Node,
        };
        use crate::translation::operator::Operator;

        let tree = Node::Operation(
            Operator::Concat,
            Box::new(Node::Operation(
                Operator::Production,
                Box::new(Node::Terminal('a', 1)),
                None,
            )),
            Some(Box::new(Node::Operation(
                Operator::Plus,
                Box::new(Node::Terminal('b', 2)),
                None,
            ))),
        );

        let prefix = prefix_set(&tree);
        let suffix = suffix_set(&tree);
        let factors = factors_set(&tree);
        let nullability = nullability_set(&tree);

        let nfa = NFA::set_to_nfa(&prefix, &suffix, &factors, &nullability);

        assert!(nfa.run("b"));
        assert!(nfa.run("ab"));
        assert!(nfa.run("aab"));
        assert!(nfa.run("bb"));
        assert!(nfa.run("abb"));
        assert!(!nfa.run(""));
        assert!(!nfa.run("a"));
        assert!(!nfa.run("aa"));
    }

    #[test]
    fn test_operator_combinations_question_star() {
        // Test a?b* (zero or one 'a' followed by zero or more 'b')
        use crate::translation::node::{
            factors_set, nullability_set, prefix_set, suffix_set, Node,
        };
        use crate::translation::operator::Operator;

        let tree = Node::Operation(
            Operator::Concat,
            Box::new(Node::Operation(
                Operator::Question,
                Box::new(Node::Terminal('a', 1)),
                None,
            )),
            Some(Box::new(Node::Operation(
                Operator::Production,
                Box::new(Node::Terminal('b', 2)),
                None,
            ))),
        );

        let prefix = prefix_set(&tree);
        let suffix = suffix_set(&tree);
        let factors = factors_set(&tree);
        let nullability = nullability_set(&tree);

        let nfa = NFA::set_to_nfa(&prefix, &suffix, &factors, &nullability);

        assert!(nfa.run(""));
        assert!(nfa.run("a"));
        assert!(nfa.run("b"));
        assert!(nfa.run("ab"));
        assert!(nfa.run("abb"));
        assert!(nfa.run("bb"));
        assert!(!nfa.run("aa"));
        assert!(!nfa.run("aab"));
    }

    #[test]
    fn test_or_with_plus_and_question() {
        // Test a+|b? (one or more 'a' OR zero or one 'b')
        use crate::translation::node::{
            factors_set, nullability_set, prefix_set, suffix_set, Node,
        };
        use crate::translation::operator::Operator;

        let tree = Node::Operation(
            Operator::Or,
            Box::new(Node::Operation(
                Operator::Plus,
                Box::new(Node::Terminal('a', 1)),
                None,
            )),
            Some(Box::new(Node::Operation(
                Operator::Question,
                Box::new(Node::Terminal('b', 2)),
                None,
            ))),
        );

        let prefix = prefix_set(&tree);
        let suffix = suffix_set(&tree);
        let factors = factors_set(&tree);
        let nullability = nullability_set(&tree);

        let nfa = NFA::set_to_nfa(&prefix, &suffix, &factors, &nullability);

        assert!(nfa.run(""));
        assert!(nfa.run("a"));
        assert!(nfa.run("aa"));
        assert!(nfa.run("b"));
        assert!(!nfa.run("ab"));
        assert!(!nfa.run("bb"));
    }

    #[test]
    fn test_nested_operators() {
        // Test (a+)* (zero or more of one-or-more 'a')
        use crate::translation::node::{
            factors_set, nullability_set, prefix_set, suffix_set, Node,
        };
        use crate::translation::operator::Operator;

        let tree = Node::Operation(
            Operator::Production,
            Box::new(Node::Operation(
                Operator::Plus,
                Box::new(Node::Terminal('a', 1)),
                None,
            )),
            None,
        );

        let prefix = prefix_set(&tree);
        let suffix = suffix_set(&tree);
        let factors = factors_set(&tree);
        let nullability = nullability_set(&tree);

        let nfa = NFA::set_to_nfa(&prefix, &suffix, &factors, &nullability);

        assert!(nfa.run(""));
        assert!(nfa.run("a"));
        assert!(nfa.run("aa"));
        assert!(nfa.run("aaa"));
        assert!(!nfa.run("b"));
    }

    #[test]
    fn test_complex_combination() {
        // Test (a|b)+c? (one or more of 'a' or 'b', followed by zero or one 'c')
        use crate::translation::node::{
            factors_set, nullability_set, prefix_set, suffix_set, Node,
        };
        use crate::translation::operator::Operator;

        let tree = Node::Operation(
            Operator::Concat,
            Box::new(Node::Operation(
                Operator::Plus,
                Box::new(Node::Operation(
                    Operator::Or,
                    Box::new(Node::Terminal('a', 1)),
                    Some(Box::new(Node::Terminal('b', 2))),
                )),
                None,
            )),
            Some(Box::new(Node::Operation(
                Operator::Question,
                Box::new(Node::Terminal('c', 3)),
                None,
            ))),
        );

        let prefix = prefix_set(&tree);
        let suffix = suffix_set(&tree);
        let factors = factors_set(&tree);
        let nullability = nullability_set(&tree);

        let nfa = NFA::set_to_nfa(&prefix, &suffix, &factors, &nullability);

        assert!(nfa.run("a"));
        assert!(nfa.run("b"));
        assert!(nfa.run("ac"));
        assert!(nfa.run("bc"));
        assert!(nfa.run("abc"));
        assert!(nfa.run("aac"));
        assert!(!nfa.run(""));
        assert!(!nfa.run("c"));
        assert!(!nfa.run("acc"));
    }
}
