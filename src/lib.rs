//! csharp_toolchain main library

//![cfg(feature = "self-rust-tokenize")]
//extern crate self_rust_tokenize;
//![cfg(feature = "self-rust-tokenize")]
//use self_rust_tokenize::SelfRustTokenize;

pub mod syntax;
pub mod codegen;
pub mod compiler;
pub mod parser;
pub mod cli;
pub mod analysis;
// Export the CLI module
