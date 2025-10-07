//! Has the implementation of a non-deterministic finite automaton (NFA).

use crate::translation::setterminal::SetTerminal;
use core::panic;
use std::collections::{HashMap, HashSet};

/// Iterator over non-overlapping matches in a text.
pub struct FindIter<'t> {
    nfa: &'t NFA,
    text: &'t str,
    pos: usize,
}

impl<'t> Iterator for FindIter<'t> {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos > self.text.len() {
            return None;
        }

        // Try to find a match starting from current position or later
        for start in self.pos..=self.text.len() {
            // Try different lengths for a match
            for end in start..=self.text.len() {
                if self.nfa.matches_exact(&self.text[start..end]) {
                    self.pos = end; // Move past this match to avoid overlaps
                    if self.pos == start {
                        // Prevent infinite loop on empty matches
                        self.pos += 1;
                    }
                    return Some((start, end));
                }
            }
        }
        None
    }
}

/// Placeholder type for capture groups (not yet implemented).
#[derive(Debug, PartialEq)]
pub struct Captures {
    // Future: will contain captured substrings
}

/// Placeholder iterator for capture groups (not yet implemented).
pub struct CapturesIter<'t> {
    _nfa: &'t NFA,
    _text: &'t str,
    _pos: usize,
}

impl<'t> Iterator for CapturesIter<'t> {
    type Item = Captures;

    fn next(&mut self) -> Option<Self::Item> {
        None // Not yet implemented
    }
}

/// The `NFA` struct represents a non-deterministic finite automaton.
#[derive(Debug, Default)]
pub struct NFA {
    /// Set of all possible states of the NFA.
    pub(crate) states: HashSet<u32>,
    /// Set of all accepting states. If the NFA ends at any one if these the simulation is succesful.
    pub(crate) accept: HashSet<u32>,
    /// The transition function is a map from a pair of a state and a character to a set of states.
    pub(crate) transition_function: HashMap<(u32, char), HashSet<u32>>,
}

impl NFA {
    /// Create a new empty NFA
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a state to the NFA
    pub fn add_state(&mut self, state: u32) {
        self.states.insert(state);
    }

    /// Add an accepting state to the NFA
    pub fn add_accept_state(&mut self, state: u32) {
        self.accept.insert(state);
    }

    /// Add a transition to the NFA
    pub fn add_transition(&mut self, from: u32, symbol: char, to: u32) {
        self.transition_function
            .entry((from, symbol))
            .or_insert_with(HashSet::new)
            .insert(to);
    }

    /// Construct an NFA from raw data (used by macros for compile-time construction)
    pub fn from_raw(
        states: Vec<u32>,
        accept: Vec<u32>,
        transitions: Vec<((u32, char), Vec<u32>)>,
    ) -> Self {
        Self {
            states: states.into_iter().collect(),
            accept: accept.into_iter().collect(),
            transition_function: transitions
                .into_iter()
                .map(|(key, vals)| (key, vals.into_iter().collect()))
                .collect(),
        }
    }

    /// Get states (for compile-time serialization)
    pub fn get_states(&self) -> Vec<u32> {
        let mut states: Vec<_> = self.states.iter().copied().collect();
        states.sort();
        states
    }

    /// Get accept states (for compile-time serialization)
    pub fn get_accept_states(&self) -> Vec<u32> {
        let mut accept: Vec<_> = self.accept.iter().copied().collect();
        accept.sort();
        accept
    }

    /// Get transitions (for compile-time serialization)
    pub fn get_transitions(&self) -> Vec<((u32, char), Vec<u32>)> {
        let mut transitions: Vec<_> = self
            .transition_function
            .iter()
            .map(|(&key, val)| {
                let mut vals: Vec<_> = val.iter().copied().collect();
                vals.sort();
                (key, vals)
            })
            .collect();
        transitions.sort_by_key(|(k, _)| *k);
        transitions
    }

    /// Checks if the pattern matches anywhere in the input text.
    ///
    /// This is the primary matching method, similar to Rust's standard regex `is_match`.
    /// It returns `true` if the pattern is found anywhere in the input string.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use gregex::*;
    ///
    /// let pattern = regex!("abc");
    /// assert!(pattern.is_match("abc"));
    /// assert!(pattern.is_match("xabcy"));  // Matches in the middle
    /// assert!(!pattern.is_match("xyz"));
    /// ```
    pub fn is_match(&self, text: &str) -> bool {
        // Try matching starting from each position in the text
        for start in 0..=text.len() {
            // Try different lengths from this starting position
            for end in start..=text.len() {
                if self.matches_exact(&text[start..end]) {
                    return true;
                }
            }
        }
        false
    }

    /// Finds the first occurrence of the pattern in the text.
    ///
    /// Returns `Some((start, end))` with byte indices if a match is found, or `None` otherwise.
    /// The returned indices represent the shortest match found.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use gregex::*;
    ///
    /// let pattern = regex!("abc");
    /// assert_eq!(pattern.find("xabcy"), Some((1, 4)));
    /// assert_eq!(pattern.find("xyz"), None);
    /// ```
    pub fn find(&self, text: &str) -> Option<(usize, usize)> {
        // Try each starting position
        for start in 0..=text.len() {
            // Try to find the shortest match from this position
            for end in start..=text.len() {
                if self.matches_exact(&text[start..end]) {
                    return Some((start, end));
                }
            }
        }
        None
    }

