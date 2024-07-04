#[doc = include_str!("../README.md")]
#[cfg(not(doctest))]
extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Expr, ExprLit, ExprMacro, Lit};

#[proc_macro]
pub fn dot(input: TokenStream) -> TokenStream {
    let inputs = parse_macro_input!(input with syn::punctuated::Punctuated::<Expr, syn::Token![,]>::parse_terminated);

    let nodes = inputs.iter().map(|expr| {
        match expr {
            Expr::Macro(ExprMacro { mac, .. }) => {
                // Handle procedural macro
                quote! { #mac }
            }
            Expr::Lit(ExprLit { lit, .. }) => match lit {
                Lit::Char(c) => {
                    let count = gregex_logic::TERMINAL_COUNT
                        .fetch_add(1, core::sync::atomic::Ordering::SeqCst);
                    quote! {
                        gregex_logic::translation::node::Node::Terminal(#c, #count)
                    }
                }
                _ => panic!("Unsupported literal type"),
            },
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

#[proc_macro]
pub fn or(input: TokenStream) -> TokenStream {
    let inputs = parse_macro_input!(input with syn::punctuated::Punctuated::<Expr, syn::Token![,]>::parse_terminated);

    let nodes = inputs.iter().map(|expr| {
        match expr {
            Expr::Macro(ExprMacro { mac, .. }) => {
                // Handle procedural macro
                quote! { #mac }
            }
            Expr::Lit(ExprLit { lit, .. }) => match lit {
                Lit::Char(c) => {
                    let count = gregex_logic::TERMINAL_COUNT
                        .fetch_add(1, core::sync::atomic::Ordering::SeqCst);
                    quote! {
                        gregex_logic::translation::node::Node::Terminal(#c, #count)
                    }
                }
                _ => panic!("Unsupported literal type"),
            },
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

#[proc_macro]
pub fn star(input: TokenStream) -> TokenStream {
    let expr = parse_macro_input!(input as Expr);

    let node = match expr {
        Expr::Macro(ExprMacro { mac, .. }) => {
            // Handle procedural macro
            quote! { #mac }
        }
        Expr::Lit(ExprLit { lit, .. }) => match lit {
            Lit::Char(c) => {
                let count =
                    gregex_logic::TERMINAL_COUNT.fetch_add(1, core::sync::atomic::Ordering::SeqCst);
                quote! {
                    gregex_logic::translation::node::Node::Terminal(#c, #count)
                }
            }
            _ => panic!("Unsupported literal type"),
        },
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

#[proc_macro]
pub fn regex(input: TokenStream) -> TokenStream {
    let expr = parse_macro_input!(input as Expr);

    // Convert the input expression into a Node structure
    let node = match expr {
        Expr::Macro(ExprMacro { mac, .. }) => {
            // Handle procedural macro
            quote! { #mac }
        }
        Expr::Lit(ExprLit { lit, .. }) => match lit {
            Lit::Char(c) => {
                let count =
                    gregex_logic::TERMINAL_COUNT.fetch_add(1, core::sync::atomic::Ordering::SeqCst);
                quote! {
                    gregex_logic::translation::node::Node::Terminal(#c, #count)
                }
            }
            _ => panic!("Unsupported literal type"),
        },
        _ => panic!("Unsupported input type"),
    };

    // Generate the code to convert the Node into a Regex
    let gen = quote! {
        {
            let regex_tree = #node;
            let prefix_set = gregex_logic::translation::node::prefix_set(&regex_tree);
            let suffix_set = gregex_logic::translation::node::suffix_set(&regex_tree);
            let factors_set = gregex_logic::translation::node::factors_set(&regex_tree);
            gregex_logic::nfa::NFA::set_to_nfa(&prefix_set, &suffix_set, &factors_set)
        }
    };

    gen.into()
}
