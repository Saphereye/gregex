//! Contains the implementation of the `Node` enum and the functions to calculate the nullability, prefix, suffix and factors sets of a regular expression tree.

use crate::translation::operator::Operator;
use crate::translation::setterminal::SetTerminal;
use std::collections::HashSet;

/// The `Node` enum represents the different types of nodes that can be used in a regular expression tree.
#[derive(Debug, PartialEq, Eq)]
pub enum Node {
    /// Represents an operation on one or two nodes.
    Operation(Operator, Box<Node>, Option<Box<Node>>),
    /// `char` represents the character, `u32` represent the unique identifier of the node.
    Terminal(char, u32),
}

/// The `nullability_set` function returns the set of [SetTerminal] that are nullable in a regular expression tree.
pub fn nullability_set(regex_tree: &Node) -> HashSet<SetTerminal> {
    let mut set = HashSet::new();
    match regex_tree {
        Node::Terminal(_, _) => {
            set.insert(SetTerminal::Empty);
        }
        Node::Operation(op, left, right) => match op {
            Operator::Or => {
                set.extend(nullability_set(left));
                set.extend(nullability_set(right.as_ref().unwrap()));
            }
            Operator::Concat => {
                set.extend(nullability_set(left));
                let right_set = nullability_set(right.as_ref().unwrap());
                set.extend(right_set);
            }
            Operator::Production => {
                set.insert(SetTerminal::Epsilon);
            }
            _ => todo!(),
        },
    }
    set
}

/// The `prefix_set` function returns the set of [SetTerminal] that are prefixes of a regular expression tree.
pub fn prefix_set(regex_tree: &Node) -> HashSet<SetTerminal> {
    let mut set = HashSet::new();
    match regex_tree {
        Node::Terminal(symbol, code) => {
            set.insert(SetTerminal::SingleElement(*symbol, *code));
        }
        Node::Operation(op, left, right) => match op {
            Operator::Or => {
                let left_set = prefix_set(left);
                let right_set = prefix_set(right.as_ref().unwrap());
                set.extend(left_set);
                set.extend(right_set);
            }
            Operator::Concat => {
                let left_set = prefix_set(left);
                set.extend(left_set);
                let right_set = prefix_set(right.as_ref().unwrap());
                let nullable_set = nullability_set(left);

                // If the left expression is nullable, include the first set of the right expression
                if nullable_set.contains(&SetTerminal::Epsilon) {
                    set.extend(right_set);
                }
            }
            Operator::Production => {
                let left_set = prefix_set(left);
                set = left_set;
            }
            _ => todo!(),
        },
    }
    set
}

/// The `suffix_set` function returns the set of [SetTerminal] that are suffixes of a regular expression tree.
pub fn suffix_set(regex_tree: &Node) -> HashSet<SetTerminal> {
    let mut set = HashSet::new();
    match regex_tree {
        Node::Terminal(symbol, code) => {
            set.insert(SetTerminal::SingleElement(*symbol, *code));
        }
        Node::Operation(op, left, right) => match op {
            Operator::Or => {
                let left_set = suffix_set(left);
                let right_set = suffix_set(right.as_ref().unwrap());
                set.extend(left_set);
                set.extend(right_set);
            }
            Operator::Concat => {
                let left_set = suffix_set(right.as_ref().unwrap());
                set.extend(left_set);
                let right_set = suffix_set(left);
                let nullable_set = nullability_set(right.as_ref().unwrap());

                // If the left expression is nullable, include the first set of the right expression
                if nullable_set.contains(&SetTerminal::Epsilon) {
                    set.extend(right_set);
                }
            }
            Operator::Production => {
                let left_set = suffix_set(left);
                set = left_set;
            }
            _ => todo!(),
        },
    }
    set
}

