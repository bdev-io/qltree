use super::{ IndexTrait, ValueTrait };
use crate::fm::roots::RootFile;

use std::fmt::Debug;
use std::path::PathBuf;
use std::sync::Arc;


pub struct Tree<I: IndexTrait, V: ValueTrait> {
  root_dir_path: PathBuf,    // NOTE : Root Directory Path 

  root: Arc<Option<RootFile<I, V>>>, // NOTE : Multiple Ownership & ThreadSafe
}

impl<I: IndexTrait, V: ValueTrait> Debug for Tree<I, V> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("Tree")
      .field("root_dir_path", &self.root_dir_path)
      .finish()
  }
}

impl<I: IndexTrait, V: ValueTrait> Tree<I, V> {
  pub fn new(root_dir_path: PathBuf) -> Self {
    Self {
      root_dir_path,
      root: Arc::new(None),
    }
  }
}
