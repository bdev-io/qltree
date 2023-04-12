use std::cmp;
use std::fs::File;
use std::io::{SeekFrom, Seek, Write, Read};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

use byteorder::{ReadBytesExt, BigEndian};

use super::{ IndexTrait, ValueTrait, NodeType };

pub struct Node<I: IndexTrait, V: ValueTrait> {
  pub node_type: NodeType,      // TYPE : 노드의 타입 (Internal, Leaf)
  pub is_root: bool,            // TYPE : 노드가 루트 노드인지 여부

  pub is_dirty: bool,           // TYPE : 노드가 변경되었는지 여부
  pub is_overflow: bool,        // TYPE : 노드가 오버플로우 되었는지 여부


  parent_offset: u64,           // TYPE : 부모 노드의 오프셋
  offset: u64,                  // TYPE : GL(Value File) 내부 오프셋

  pub keys: Vec<I>,             // TYPE : 키 벡터
  pub values: Vec<V>,           // TYPE : 값 벡터
  pub children: Vec<u64>,       // TYPE : 자식 노드 벡터

  pub phantom: std::marker::PhantomData<(I, V)>,
}

impl<I: IndexTrait, V: ValueTrait> Default for Node<I, V> {
  fn default() -> Self {
    Self {
      node_type: NodeType::Internal,
      is_root: false,
      is_dirty: false,
      is_overflow: false,

      parent_offset: 0,
      offset: 0,

      keys: Vec::new(),
      values: Vec::new(),
      children: Vec::with_capacity(0),

      phantom: std::marker::PhantomData,
    }
  }
}

impl<I: IndexTrait, V:ValueTrait> std::fmt::Debug for Node<I, V> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("Node")
      .field("node_type", match self.node_type {
        NodeType::Internal => &"Internal",
        NodeType::Leaf(_, _) => &"Leaf",
      })
      .field("is_root", &self.is_root)
      .field("is_dirty", &self.is_dirty)
      .field("is_overflow", &self.is_overflow)
      .field("parent_offset", &self.parent_offset)
      .field("offset", &self.offset)
      .field("keys", &self.keys)
      .field("values", &self.values)
      .field("children", &self.children)
      .finish()
  }
}


// NOTE : 노드의 생성, 속성 부여에 대한 정의
impl<I: IndexTrait, V: ValueTrait> Node<I, V> {
  pub fn make_internal(degree: usize) -> Self {
    Self {
      node_type: NodeType::Internal,
      is_root: false,
      is_dirty: false,
      is_overflow: false,
      parent_offset: 0,
      offset: 0,
      keys: Vec::with_capacity(degree - 1),
      values: Vec::with_capacity(0),
      children: Vec::with_capacity(degree),
      phantom: std::marker::PhantomData,
    }
  }

  pub fn make_leaf(degree: usize, prev_offset: Option<u64>, next_offset: Option<u64>) -> Self {
    Self {
      node_type: NodeType::Leaf(prev_offset, next_offset),
      is_root: false,
      is_dirty: false,
      is_overflow: false,

      parent_offset: 0,
      offset: 0,
      keys: Vec::with_capacity(degree - 1),
      values: Vec::with_capacity(degree - 1),
      children: Vec::with_capacity(0),
      phantom: std::marker::PhantomData,
    }
  }

  pub fn set_root(&mut self) {
    self.is_root = true;
  }

  pub fn set_parent_offset(&mut self, offset: u64) {
    self.parent_offset = offset;
  }

  pub fn set_offset(&mut self, offset: u64) {
    self.offset = offset;
  }

  pub fn set_dirty(&mut self) {
    self.is_dirty = true;
  }

  pub fn get_degree(&self) -> usize {
    self.keys.capacity()
  }

  pub fn is_full(&self) -> bool {
    self.keys.len() == self.keys.capacity()
  }




}

// NOTE : 노드의 생성, 속성 부여에 대한 정의


