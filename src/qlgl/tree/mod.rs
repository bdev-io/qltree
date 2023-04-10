#![allow(dead_code)]
use super::{ IndexTrait, ValueTrait };
use std::fmt::Debug;
use std::fs::{ File, OpenOptions };
use std::marker::PhantomData;
use std::path::PathBuf;
use std::sync::{ Arc, Mutex };

use nodes::Node;


pub struct Tree<I: IndexTrait, V: ValueTrait> {
  base_path: PathBuf,             // TYPE : 트리의 기본 경로
  node_name: String,              // TYPE : 트리의 이름

  is_initialized: bool,           // TYPE : 트리가 초기화 되었는지 여부(open?)

  degree: usize,                  // TYPE : 트리의 차수 ( 홀수여야함, I,V 개수는 degree-1, C 개수는 degree)
  root: Arc<Mutex<Node<I, V>>>,               // TYPE : 트리의 루트 노드

  node_file: Option<Arc<File>>,        // TYPE : 노드 파일
  value_file: Option<Arc<File>>,       // TYPE : 값 파일

  phantom: PhantomData<(I, V)>,  // TYPE : 트리의 인덱스와 값의 타입을 저장
}

impl<I: IndexTrait, V: ValueTrait> Drop for Tree<I, V> {
  fn drop(&mut self) {
    self.is_initialized = false;
    self.node_file = None;
    self.value_file = None;
  }
}






impl<I: IndexTrait, V: ValueTrait> Tree<I, V> {
  pub fn new(base_path: PathBuf, node_name: &str, degree: usize) -> Self {

    assert!(degree % 2 == 1, "Degree must be odd number");

    let mut root = Node::<I, V>::new(degree);
    root.set_root();
    root.set_leaf();
    root.set_dirty();

    Self {
      base_path,
      node_name: node_name.to_string(),

      is_initialized: false,
      degree,
      root: Arc::new(Mutex::new(root)),

      node_file: None,
      value_file: None,

      phantom: PhantomData,

    }
  }

  pub fn open(&mut self) {
    if self.node_file.is_none() {
      let file = OpenOptions::new().write(true).read(true).create(true).open(self.base_path.join(format!("{}.ql", self.node_name))).unwrap();
      self.node_file = Some(Arc::new(file));
    }
    if self.value_file.is_none() {
      let file = OpenOptions::new().write(true).read(true).create(true).open(self.base_path.join(format!("{}.gl", self.node_name))).unwrap();
      self.value_file = Some(Arc::new(file));
    }
    self.is_initialized = true;
  }

  pub fn get_node_file(&self) -> Arc<File> {
    self.node_file.as_ref().unwrap().clone()
  }

  pub fn get_value_file(&self) -> Arc<File> {
    self.value_file.as_ref().unwrap().clone()
  }

  pub fn insert(&mut self, key: I, value: V) -> Result<(), std::io::Error> {
    if !self.is_initialized {
      self.open();
    }

    let mut root = self.root.lock().unwrap();
    if root.insert(key, value).is_ok() {
      Ok(())
    } else {
      Err(std::io::Error::new(std::io::ErrorKind::Other, "insert error"))
    }
  }

  pub fn search(&self, key: I) -> Option<V> {
    None
  }

}


mod nodes;

// #[cfg(feature = "sync")]
