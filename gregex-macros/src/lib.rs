#[doc = include_str!("../README.md")]
#[cfg(not(doctest))]
extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Expr, ExprLit, ExprMacro, Lit};

/// Internal regex parser module using Pratt parsing technique.
///
/// This module implements a recursive descent parser with operator precedence
/// for parsing regex syntax strings at compile time. It supports:
/// - Literals (a, b, c, ...)
/// - Postfix operators (*, +, ?)
/// - Infix operator (|)
/// - Grouping with parentheses ()
/// - Implicit concatenation
mod regex_parser {
    use gregex_logic::translation::node::Node;
    use gregex_logic::translation::operator::Operator;
    use quote::quote;
    use std::sync::atomic::Ordering;

    #[derive(Debug, Clone, PartialEq)]
    enum Token {
        Char(char),
        Star,
        Plus,
        Question,
        Pipe,
        LParen,
        RParen,
        Eof,
    }

    struct Lexer {
        chars: Vec<char>,
        pos: usize,
    }

    impl Lexer {
        fn new(input: &str) -> Self {
            Lexer {
                chars: input.chars().collect(),
                pos: 0,
            }
        }

        fn next(&mut self) -> Token {
            if self.pos >= self.chars.len() {
                return Token::Eof;
            }

            let ch = self.chars[self.pos];
            self.pos += 1;

            match ch {
                '*' => Token::Star,
                '+' => Token::Plus,
                '?' => Token::Question,
                '|' => Token::Pipe,
                '(' => Token::LParen,
                ')' => Token::RParen,
                c => Token::Char(c),
            }
        }

        fn peek(&self) -> Token {
            if self.pos >= self.chars.len() {
                return Token::Eof;
            }

            let ch = self.chars[self.pos];
            match ch {
                '*' => Token::Star,
                '+' => Token::Plus,
                '?' => Token::Question,
                '|' => Token::Pipe,
                '(' => Token::LParen,
                ')' => Token::RParen,
                c => Token::Char(c),
            }
        }
    }

    pub fn parse(input: &str) -> proc_macro2::TokenStream {
        let mut lexer = Lexer::new(input);
        parse_or(&mut lexer)
    }

