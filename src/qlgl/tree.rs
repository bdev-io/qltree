#![allow(dead_code)]
use super::{ IndexTrait, ValueTrait };
use std::fmt::Debug;
use std::fs::{ File, OpenOptions };
use std::marker::PhantomData;
use std::path::PathBuf;
use std::sync::{ Arc, Mutex };

use super::nodes::Node;


pub struct Tree<I: IndexTrait, V: ValueTrait> {
  base_path: PathBuf,                         // TYPE : 트리의 기본 경로
  node_name: String,                          // TYPE : 트리의 이름

  is_initialized: bool,                       // TYPE : 트리가 초기화 되었는지 여부(open?)

  degree: usize,                              // TYPE : 트리의 최대 차수 (최소차수 * 2 - 1), 홀수임
  root: Arc<Mutex<Node<I, V>>>,               // TYPE : 트리의 루트 노드

  node_file: Option<Arc<File>>,               // TYPE : 노드 파일
  data_file: Option<Arc<File>>,               // TYPE : 데이터 파일

  phantom: PhantomData<(I, V)>,               // TYPE : 트리의 인덱스와 값의 타입을 저장
}

impl<I: IndexTrait, V: ValueTrait> Drop for Tree<I, V> {
  fn drop(&mut self) {
    self.is_initialized = false;
    if let Some(f) = &self.node_file {
      f.sync_all().unwrap();
    }
    if let Some(f) = &self.data_file {
      f.sync_all().unwrap();
    }
    self.node_file = None;
    self.data_file = None;
  }
}

impl<I: IndexTrait, V:ValueTrait> Default for Tree<I, V> {
  fn default() -> Self {
    Self {
      base_path: PathBuf::new(),
      node_name: String::new(),

      is_initialized: false,
      degree: 0,
      root: Arc::new(Mutex::new(Node::<I, V>::default())),

      node_file: None,
      data_file: None,

      phantom: PhantomData,
    }
  }
}




// NOTE : 트리의 파일입출력및 기본 메소드
impl<I: IndexTrait, V: ValueTrait> Tree<I, V> {
  pub fn new(base_path: PathBuf, node_name: &str, degree: usize) -> Self {

    let degree = degree * 2 - 1;

    let mut root = Node::<I, V>::make_internal(degree);
    root.set_root();
    root.set_dirty();
    debug!("Root Node Created: {:?}, kc: {}", root, root.keys.capacity());

    Self {
      base_path,
      node_name: node_name.to_string(),
      degree,
      root: Arc::new(Mutex::new(root)),
      phantom: PhantomData,
      node_file: None,
      data_file: None,
      ..Default::default()
    }
  }

  fn sync(&self) {
    if let Some(f) = &self.node_file {
      f.sync_all().unwrap();
    }
    if let Some(f) = &self.data_file {
      f.sync_all().unwrap();
    }
  }

  pub fn open(&mut self) {
    let node_path = self.base_path.join(format!("{}.ql", self.node_name));
    let data_path = self.base_path.join(format!("{}.gl", self.node_name));

    if self.node_file.is_none() {
      if node_path.exists() {
        let node_file = OpenOptions::new().write(true).read(true).append(true).open(node_path).unwrap();
        let _root = self.root.lock().unwrap();
        // root.read_from_file(&mut node_file, 0_u64).unwrap();
        self.node_file = Some(Arc::new(node_file));
      } else {
        debug!("File Not Exists, Create New One");
        let node_file = OpenOptions::new().write(true).read(true).create(true).open(node_path).unwrap();
        self.node_file = Some(Arc::new(node_file));
      }

      if data_path.exists() {
        let data_file = OpenOptions::new().write(true).read(true).append(true).open(data_path).unwrap();
        self.data_file = Some(Arc::new(data_file));
      } else {
        let data_file = OpenOptions::new().write(true).read(true).create(true).open(data_path).unwrap();
        self.data_file = Some(Arc::new(data_file));
      }
    }

    self.is_initialized = true;
  }

  pub fn get_node_file(&self) -> Arc<File> {
    self.node_file.as_ref().unwrap().clone()
  }

  pub fn get_data_file(&self) -> Arc<File> {
    self.node_file.as_ref().unwrap().clone()
  }
}

// NOTE : 트리의 파일입출력및 기본 메소드


// NOTE : 트리의 추가 / 삭제 / 변경등의 메소드
impl<I: IndexTrait, V: ValueTrait> Tree<I, V> {
  pub fn insert(&mut self, key: I, value: V) -> Result<(), std::io::Error> {
    if !self.is_initialized {
      self.open();
    }

    let mut node_file = self.get_node_file().try_clone().unwrap();

    if self.root.lock().unwrap().insert(&mut node_file, key, value).is_ok() {
      Ok(())
    } else {
      Err(std::io::Error::new(std::io::ErrorKind::Other, "Insert failed"))
    }
  }

  // pub fn search(&self, key: I) -> Option<V> {
  //   None
  // }
}
// NOTE : 트리의 추가 / 삭제 / 변경등의 메소드


// #[cfg(feature = "sync")]
