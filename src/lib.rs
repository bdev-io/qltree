#![doc = include_str!("crate-doc.md")]
#![cfg_attr(doc, deny(rustdoc::all))]
#![allow(
    clippy::missing_errors_doc, // TODO clippy::missing_errors_doc
    clippy::option_if_let_else,
    clippy::module_name_repetitions,
)]

// fn main() {
//   let x = "";
// }







// -- lib.rs --

mod tree;
mod fm;
mod qlgl;

// -- lib.rs --






// #[cfg(test)]
// mod tests {
//   use super::*;
//   use crate::tree::Tree;
//   use std::path::PathBuf;
//
//   #[tokio::test]
//   async fn it_works() {
//     // let path = PathBuf::from("test");
//     // let tree = Tree::new(path);
//     assert!(true);
//   }
// }
//
//
