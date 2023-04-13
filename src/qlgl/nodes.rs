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

impl<I: IndexTrait, V: ValueTrait> AsRef<Node<I, V>> for Node<I, V> {
  fn as_ref(&self) -> &Self {
    self
  }
}

impl<I: IndexTrait, V: ValueTrait> AsMut<Node<I, V>> for Node<I, V> {
  fn as_mut(&mut self) -> &mut Self {
    self
  }
}

impl<I: IndexTrait, V: ValueTrait> Clone for Node<I, V> {
  fn clone(&self) -> Self {

    let mut keys: Vec<I> = Vec::with_capacity(self.keys.capacity());
    let mut values: Vec<V> = Vec::with_capacity(self.values.capacity());
    let mut children: Vec<u64> = Vec::with_capacity(self.children.capacity());

    keys.extend_from_slice(&self.keys);
    values.extend_from_slice(&self.values);
    children.extend_from_slice(&self.children);

    Self {
      node_type: self.node_type,
      is_root: self.is_root,
      is_dirty: self.is_dirty,
      is_overflow: self.is_overflow,

      parent_offset: self.parent_offset,
      offset: self.offset,

      keys,
      values,
      children,

      phantom: std::marker::PhantomData,
    }
  }
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
        NodeType::Leaf(_) => &"Leaf",
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

  pub fn make_leaf(degree: usize, next_offset: Option<u64>) -> Self {
    Self {
      node_type: NodeType::Leaf(next_offset),
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

  pub fn read_from_file(node_file: &File, offset: u64) -> Self {
    // TODO : 파일에서 특정 부분을 읽어서 노드를 생성해주어야함.
    debug!("Read From File");

    Self {
      node_type: NodeType::Leaf(None),
      ..Default::default()
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
    self.keys.capacity() + 1
  }

  pub fn is_full(&self) -> bool {
    debug!("is_full : {} == {}", self.keys.len(), self.keys.capacity());
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
    let node_type_size = NodeType::get_size();

    let common_size: usize = node_type_size + 1 + 3 + 8 + 8 + ((degree - 1) * index_size); // NOTE : node_type(16) + is_root(1) + padding(3) + parent_offset(8) + offset(8) + (degree - 1) * index_size
    if let NodeType::Internal = self.node_type {
      common_size + (degree * 8) // NOTE : degree * children_offset(8)
    } else {
      common_size + (degree - 1) * value_size // NOTE : ((degree - 1) * value_size)
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
    if let NodeType::Leaf(_) = &self.node_type {
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
    } else if let NodeType::Leaf(next_offset) = &self.node_type {
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
    debug!("{:?}", self);
    todo!("꽉 차있어서 분할이 필요함");
    Ok(())
  }

  pub fn insert_non_full(&mut self, file: &mut File, key: I, value: V) -> Result<()> {
    // todo!("꽉 차있지 않아서 분할이 필요없음");
    Ok(())
  }

  pub fn insert_search_node(&mut self, file: &mut File, key: I) -> Result<Box<Node<I, V>>> {
    debug!("INSERT SEARCH NODE (OFFSET: {}), kc: {}", self.offset, self.keys.capacity());
    if matches!(self.node_type, NodeType::Leaf(_)) {
      return Ok(Box::new(self.clone()));
    }

    if matches!(self.node_type, NodeType::Internal) {
      if self.is_root {
        return Ok(Box::new(self.clone()));
      }
      let mut index = 0;
      for i in 0..self.keys.len() {
        if key < self.keys[i] {
          index = i;
          break;
        }
      }
      let child_offset = self.children[index];

      let mut child_node: Node<I, V> = Node::read_from_file(file, child_offset);
      let recursive_result = child_node.insert_search_node(file, key)?;
      Ok(recursive_result)
    } else {
      Err(std::io::Error::new(std::io::ErrorKind::Other, "Invalid Node Type").into())
    }
  }

  pub fn insert_kv(&mut self, file: &mut File, key: I, value: V) -> Result<()> {
    debug!("INSERT_KV: {:?}\n{}", self, self.keys.capacity());
    if self.is_full() {
      self.insert_full(file, key, value)?;
    } else {
      self.insert_non_full(file, key, value)?;
    }
    Ok(())
  }

  pub fn insert(&mut self, file: &mut File, key: I, value: V) -> Result<()> {
    if self.is_root {
      let mut target_node: Box<Node<I, V>> = self.insert_search_node(file, key)?;
      target_node.insert_kv(file, key, value)?;

      todo!("노드에 키와 값 삽입.");
    }

    todo!("노드에 키와 값 삽입.");
    Ok(())
  }

}
// NOTE : 노드의 데이터 추가, 삭제, 수정에 대한 정의
