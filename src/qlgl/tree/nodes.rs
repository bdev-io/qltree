use std::cmp;
use std::fs::File;
use std::io::{SeekFrom, Seek};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

use super::{ IndexTrait, ValueTrait };

pub struct Node<I: IndexTrait, V: ValueTrait> {
  pub is_root: bool,            // TYPE : 노드가 루트 노드인지 여부
  pub is_leaf: bool,            // TYPE : 노드가 리프 노드인지 여부
  pub is_dirty: bool,           // TYPE : 노드가 변경되었는지 여부

  offset: u64,                  // TYPE : GL(Value File) 내부 오프셋

  pub keys: Vec<I>,             // TYPE : 키 벡터
  pub values: Vec<V>,           // TYPE : 값 벡터
  pub children: Vec<u64>,       // TYPE : 자식 노드 벡터


  pub phantom: std::marker::PhantomData<(I, V)>,
}


// === 본 구현체 ===

impl<I: IndexTrait, V: ValueTrait> Node<I, V> {
  pub fn new(degree: usize) -> Self {
    Self {
      is_root: false,
      is_leaf: false,
      is_dirty: false,

      offset: 0,

      keys: Vec::with_capacity(degree - 1),
      values: Vec::with_capacity(degree - 1),
      children: Vec::with_capacity(degree),

      phantom: std::marker::PhantomData,
    }
  }

  pub fn set_root(&mut self) {
    self.is_root = true;
  }

  pub fn set_leaf(&mut self) {
    self.is_leaf = true;
  }

  pub fn set_dirty(&mut self) {
    self.is_dirty = true;
  }

}

impl<I: IndexTrait, V:ValueTrait> Node<I, V> {
  pub fn write(&self, node_file: &mut File) -> Result<u64> {
    // let mut buf = Vec::new();
    let offset = node_file.seek(SeekFrom::End(0))?;
    Ok(offset)
  }

  pub fn read(&mut self, node_file: &mut File, value_file: &mut File, offset: u64) -> Result<()> {
    todo!("read")
  }
}


impl<I: IndexTrait, V:ValueTrait> Node<I, V> {
  pub fn insert(&mut self, key: I, value: V) -> Result<()> {
    todo!("insert")
  }

  pub fn update(&mut self, key: I, value: V) -> Result<()> {
    todo!("upate")
  }

}