    /// Returns an iterator over all non-overlapping matches in the text.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use gregex::*;
    ///
    /// let pattern = regex!("ab");
    /// let matches: Vec<_> = pattern.find_iter("abxabxab").collect();
    /// // Returns positions of all "ab" occurrences
    /// ```
    pub fn find_iter<'t>(&'t self, text: &'t str) -> FindIter<'t> {
        FindIter {
            nfa: self,
            text,
            pos: 0,
        }
    }

    /// Placeholder for capture group functionality.
    ///
    /// **Note**: Capture groups are not yet implemented. This method currently
    /// returns `None`. The current implementation focuses on matching without
    /// capturing subgroups.
    ///
    /// # Future Enhancement
    ///
    /// A future version will support capturing groups with syntax like `(a+)`.
    pub fn captures(&self, _text: &str) -> Option<Captures> {
        None // Not yet implemented
    }

    /// Placeholder for capture group iterator functionality.
    ///
    /// **Note**: Capture groups are not yet implemented. This method currently
    /// returns an empty iterator.
    pub fn captures_iter<'t>(&'t self, text: &'t str) -> CapturesIter<'t> {
        CapturesIter {
            _nfa: self,
            _text: text,
            _pos: 0,
        }
    }

    /// Checks if the pattern matches the entire input string exactly.
    ///
    /// This is the core matching logic that verifies if the entire input
    /// string matches the regex pattern from start to end.
    ///
    /// For substring matching (finding pattern anywhere in text), use `is_match()` instead.
    ///
    /// # Arguments
    ///
    /// * `input` - The string to match against
    ///
    /// # Returns
    ///
    /// `true` if the entire input string exactly matches the pattern, `false` otherwise.
    pub fn matches_exact(&self, input: &str) -> bool {
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
        assert!(nfa.matches_exact("ab"));
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
        assert!(nfa.matches_exact("ab"));
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

        assert!(nfa.matches_exact("a"));
        assert!(nfa.matches_exact("aa"));
        assert!(nfa.matches_exact("aaa"));
        assert!(!nfa.matches_exact(""));
        assert!(!nfa.matches_exact("b"));
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
        assert!(nfa.matches_exact("a"));
        assert!(nfa.matches_exact(""));
        assert!(!nfa.matches_exact("aa"));
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

        assert!(nfa.matches_exact("ab"));
        assert!(nfa.matches_exact("abab"));
        assert!(nfa.matches_exact("ababab"));
        assert!(!nfa.matches_exact(""));
        assert!(!nfa.matches_exact("a"));
        assert!(!nfa.matches_exact("b"));
        assert!(!nfa.matches_exact("ba"));
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

        assert!(nfa.matches_exact("a"));
        assert!(nfa.matches_exact("ab"));
        assert!(nfa.matches_exact("aa"));
        assert!(nfa.matches_exact("aab"));
        assert!(!nfa.matches_exact("abb"));
        assert!(!nfa.matches_exact(""));
        assert!(!nfa.matches_exact("b"));
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

        assert!(nfa.matches_exact("b"));
        assert!(nfa.matches_exact("ab"));
        assert!(nfa.matches_exact("aab"));
        assert!(nfa.matches_exact("bb"));
        assert!(nfa.matches_exact("abb"));
        assert!(!nfa.matches_exact(""));
        assert!(!nfa.matches_exact("a"));
        assert!(!nfa.matches_exact("aa"));
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

        assert!(nfa.matches_exact(""));
        assert!(nfa.matches_exact("a"));
        assert!(nfa.matches_exact("b"));
        assert!(nfa.matches_exact("ab"));
        assert!(nfa.matches_exact("abb"));
        assert!(nfa.matches_exact("bb"));
        assert!(!nfa.matches_exact("aa"));
        assert!(!nfa.matches_exact("aab"));
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

        assert!(nfa.matches_exact(""));
        assert!(nfa.matches_exact("a"));
        assert!(nfa.matches_exact("aa"));
        assert!(nfa.matches_exact("b"));
        assert!(!nfa.matches_exact("ab"));
        assert!(!nfa.matches_exact("bb"));
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

        assert!(nfa.matches_exact(""));
        assert!(nfa.matches_exact("a"));
        assert!(nfa.matches_exact("aa"));
        assert!(nfa.matches_exact("aaa"));
        assert!(!nfa.matches_exact("b"));
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

        assert!(nfa.matches_exact("a"));
        assert!(nfa.matches_exact("b"));
        assert!(nfa.matches_exact("ac"));
        assert!(nfa.matches_exact("bc"));
        assert!(nfa.matches_exact("abc"));
        assert!(nfa.matches_exact("aac"));
        assert!(!nfa.matches_exact(""));
        assert!(!nfa.matches_exact("c"));
        assert!(!nfa.matches_exact("acc"));
    }
}
