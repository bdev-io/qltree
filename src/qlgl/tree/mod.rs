#![allow(dead_code)]
use super::{ IndexTrait, ValueTrait };
use crate::fm::roots::RootFile;

use std::fmt::Debug;
use std::marker::PhantomData;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};


pub struct Tree<I: IndexTrait, V: ValueTrait> {
  root_dir_path: PathBuf,    // TYPE : Root Directory Path 
  is_open: bool,             // TYPE : Is Open

  pub root: Arc<Mutex<Option<RootFile<I, V>>>>, // TYPE : Multiple Ownership & ThreadSafe
  phantom: PhantomData<(I, V)>,
}

#[cfg(feature = "sync")]
mod sync;
