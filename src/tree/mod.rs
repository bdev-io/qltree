use super::qlgl::{ IndexTrait, ValueTrait };
use super::fm::roots::RootFile;

use std::fmt::Debug;
use std::path::PathBuf;
use std::sync::Arc;


pub struct Tree<I: IndexTrait, V: ValueTrait> { {
  root_dir_path: PathBuf,

  root: Arc<RootFile<I, V>>
}

