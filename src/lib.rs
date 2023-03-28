#![doc = include_str!("crate-doc.md")]
#![cfg_attr(doc, deny(rustdoc::all))]
#![allow(
    clippy::missing_errors_doc, // TODO clippy::missing_errors_doc
    clippy::option_if_let_else,
    clippy::module_name_repetitions,
)]

use std::path::PathBuf;


// -- lib.rs --

mod fm;
mod qlgl;

// -- lib.rs --



#[allow(dead_code)]
fn sync_test() -> Result<(), Box<dyn std::error::Error>> {
  let path = PathBuf::from("/tmp/test");
  let _tree = qlgl::Config
    ::new(path, 3)
    .build::<i64, i64>();

  Ok(())
}

#[cfg(all(test, feature = "sync"))]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    assert!(sync_test().is_ok());
  }
}

// #[cfg(all(test, feature = "async"))]
