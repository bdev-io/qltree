#![allow(dead_code)]
use super::{ IndexTrait, ValueTrait };
use std::fmt::Debug;
use std::marker::PhantomData;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use nodes::Node;


pub struct Tree<I: IndexTrait, V: ValueTrait> {
  root_dir_path: PathBuf,    // TYPE : Root Directory Path 
  pub root: Arc<Mutex<Node<I, V>>>, // TYPE : Multiple Ownership & ThreadSafe
  phantom: PhantomData<(I, V)>,
}

impl<I: IndexTrait, V: ValueTrait> Tree<I, V> {
  pub fn new(root_dir_path: PathBuf) -> Self {
    Self {
      root_dir_path,
      root: Arc::new(Mutex::new(Node::new())),
      phantom: PhantomData,
    }
  }
}

mod nodes;

// #[cfg(feature = "sync")]
