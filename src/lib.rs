//! This crate provides a [regular expression](https://en.wikipedia.org/wiki/Regular_expression) engine that uses a [Nondeterministic finite automaton](https://en.wikipedia.org/wiki/Nondeterministic_finite_automaton) to simulate the regular expression.
//!  Here is a short example on how to use this crate
//!
//! ```rust
//! use::gregex::regex;
//!
//! let regex = regex("(a.b)*");
//! assert!(regex.simulate("abab"));
//! ```
//!
//! The regex function uses the regular expression string to create a NFA that can be used to simulate the regular expression.
//! The program uses the [Glushkov's construction algorithm](https://en.wikipedia.org/wiki/Glushkov%27s_construction_algorithm) to create its NFA.
//! The NFA is then later simulated to check if the input string matches the regular expression.
//!
//! A brief overview of the pipeline:
//! [![](https://mermaid.ink/img/pako:eNptkEGLwjAQhf9KmNMK-gd6ENRat6CymGPiITTTGtYkEiesi_jfN2260IXNad77XibhPaHxGqGA9uq_mosKxPYn6Vg6K3HCDh-MUzCuO7PFYrkWtWvNg5FnHwHTdM7RdQ_ZZrxAAXEEmwGUIqcZR_oDtoLH9j9QiUo15MN9QsqB7MSxWo3ONjtZVFPx_la7W6Tx77MB1RntpqIW3Nh4VemptLP3YA4Wg1VGp1KevSOBLmhRQpFGrcKnBOleKacief7tGigoRJxDvGlFWBrVBWV_TdQmLT_kkoeuXz_VYW9H?type=png)](https://mermaid.live/edit#pako:eNptkEGLwjAQhf9KmNMK-gd6ENRat6CymGPiITTTGtYkEiesi_jfN2260IXNad77XibhPaHxGqGA9uq_mosKxPYn6Vg6K3HCDh-MUzCuO7PFYrkWtWvNg5FnHwHTdM7RdQ_ZZrxAAXEEmwGUIqcZR_oDtoLH9j9QiUo15MN9QsqB7MSxWo3ONjtZVFPx_la7W6Tx77MB1RntpqIW3Nh4VemptLP3YA4Wg1VGp1KevSOBLmhRQpFGrcKnBOleKacief7tGigoRJxDvGlFWBrVBWV_TdQmLT_kkoeuXz_VYW9H)

pub mod nfa;
pub mod translation;

use nfa::*;
use translation::linearize::linearize;
use translation::node::*;

type Regex = NFA;

/// Creates a NFA from a regular expression string.
///
/// # Note
/// Currently the regular expression engine only supports the following operators:
/// - `.` (Concatenation)
/// - `*` (Kleene star)
/// - `|` (Or)
///
/// # Example
/// ```rust
/// use::gregex::regex;
///
/// let regex = regex("(a.b)*");
/// assert!(regex.simulate("abab"));
/// ```
pub fn regex(regex_string: &str) -> Regex {
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
