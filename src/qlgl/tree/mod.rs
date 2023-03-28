#![allow(dead_code)]
use super::{ IndexTrait, ValueTrait };
use crate::fm::roots::RootFile;

use std::fmt::Debug;
use std::path::PathBuf;
use std::sync::Arc;


pub struct Tree<I: IndexTrait, V: ValueTrait> {
  root_dir_path: PathBuf,    // TYPE : Root Directory Path 
  is_open: bool,             // TYPE : Is Open

  pub root: Arc<Option<RootFile<I, V>>>, // TYPE : Multiple Ownership & ThreadSafe
}

#[cfg(feature = "sync")]
mod sync;
