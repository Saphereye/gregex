//! Converts input regex to its linear form. Then it converts it into a Node tree.

use std::collections::HashMap;
use std::process::Child;

use crate::translation::node::Node;
use crate::translation::operator::Operator;

fn string_to_infix(input: &str) -> String {
    input.replace(")(", ").(")
}

fn precedence(c: &char) -> u8 {
    match c {
        '*' => 3,
        '|' => 2,
        '.' => 1,
        _ => 0,
    }
}

fn infix_to_postfix(infix: &str) -> String {
    let mut stack = Vec::new();
    let mut postfix = String::new();

    for c in infix.chars() {
        match c {
            '(' => stack.push(c),
            ')' => {
                while let Some(top) = stack.pop() {
                    if top == '(' {
                        break;
                    }
                    postfix.push(top);
                }
            }
            '*' | '|' | '.' => {
                while let Some(top) = stack.last() {
                    if precedence(&c) <= precedence(top) {
                        postfix.push(stack.pop().unwrap());
                    } else {
                        break;
                    }
                }
                stack.push(c);
            }
            _ => postfix.push(c),
        }
    }

    while let Some(top) = stack.pop() {
        postfix.push(top);
    }

    postfix
}

fn postfix_to_nodetree(postfix: &str) -> Node {
    let mut stack = Vec::new();

    let mut count = 0;

    for c in postfix.chars() {
        match c {
            '*' => {
                let child = stack.pop().unwrap();
                stack.push(Node::Operation(Operator::Production, Box::new(child), None));
            }
            '|' => {
                let right = stack.pop().unwrap();
                let left = stack.pop().unwrap();
                stack.push(Node::Operation(
                    Operator::Or,
                    Box::new(left),
                    Some(Box::new(right)),
                ));
            }
            '.' => {
                let right = stack.pop().unwrap();
                let left = stack.pop().unwrap();
                stack.push(Node::Operation(
                    Operator::Concat,
                    Box::new(left),
                    Some(Box::new(right)),
                ));
            }
            _ => {
                count += 1;
                stack.push(Node::Terminal(c, count));
            }
        }
    }

    stack.pop().unwrap()
}

/// Converts input regex to its linear form. Then it converts it into a Node tree.
pub fn linearize(input: &str) -> Node {
    let infix = string_to_infix(input);
    let postfix = infix_to_postfix(&infix);
    postfix_to_nodetree(&postfix)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_infix_to_postfix() {
        assert_eq!(infix_to_postfix("a"), "a");
        assert_eq!(infix_to_postfix("a*"), "a*");
        assert_eq!(infix_to_postfix("a|b"), "ab|");
        assert_eq!(infix_to_postfix("(a.b)|b"), "ab.b|");
    }

    #[test]
    fn test_postfix_to_nodetree() {
        assert_eq!(postfix_to_nodetree("a"), Node::Terminal('a', 1));
        assert_eq!(
            postfix_to_nodetree("a*"),
            Node::Operation(Operator::Production, Box::new(Node::Terminal('a', 1)), None)
        );
        assert_eq!(
            postfix_to_nodetree("ab|"),
            Node::Operation(
                Operator::Or,
                Box::new(Node::Terminal('a', 1)),
                Some(Box::new(Node::Terminal('b', 2)))
            )
        );
        assert_eq!(
            postfix_to_nodetree("ab|*"),
            Node::Operation(
                Operator::Production,
                Box::new(Node::Operation(
                    Operator::Or,
                    Box::new(Node::Terminal('a', 1)),
                    Some(Box::new(Node::Terminal('b', 2)))
                )),
                None
            )
        )
    }
}
