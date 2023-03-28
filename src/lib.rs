#![doc = include_str!("crate-doc.md")]
#![cfg_attr(doc, deny(rustdoc::all))]
#![allow(
    clippy::missing_errors_doc, // TODO clippy::missing_errors_doc
    clippy::option_if_let_else,
    clippy::module_name_repetitions,
)]



















// -- lib.rs --

mod fm;
mod qlgl;

// -- lib.rs --






#[cfg(test)]
mod tests {
  use super::*;
  use std::path::PathBuf;
  use qlgl::Config;

  #[test]
  fn it_works() {
    let path = PathBuf::from("/tmp/test");
    let tree = Config
      ::new(path, 3)
      .build::<i64, i64>();

    assert!(true);
  }
}