    fn parse_or(lexer: &mut Lexer) -> proc_macro2::TokenStream {
        let mut left = parse_concat(lexer);

        while lexer.peek() == Token::Pipe {
            lexer.next(); // consume '|'
            let right = parse_concat(lexer);
            left = quote! {
                gregex_logic::translation::node::Node::Operation(
                    gregex_logic::translation::operator::Operator::Or,
                    Box::new(#left),
                    Some(Box::new(#right))
                )
            };
        }

        left
    }

    fn parse_concat(lexer: &mut Lexer) -> proc_macro2::TokenStream {
        let mut nodes = Vec::new();

        loop {
            match lexer.peek() {
                Token::Eof | Token::RParen | Token::Pipe => break,
                _ => nodes.push(parse_postfix(lexer)),
            }
        }

        if nodes.is_empty() {
            panic!("Empty expression");
        }

        let mut result = nodes[0].clone();
        for node in nodes.iter().skip(1) {
            result = quote! {
                gregex_logic::translation::node::Node::Operation(
                    gregex_logic::translation::operator::Operator::Concat,
                    Box::new(#result),
                    Some(Box::new(#node))
                )
            };
        }

        result
    }

    fn parse_postfix(lexer: &mut Lexer) -> proc_macro2::TokenStream {
        let mut node = parse_atom(lexer);

        loop {
            match lexer.peek() {
                Token::Star => {
                    lexer.next();
                    node = quote! {
                        gregex_logic::translation::node::Node::Operation(
                            gregex_logic::translation::operator::Operator::Production,
                            Box::new(#node),
                            None
                        )
                    };
                }
                Token::Plus => {
                    lexer.next();
                    node = quote! {
                        gregex_logic::translation::node::Node::Operation(
                            gregex_logic::translation::operator::Operator::Plus,
                            Box::new(#node),
                            None
                        )
                    };
                }
                Token::Question => {
                    lexer.next();
                    node = quote! {
                        gregex_logic::translation::node::Node::Operation(
                            gregex_logic::translation::operator::Operator::Question,
                            Box::new(#node),
                            None
                        )
                    };
                }
                _ => break,
            }
        }

        node
    }

    fn parse_atom(lexer: &mut Lexer) -> proc_macro2::TokenStream {
        match lexer.next() {
            Token::Char(c) => {
                let count =
                    gregex_logic::TERMINAL_COUNT.fetch_add(1, core::sync::atomic::Ordering::SeqCst);
                quote! {
                    gregex_logic::translation::node::Node::Terminal(#c, #count)
                }
            }
            Token::LParen => {
                let node = parse_or(lexer);
                if lexer.next() != Token::RParen {
                    panic!("Expected closing parenthesis");
                }
                node
            }
            _ => panic!("Unexpected token in atom"),
        }
    }

    /// Parse a regex string directly to a Node (for compile-time NFA construction)
    pub fn parse_to_node(pattern: &str) -> Node {
        let mut lexer = Lexer::new(pattern);
        parse_or_impl(&mut lexer)
    }

    fn parse_or_impl(lexer: &mut Lexer) -> Node {
        let mut left = parse_concat_impl(lexer);

        while lexer.peek() == Token::Pipe {
            lexer.next(); // consume |
            let right = parse_concat_impl(lexer);
            left = Node::Operation(Operator::Or, Box::new(left), Some(Box::new(right)));
        }

        left
    }

    fn parse_concat_impl(lexer: &mut Lexer) -> Node {
        let mut nodes = Vec::new();

        loop {
            match lexer.peek() {
                Token::Char(_) | Token::LParen => {
                    nodes.push(parse_postfix_impl(lexer));
                }
                _ => break,
            }
        }

        if nodes.is_empty() {
            panic!("Empty expression");
        }

        if nodes.len() == 1 {
            return nodes.into_iter().next().unwrap();
        }

        let mut iter = nodes.into_iter();
        let mut result = iter.next().unwrap();
        for node in iter {
            result = Node::Operation(Operator::Concat, Box::new(result), Some(Box::new(node)));
        }
        result
    }

    fn parse_postfix_impl(lexer: &mut Lexer) -> Node {
        let mut node = parse_atom_impl(lexer);

        loop {
            match lexer.peek() {
                Token::Star => {
                    lexer.next();
                    node = Node::Operation(Operator::Production, Box::new(node), None);
                }
                Token::Plus => {
                    lexer.next();
                    node = Node::Operation(Operator::Plus, Box::new(node), None);
                }
                Token::Question => {
                    lexer.next();
                    node = Node::Operation(Operator::Question, Box::new(node), None);
                }
                _ => break,
            }
        }

        node
    }

    fn parse_atom_impl(lexer: &mut Lexer) -> Node {
        match lexer.next() {
            Token::Char(c) => {
                let count = gregex_logic::TERMINAL_COUNT.fetch_add(1, Ordering::SeqCst);
                Node::Terminal(c, count)
            }
            Token::LParen => {
                let node = parse_or_impl(lexer);
                match lexer.next() {
                    Token::RParen => node,
                    _ => panic!("Expected closing parenthesis"),
                }
            }
            _ => panic!("Unexpected token in atom"),
        }
    }
}

/// Helper function to convert a literal (char or string) into a Node tree.
///
/// This function handles both single character literals and string literals,
/// automatically expanding strings into concatenated terminal nodes.
///
/// # Arguments
///
/// * `lit` - A reference to a `Lit` (literal) from the syn crate
///
/// # Returns
///
/// A `TokenStream` representing the generated Node structure
///
/// # Panics
///
/// Panics if the literal is not a `Char` or `Str`, or if the string is empty.
fn lit_to_node(lit: &Lit) -> proc_macro2::TokenStream {
    match lit {
        Lit::Char(c) => {
            let count =
                gregex_logic::TERMINAL_COUNT.fetch_add(1, core::sync::atomic::Ordering::SeqCst);
            quote! {
                gregex_logic::translation::node::Node::Terminal(#c, #count)
            }
        }
        Lit::Str(s) => {
            let chars: Vec<char> = s.value().chars().collect();
            if chars.is_empty() {
                panic!("Empty strings are not supported");
            }
            let nodes: Vec<_> = chars
                .iter()
                .map(|c| {
                    let count = gregex_logic::TERMINAL_COUNT
                        .fetch_add(1, core::sync::atomic::Ordering::SeqCst);
                    quote! {
                        gregex_logic::translation::node::Node::Terminal(#c, #count)
                    }
                })
                .collect();

            // Chain nodes with Concat operators
            let mut result = nodes[0].clone();
            for node in nodes.iter().skip(1) {
                result = quote! {
                    gregex_logic::translation::node::Node::Operation(
                        gregex_logic::translation::operator::Operator::Concat,
                        Box::new(#result),
                        Some(Box::new(#node))
                    )
                };
            }
            result
        }
        _ => panic!("Unsupported literal type"),
    }
}

/// Creates a concatenation (sequence) pattern from the given expressions.
///
/// Accepts character literals, string literals, and nested macro expressions.
/// String literals are automatically expanded into sequences.
#[proc_macro]
pub fn dot(input: TokenStream) -> TokenStream {
    let inputs = parse_macro_input!(input with syn::punctuated::Punctuated::<Expr, syn::Token![,]>::parse_terminated);

    let nodes = inputs.iter().map(|expr| {
        match expr {
            Expr::Macro(ExprMacro { mac, .. }) => {
                // Handle procedural macro
                quote! { #mac }
            }
            Expr::Lit(ExprLit { lit, .. }) => lit_to_node(&lit),
            _ => panic!("Unsupported input type"),
        }
    });

    // Generate the code for concatenating nodes
    let mut iter = nodes.into_iter();
    let first = iter.next().expect("The input is empty");
    let operations = iter.fold(first, |left, right| {
        quote! {
            gregex_logic::translation::node::Node::Operation(
                gregex_logic::translation::operator::Operator::Concat,
                Box::new(#left),
                Some(Box::new(#right))
            )
        }
    });

    // Generate the final token stream
    let gen = quote! {
        #operations
    };

    gen.into()
}

/// Creates an alternation (OR) pattern from the given expressions.
///
/// Matches if any one of the given expressions matches.
/// Accepts character literals, string literals, and nested macro expressions.
#[proc_macro]
pub fn or(input: TokenStream) -> TokenStream {
    let inputs = parse_macro_input!(input with syn::punctuated::Punctuated::<Expr, syn::Token![,]>::parse_terminated);

    let nodes = inputs.iter().map(|expr| {
        match expr {
            Expr::Macro(ExprMacro { mac, .. }) => {
                // Handle procedural macro
                quote! { #mac }
            }
            Expr::Lit(ExprLit { lit, .. }) => lit_to_node(&lit),
            _ => panic!("Unsupported input type"),
        }
    });

    // Generate the code for concatenating nodes
    let mut iter = nodes.into_iter();
    let first = iter.next().expect("The input is empty");
    let operations = iter.fold(first, |left, right| {
        quote! {
            gregex_logic::translation::node::Node::Operation(
                gregex_logic::translation::operator::Operator::Or,
                Box::new(#left),
                Some(Box::new(#right))
            )
        }
    });

    // Generate the final token stream
    let gen = quote! {
        #operations
    };

    gen.into()
}

/// Creates a Kleene star (zero or more) pattern for the given expression.
///
/// Matches zero or more repetitions of the input.
/// Accepts character literals, string literals, and nested macro expressions.
#[proc_macro]
pub fn star(input: TokenStream) -> TokenStream {
    let expr = parse_macro_input!(input as Expr);

    let node = match expr {
        Expr::Macro(ExprMacro { mac, .. }) => {
            // Handle procedural macro
            quote! { #mac }
        }
        Expr::Lit(ExprLit { lit, .. }) => lit_to_node(&lit),
        _ => panic!("Unsupported input type"),
    };

    // Generate the code for the star operation
    let operation = quote! {
        gregex_logic::translation::node::Node::Operation(
            gregex_logic::translation::operator::Operator::Production,
            Box::new(#node),
            None
        )
    };

    // Generate the final token stream
    let gen = quote! {
        #operation
    };

    gen.into()
}

/// Creates a plus (one or more) pattern for the given expression.
///
/// Matches one or more repetitions of the input.
/// Accepts character literals, string literals, and nested macro expressions.
#[proc_macro]
pub fn plus(input: TokenStream) -> TokenStream {
    let expr = parse_macro_input!(input as Expr);

    let node = match expr {
        Expr::Macro(ExprMacro { mac, .. }) => {
            // Handle procedural macro
            quote! { #mac }
        }
        Expr::Lit(ExprLit { lit, .. }) => lit_to_node(&lit),
        _ => panic!("Unsupported input type"),
    };

    // Generate the code for the plus operation
    let operation = quote! {
        gregex_logic::translation::node::Node::Operation(
            gregex_logic::translation::operator::Operator::Plus,
            Box::new(#node),
            None
        )
    };

    // Generate the final token stream
    let gen = quote! {
        #operation
    };

    gen.into()
}

/// Creates a question (zero or one) pattern for the given expression.
///
/// Matches zero or one occurrence of the input.
/// Accepts character literals, string literals, and nested macro expressions.
#[proc_macro]
pub fn question(input: TokenStream) -> TokenStream {
    let expr = parse_macro_input!(input as Expr);

    let node = match expr {
        Expr::Macro(ExprMacro { mac, .. }) => {
            // Handle procedural macro
            quote! { #mac }
        }
        Expr::Lit(ExprLit { lit, .. }) => lit_to_node(&lit),
        _ => panic!("Unsupported input type"),
    };

    // Generate the code for the question operation
    let operation = quote! {
        gregex_logic::translation::node::Node::Operation(
            gregex_logic::translation::operator::Operator::Question,
            Box::new(#node),
            None
        )
    };

    // Generate the final token stream
    let gen = quote! {
        #operation
    };

    gen.into()
}

/// Main regex macro that builds an NFA from a pattern.
///
/// Supports three modes:
/// 1. String parsing (recommended): Parse regex syntax strings directly like `regex!("(a|b)+")`
/// 2. Nested macros: Use operator macros like `regex!(dot!(...))`
/// 3. Character literals: Simple single-character patterns like `regex!('a')`
///
/// String syntax supports: literals, `ab` (concat), `a|b` (or), `a*` (star), `a+` (plus), `a?` (question), `(...)` (grouping)
///
/// **Note**: The macro now compiles the NFA at compile-time and embeds it directly, resulting in
/// zero runtime NFA construction overhead.
#[proc_macro]
pub fn regex(input: TokenStream) -> TokenStream {
    let expr = parse_macro_input!(input as Expr);

    // Convert the input expression into a Node structure
    match expr {
        Expr::Macro(ExprMacro { mac, .. }) => {
            // Handle procedural macro - return runtime construction
            let gen = quote! {
                {
                    let regex_tree = #mac;
                    let prefix_set = gregex_logic::translation::node::prefix_set(&regex_tree);
                    let suffix_set = gregex_logic::translation::node::suffix_set(&regex_tree);
                    let factors_set = gregex_logic::translation::node::factors_set(&regex_tree);
                    let nullability_set = gregex_logic::translation::node::nullability_set(&regex_tree);
                    gregex_logic::nfa::NFA::set_to_nfa(&prefix_set, &suffix_set, &factors_set, &nullability_set)
                }
            };
            gen.into()
        }
        Expr::Lit(ExprLit { lit, .. }) => match lit {
            Lit::Char(c) => build_nfa_for_char(c.value()),
            Lit::Str(s) => build_nfa_for_string(&s.value()),
            _ => panic!("Unsupported literal type"),
        },
        _ => panic!("Unsupported input type"),
    }
}

/// Helper function to build NFA at compile time for a single character
fn build_nfa_for_char(c: char) -> TokenStream {
    use gregex_logic::translation::node::Node;
    use gregex_logic::TERMINAL_COUNT;

    // Build the node tree at compile time
    let count = TERMINAL_COUNT.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    let node = Node::Terminal(c, count);

    // Convert to NFA at compile time
    build_nfa_from_node(&node)
}

/// Helper function to build NFA at compile time for a regex string
fn build_nfa_for_string(pattern: &str) -> TokenStream {
    // Parse the regex string into a Node tree at compile time
    let node = regex_parser::parse_to_node(pattern);

    // Convert to NFA at compile time
    build_nfa_from_node(&node)
}

/// Build NFA from a node at compile time and generate code for it
fn build_nfa_from_node(node: &gregex_logic::translation::node::Node) -> TokenStream {
    use gregex_logic::nfa::NFA;
    use gregex_logic::translation::node::{factors_set, nullability_set, prefix_set, suffix_set};

    // Compute sets at compile time
    let prefix = prefix_set(node);
    let suffix = suffix_set(node);
    let factors = factors_set(node);
    let nullability = nullability_set(node);

    // Build NFA at compile time
    let nfa = NFA::set_to_nfa(&prefix, &suffix, &factors, &nullability);

    // Serialize the NFA to code
    serialize_nfa(&nfa)
}

/// Serialize an NFA to Rust code
fn serialize_nfa(nfa: &gregex_logic::nfa::NFA) -> TokenStream {
    let states = nfa.get_states();
    let accept = nfa.get_accept_states();
    let transitions = nfa.get_transitions();

    // Convert transitions to token stream
    let transition_items = transitions.iter().map(|((from, c), tos)| {
        let to_vals = tos.iter().map(|&t| quote! { #t });
        quote! { ((#from, #c), vec![#(#to_vals),*]) }
    });

    quote! {
        gregex_logic::nfa::NFA::from_raw(
            vec![#(#states),*],
            vec![#(#accept),*],
            vec![#(#transition_items),*]
        )
    }
    .into()
}
