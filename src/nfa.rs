//! This module contains the implementation of a non-deterministic finite automaton (NFA).

use crate::translation::setterminal::SetTerminal;
use core::panic;
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
pub struct NFA {
    states: HashSet<u32>,
    accept: HashSet<u32>,
    /// The transition function is a map from a pair of a state and a character to a set of states.
    transition_function: HashMap<(u32, char), HashSet<u32>>,
}

impl Default for NFA {
    fn default() -> Self {
        NFA {
            states: HashSet::new(),
            accept: HashSet::new(),
            transition_function: HashMap::new(),
        }
    }
}

impl NFA {
    fn new(
        states: HashSet<u32>,
        accept: HashSet<u32>,
        transition_function: HashMap<(u32, char), HashSet<u32>>,
    ) -> NFA {
        NFA {
            states,
            accept,
            transition_function,
        }
    }

    pub fn simulate(&self, input: &str) -> bool {
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

    pub fn set_to_nfa(
        prefix_set: &HashSet<SetTerminal>,
        suffix_set: &HashSet<SetTerminal>,
        factors_set: &HashSet<SetTerminal>,
    ) -> Self {
        let mut nfa = Self::default();
    
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
                    nfa.transition_function.entry((index1, symbol2)).or_insert_with(|| HashSet::new()).insert(index2);
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
        assert!(nfa.simulate("ab"));
    }

    #[test]
    fn set_to_nfa_simple_test() {
        let prefix_set = vec![SetTerminal::SingleElement('a', 1)].into_iter().collect();
        let suffix_set = vec![SetTerminal::SingleElement('b', 2)].into_iter().collect();
        let factors_set = vec![SetTerminal::DoubleElement('a', 1, 'b', 2)].into_iter().collect();
        let nfa = NFA::set_to_nfa(&prefix_set, &suffix_set, &factors_set);
        assert!(nfa.simulate("ab"));
    }
}
