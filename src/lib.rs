mod nfa;
mod translation;

use nfa::*;
use translation::linearize::linearize;
use translation::node::*;

type Regex = NFA;

fn regex(regex_string: &str) -> Regex {
    let regex_tree = linearize(regex_string);
    let prefix_set = &prefix_set(&regex_tree);
    let suffix_set = &suffix_set(&regex_tree);
    let factors_set = &factors_set(&regex_tree);
    NFA::set_to_nfa(prefix_set, suffix_set, factors_set)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_regex() {
        assert!(regex("a*.b").simulate("b"));
        assert!(regex("a*.b").simulate("ab"));
        assert!(regex("a*.b").simulate("aaab"));
        assert!(regex("a.b").simulate("ab"));
        assert!(!regex("a.b").simulate("a"));
        assert!(regex("a|b").simulate("a"));
    }
}
