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

  let config = qlgl::Config::new(path, 3);
  let mut tree = config.build::<i64, i64>();
  
  tree.open()?;
  tree.insert(1, 2)?;
  tree.insert(2, 3)?;
  tree.insert(3, 4)?;
  tree.update(1, 4)?;
  tree.delete(1)?;
  tree.insert(1, 2)?;

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