/// The `factors_set` function returns the set of [SetTerminal] that are factors of a regular expression tree.
/// 
/// Factors in this scenario mean the set of terminals that can be produced by the regular expression.
pub fn factors_set(regex_tree: &Node) -> HashSet<SetTerminal> {
    let mut set = HashSet::new();
    match regex_tree {
        Node::Terminal(_, _) => {
            set.insert(SetTerminal::Empty);
        }
        Node::Operation(op, left, right) => match op {
            Operator::Or => {
                let left_set = factors_set(left);
                let right_set = factors_set(right.as_ref().unwrap());
                set.extend(left_set);
                set.extend(right_set);
            }
            Operator::Concat => {
                let left_set = factors_set(left);
                let right_set = factors_set(right.as_ref().unwrap());
                let suffix_set = suffix_set(left);
                let prefix_set = prefix_set(right.as_ref().unwrap());
                set.extend(left_set);
                set.extend(right_set);
                for i in suffix_set {
                    for j in &prefix_set {
                        set.insert(i.product(j));
                    }
                }
            }
            Operator::Production => {
                let left_set = factors_set(left);
                let suffix_set = suffix_set(left);
                let prefix_set = prefix_set(left);
                set.extend(left_set);

                for i in suffix_set {
                    for j in &prefix_set {
                        set.insert(i.product(j));
                    }
                }
            }
            _ => todo!(),
        },
    }

    if set.contains(&SetTerminal::Empty) && set.len() > 1 {
        set.remove(&SetTerminal::Empty);
    }
    set
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nullability_set_test_or() {
        let tree = Node::Operation(
            Operator::Or,
            Box::new(Node::Terminal('a', 1)),
            Option::Some(Box::new(Node::Terminal('b', 2))),
        );

        let set = nullability_set(&tree);
        let mut test_set = HashSet::new();
        test_set.insert(SetTerminal::Empty);
        assert_eq!(set, test_set);
    }

    #[test]
    fn nullability_set_test_concat() {
        let tree = Node::Operation(
            Operator::Concat,
            Box::new(Node::Terminal('a', 1)),
            Option::Some(Box::new(Node::Terminal('b', 2))),
        );

        let set = nullability_set(&tree);
        let mut test_set = HashSet::new();
        test_set.insert(SetTerminal::Empty);
        assert_eq!(set, test_set);
    }

    #[test]
    fn nullability_set_test_production() {
        let tree = Node::Operation(Operator::Production, Box::new(Node::Terminal('a', 1)), None);

        let set = nullability_set(&tree);
        let mut test_set = HashSet::new();
        test_set.insert(SetTerminal::Epsilon);
        assert_eq!(set, test_set);
    }

    #[test]
    fn nullability_set_test_terminal() {
        let tree = Node::Terminal('a', 1);

        let set = nullability_set(&tree);
        let mut test_set = HashSet::new();
        test_set.insert(SetTerminal::Empty);
        assert_eq!(set, test_set);
    }

    #[test]
    fn prefix_set_test_or() {
        let tree = Node::Operation(
            Operator::Or,
            Box::new(Node::Terminal('a', 1)),
            Option::Some(Box::new(Node::Terminal('b', 2))),
        );

        let set = prefix_set(&tree);
        let mut test_set = HashSet::new();
        test_set.insert(SetTerminal::SingleElement('a', 1));
        test_set.insert(SetTerminal::SingleElement('b', 2));
        assert_eq!(set, test_set);
    }

    #[test]
    fn prefix_set_test_production() {
        let tree = Node::Operation(Operator::Production, Box::new(Node::Terminal('a', 1)), None);

        let set = prefix_set(&tree);
        let mut test_set = HashSet::new();
        test_set.insert(SetTerminal::SingleElement('a', 1));
        assert_eq!(set, test_set);
    }

    #[test]
    fn prefix_set_test_concat() {
        let tree = Node::Operation(
            Operator::Concat,
            Box::new(Node::Terminal('a', 1)),
            Option::Some(Box::new(Node::Terminal('b', 2))),
        );

        let set = prefix_set(&tree);
        let mut test_set = HashSet::new();
        test_set.insert(SetTerminal::SingleElement('a', 1));
        assert_eq!(set, test_set);
    }

    #[test]
    fn prefix_set_test_terminal() {
        let tree = Node::Terminal('a', 1);

        let set = prefix_set(&tree);
        let mut test_set = HashSet::new();
        test_set.insert(SetTerminal::SingleElement('a', 1));
        assert_eq!(set, test_set);
    }

    #[test]
    fn prefix_set_test_complete() {
        // Linearized regex: (a(ab)*)* + (ba)*
        let tree = Node::Operation(
            Operator::Or,
            Box::new(Node::Operation(
                Operator::Production,
                Box::new(Node::Operation(
                    Operator::Concat,
                    Box::new(Node::Terminal('a', 1)),
                    Some(Box::new(Node::Operation(
                        Operator::Production,
                        Box::new(Node::Operation(
                            Operator::Concat,
                            Box::new(Node::Terminal('a', 2)),
                            Option::Some(Box::new(Node::Terminal('b', 3))),
                        )),
                        None,
                    ))),
                )),
                None,
            )),
            Option::Some(Box::new(Node::Operation(
                Operator::Production,
                Box::new(Node::Operation(
                    Operator::Concat,
                    Box::new(Node::Terminal('b', 4)),
                    Option::Some(Box::new(Node::Terminal('a', 5))),
                )),
                None,
            ))),
        );

        let set = prefix_set(&tree);
        let mut test_set = HashSet::new();
        test_set.insert(SetTerminal::SingleElement('a', 1));
        test_set.insert(SetTerminal::SingleElement('b', 4));
        assert_eq!(set, test_set);
    }

    #[test]
    fn suffix_set_test_or() {
        let tree = Node::Operation(
            Operator::Or,
            Box::new(Node::Terminal('a', 1)),
            Option::Some(Box::new(Node::Terminal('b', 2))),
        );

        let set = suffix_set(&tree);
        let mut test_set = HashSet::new();
        test_set.insert(SetTerminal::SingleElement('a', 1));
        test_set.insert(SetTerminal::SingleElement('b', 2));
        assert_eq!(set, test_set);
    }

    #[test]
    fn suffix_set_test_production() {
        let tree = Node::Operation(Operator::Production, Box::new(Node::Terminal('a', 1)), None);

        let set = suffix_set(&tree);
        let mut test_set = HashSet::new();
        test_set.insert(SetTerminal::SingleElement('a', 1));
        assert_eq!(set, test_set);
    }

    #[test]
    fn suffix_set_test_concat() {
        let tree = Node::Operation(
            Operator::Concat,
            Box::new(Node::Terminal('a', 1)),
            Option::Some(Box::new(Node::Terminal('b', 2))),
        );

        let set = suffix_set(&tree);
        let mut test_set = HashSet::new();
        test_set.insert(SetTerminal::SingleElement('b', 2));
        assert_eq!(set, test_set);
    }

    #[test]
    fn suffix_set_test_terminal() {
        let tree = Node::Terminal('a', 1);

        let set = suffix_set(&tree);
        let mut test_set = HashSet::new();
        test_set.insert(SetTerminal::SingleElement('a', 1));
        assert_eq!(set, test_set);
    }

    #[test]
    fn suffix_set_test_complete() {
        // Linearized regex: (a(ab)*)* + (ba)*
        let tree = Node::Operation(
            Operator::Or,
            Box::new(Node::Operation(
                Operator::Production,
                Box::new(Node::Operation(
                    Operator::Concat,
                    Box::new(Node::Terminal('a', 1)),
                    Some(Box::new(Node::Operation(
                        Operator::Production,
                        Box::new(Node::Operation(
                            Operator::Concat,
                            Box::new(Node::Terminal('a', 2)),
                            Option::Some(Box::new(Node::Terminal('b', 3))),
                        )),
                        None,
                    ))),
                )),
                None,
            )),
            Option::Some(Box::new(Node::Operation(
                Operator::Production,
                Box::new(Node::Operation(
                    Operator::Concat,
                    Box::new(Node::Terminal('b', 4)),
                    Option::Some(Box::new(Node::Terminal('a', 5))),
                )),
                None,
            ))),
        );

        let set = suffix_set(&tree);
        let mut test_set = HashSet::new();
        test_set.insert(SetTerminal::SingleElement('a', 1));
        test_set.insert(SetTerminal::SingleElement('b', 3));
        test_set.insert(SetTerminal::SingleElement('a', 5));
        assert_eq!(set, test_set);
    }

    #[test]
    fn factors_set_test_or() {
        let tree = Node::Operation(
            Operator::Or,
            Box::new(Node::Terminal('a', 1)),
            Option::Some(Box::new(Node::Terminal('b', 2))),
        );

        let set = factors_set(&tree);
        let mut test_set = HashSet::new();
        test_set.insert(SetTerminal::Empty);
        assert_eq!(set, test_set);
    }

    #[test]
    fn factors_set_test_production() {
        let tree = Node::Operation(Operator::Production, Box::new(Node::Terminal('a', 1)), None);

        let set = factors_set(&tree);
        let mut test_set = HashSet::new();
        test_set.insert(SetTerminal::DoubleElement('a', 1, 'a', 1));
        assert_eq!(set, test_set);
    }

    #[test]
    fn factors_set_test_concat() {
        let tree = Node::Operation(
            Operator::Concat,
            Box::new(Node::Terminal('a', 1)),
            Option::Some(Box::new(Node::Terminal('b', 2))),
        );

        let set = factors_set(&tree);
        let mut test_set = HashSet::new();
        test_set.insert(SetTerminal::DoubleElement('a', 1, 'b', 2));
        assert_eq!(set, test_set);
    }

    #[test]
    fn factors_set_test_complete() {
        // Linearized regex: (a(ab)*)* + (ba)*
        let tree = Node::Operation(
            Operator::Or,
            Box::new(Node::Operation(
                Operator::Production,
                Box::new(Node::Operation(
                    Operator::Concat,
                    Box::new(Node::Terminal('a', 1)),
                    Some(Box::new(Node::Operation(
                        Operator::Production,
                        Box::new(Node::Operation(
                            Operator::Concat,
                            Box::new(Node::Terminal('a', 2)),
                            Option::Some(Box::new(Node::Terminal('b', 3))),
                        )),
                        None,
                    ))),
                )),
                None,
            )),
            Option::Some(Box::new(Node::Operation(
                Operator::Production,
                Box::new(Node::Operation(
                    Operator::Concat,
                    Box::new(Node::Terminal('b', 4)),
                    Option::Some(Box::new(Node::Terminal('a', 5))),
                )),
                None,
            ))),
        );

        let set = factors_set(&tree);
        let mut test_set = HashSet::new();
        test_set.insert(SetTerminal::DoubleElement('a', 1, 'a', 2));
        test_set.insert(SetTerminal::DoubleElement('a', 1, 'a', 1));
        test_set.insert(SetTerminal::DoubleElement('a', 2, 'b', 3));
        test_set.insert(SetTerminal::DoubleElement('b', 3, 'a', 1));
        test_set.insert(SetTerminal::DoubleElement('b', 3, 'a', 2));
        test_set.insert(SetTerminal::DoubleElement('b', 4, 'a', 5));
        test_set.insert(SetTerminal::DoubleElement('a', 5, 'b', 4));
        assert_eq!(set, test_set);
    }
}
