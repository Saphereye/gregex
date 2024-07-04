#[doc = include_str!("../README.md")]
#[cfg(not(doctest))]
pub mod nfa;
pub mod translation;

use std::sync::atomic::AtomicU32;
pub static TERMINAL_COUNT: AtomicU32 = AtomicU32::new(0);
