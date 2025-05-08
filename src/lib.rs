//! csharp_toolchain main library

//![cfg(feature = "self-rust-tokenize")]
//extern crate self_rust_tokenize;
//![cfg(feature = "self-rust-tokenize")]
//use self_rust_tokenize::SelfRustTokenize;

pub mod parser;
pub mod codegen;
pub mod compiler;
pub mod parsers;
pub mod cli; // Export the CLI module

#[cfg(test)]
mod tests {
}
