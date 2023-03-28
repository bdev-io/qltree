#![allow(dead_code)]

use std::path::PathBuf;
use super::{IndexTrait, ValueTrait};
use super::tree::Tree;

#[derive(Debug)]
#[must_use]
pub struct Config {
  dir_path: PathBuf,
  key_size: u64,
}


impl Config {
  pub fn new(dir_path: PathBuf, key_size: u64) -> Self {
    if ( dir_path.is_dir() || !dir_path.exists() ) && key_size > 0 && key_size % 2 == 1 {
      if !dir_path.exists() {
        std::fs::create_dir_all(&dir_path).unwrap();
      }
      Self {
        dir_path,
        key_size,
      }
    } else {
      if key_size == 0 {
        panic!("Key Size must be greater than 0 and ODD NUMBER");
      } else if key_size % 2 == 0 {
        panic!("Key Size must be ODD NUMBER");
      } else if dir_path.is_file() {
        panic!("{} is not a directory", dir_path.display());
      }
      panic!("ERRRRRRRR");
    }
  }

  // TODO : Load Configure from dir_path
  pub fn load(_dir_path: PathBuf) -> Self {
    todo!("Load Configure from dir Path")
  }

  pub fn get_dir_path(&self) -> &PathBuf {
    &self.dir_path
  }

  // TODO : Set Key Size
  pub fn set_key_size(&mut self, _key_size: usize) {
    todo!("Set Key Size")
  }
}
// NOTE : Building Configure


impl Config {
  pub fn build<I: IndexTrait, V: ValueTrait>(&self) -> Tree<I, V> {
    Tree::<I, V>::new(self.dir_path.clone())
  }
}
