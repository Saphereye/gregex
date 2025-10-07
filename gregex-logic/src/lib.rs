//! # Gregex Logic
//!
//! Core logic library for the Gregex regular expression engine.
//!
//! This crate implements the fundamental algorithms and data structures for regular expression
//! matching using Non-deterministic Finite Automata (NFA) with Glushkov's construction algorithm.
//!
//! For detailed documentation, see the [README](https://github.com/Saphereye/gregex/blob/master/gregex-logic/README.md).

#![doc = include_str!("../README.md")]

#[cfg(not(doctest))]
pub mod nfa;
pub mod translation;

use std::sync::atomic::AtomicU32;
pub static TERMINAL_COUNT: AtomicU32 = AtomicU32::new(1);
