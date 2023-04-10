#![allow(dead_code)]
use super::{ IndexTrait, ValueTrait };
use std::fmt::Debug;
use std::fs::{ File, OpenOptions };
use std::marker::PhantomData;
use std::path::PathBuf;
use std::sync::{ Arc, Mutex };

use super::nodes::Node;


pub struct Tree<I: IndexTrait, V: ValueTrait> {
  base_path: PathBuf,             // TYPE : 트리의 기본 경로
  node_name: String,              // TYPE : 트리의 이름

  is_initialized: bool,           // TYPE : 트리가 초기화 되었는지 여부(open?)

  degree: usize,                  // TYPE : 트리의 차수 ( 홀수여야함, I,V 개수는 degree-1, C 개수는 degree)
  root: Arc<Mutex<Node<I, V>>>,               // TYPE : 트리의 루트 노드

  file: Option<Arc<File>>,        // TYPE : 노드 파일

  phantom: PhantomData<(I, V)>,  // TYPE : 트리의 인덱스와 값의 타입을 저장
}

impl<I: IndexTrait, V: ValueTrait> Drop for Tree<I, V> {
  fn drop(&mut self) {
    self.is_initialized = false;
    if let Some(f) = &self.file {
      f.sync_all().unwrap();
    }
    self.file = None;
  }
}






// NOTE : 트리의 파일입출력및 기본 메소드
impl<I: IndexTrait, V: ValueTrait> Tree<I, V> {
  pub fn new(base_path: PathBuf, node_name: &str, degree: usize) -> Self {

    assert!(degree % 2 == 1, "Degree must be odd number");

    let root = Node::<I, V>::make_root(degree);

    Self {
      base_path,
      node_name: node_name.to_string(),

      is_initialized: false,
      degree,
      root: Arc::new(Mutex::new(root)),

      file: None,

      phantom: PhantomData,

    }
  }

  pub fn open(&mut self) {
    let file_path = self.base_path.join(format!("{}.ql", self.node_name));
    if self.file.is_none() {
      if file_path.exists() {
        debug!("FILE Exists.");
        let mut file = OpenOptions::new().write(true).read(true).append(true).open(file_path).unwrap();
        let mut root = self.root.lock().unwrap();
        root.read(&mut file, 0_u64).unwrap();
        // debug!("File Read, root: {:#?}", root.keys);

        self.file = Some(Arc::new(file));
      } else {
        debug!("File Not Exists, Crate New One");
        let file = OpenOptions::new().write(true).read(true).create(true).open(file_path).unwrap();
        self.file = Some(Arc::new(file));
      }
    }

    self.is_initialized = true;
  }

  pub fn get_file(&self) -> Arc<File> {
    self.file.as_ref().unwrap().clone()
  }

}
// NOTE : 트리의 파일입출력및 기본 메소드


// NOTE : 트리의 추가 / 삭제 / 변경등의 메소드
impl<I: IndexTrait, V: ValueTrait> Tree<I, V> {
    pub fn insert(&mut self, key: I, value: V) -> Result<(), std::io::Error> {
    if !self.is_initialized {
      self.open();
    }

    let mut file = self.get_file().try_clone().unwrap();

    if self.root.lock().unwrap().insert(&mut file, key, value).is_ok() {
      Ok(())
    } else {
      Err(std::io::Error::new(std::io::ErrorKind::Other, "Insert failed"))
    }
  }

  pub fn search(&self, key: I) -> Option<V> {
    None
  }
}
// NOTE : 트리의 추가 / 삭제 / 변경등의 메소드


// #[cfg(feature = "sync")]