// NOTE : 노드의 파일 입출력에 대한 정의
impl<I: IndexTrait, V:ValueTrait> Node<I, V> {
  pub fn get_bytes_size(&self) -> usize {
    let degree = self.get_degree();
    let index_size = I::get_size();
    let value_size = V::get_size();

    let common_size: usize = 24 + 1 + 3 + 8 + 8 + ((degree - 1) * index_size); // NOTE : node_type(24) + is_root(1) + padding(3) + parent_offset(8) + offset(8) + (degree - 1) * index_size
    if let NodeType::Internal = self.node_type {
      common_size + (degree * 8) // NOTE : degree * children_offset(8)
    } else {
      common_size + (degree - 1) * value_size + 8 + 8 // NOTE : ((degree - 1) * value_size) + prev_offset(8) + next_offset(8)
    }
  }
  // NOTE : byte_size

  pub fn to_bytes(& self) -> Result<Vec<u8>> {
    let mut bytes: Vec<u8> = vec![0u8; self.get_bytes_size()];

    let node_type_vec = self.node_type.to_bytes();            // TYPE : 24 byte
    bytes.extend_from_slice(&node_type_vec);

    bytes.push(self.is_root as u8);                           // TYPE : 1 byte
    bytes.push(0u8);                                          // TYPE : 1 byte
    bytes.push(0u8);                                          // TYPE : 1 byte
    bytes.push(0u8);                                          // TYPE : 1 byte
    // NOTE : 3 byte padding

    bytes.extend_from_slice(&self.parent_offset.to_be_bytes()); // TYPE : 8 byte
    bytes.extend_from_slice(&self.offset.to_be_bytes());      // TYPE : 8 byte


    let degree = self.get_degree();
    let index_size = I::get_size();
    let value_size = V::get_size();
    

    for i in 0..(degree) {
      if i < self.keys.len() {
        let raw = self.keys[i].to_bytes();
        bytes.extend_from_slice(&raw);
      } else {
        bytes.extend_from_slice(&vec![0u8; index_size]);
      }
    }                                                         // TYPE : (degree - 1) * index_size byte

    // NOTE : 만약 Internal Node라면, Value는 존재하지 않는다.
    if let NodeType::Leaf(_, _) = &self.node_type {
      for i in 0..(degree - 1) {
        if i < self.values.len() {
          let raw = self.values[i].to_bytes();
          bytes.extend_from_slice(&raw);
        } else {
          bytes.extend_from_slice(&vec![0u8; value_size]);
        }
      }                                                         // TYPE : (degree - 1) * value_size byte
    }

    if let NodeType::Internal = self.node_type {
      for i in 0..(degree + 1) {
        if i < self.children.len() {
          bytes.extend_from_slice(&self.children[i].to_be_bytes());
        } else {
          bytes.extend_from_slice(&0u64.to_be_bytes());
        }
      }                                                       // TYPE : degree * 8 byte
    } else if let NodeType::Leaf(prev_offset, next_offset) = &self.node_type {
        bytes.extend_from_slice(&prev_offset.unwrap_or(0).to_be_bytes());
        bytes.extend_from_slice(&next_offset.unwrap_or(0).to_be_bytes());
      }

    Ok(bytes)
  }



  pub fn write_to_file(&mut self, file: &mut File) -> Result<u64> {
    if !self.is_dirty {
      return Ok(self.offset);
    }

    debug!("DIRTY NODE, Write! (OFFSET: {})", self.offset);

    file.seek(std::io::SeekFrom::Start(self.offset))?;
    let bytes = self.to_bytes()?;
    file.write_all(&bytes)?;
    file.flush()?;
    self.is_dirty = false;
    Ok(self.offset)
  } // NOTE : NODE_FILE OR DATA_FILE


}
// NOTE : 노드의 파일 입출력에 대한 정의


// NOTE : 노드의 데이터 추가, 삭제, 수정에 대한 정의
impl<I: IndexTrait, V:ValueTrait> Node<I, V> {

  pub fn divide(&mut self, file: &mut File) -> Result<()> {
    todo!("분할이 필요함");
    Ok(())
  }

  pub fn insert_full(&mut self, file: &mut File, key: I, value: V) -> Result<()> {
    todo!("꽉 차있어서 분할이 필요함");
    Ok(())
  }

  pub fn insert_non_full(&mut self, file: &mut File, key: I, value: V) -> Result<()> {
    // todo!("꽉 차있지 않아서 분할이 필요없음");


    Ok(())
  }

  pub fn insert(&mut self, file: &mut File, key: I, value: V) -> Result<()> {
    todo!("1. ")
  }

}
// NOTE : 노드의 데이터 추가, 삭제, 수정에 대한 정의
