use std::cmp;
use std::fs::File;
use std::io::{SeekFrom, Seek, Write, Read};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

use byteorder::{ReadBytesExt, BigEndian};

use super::{ IndexTrait, ValueTrait };

pub struct Node<I: IndexTrait, V: ValueTrait> {
  pub is_root: bool,            // TYPE : 노드가 루트 노드인지 여부
  pub is_leaf: bool,            // TYPE : 노드가 리프 노드인지 여부
  pub is_dirty: bool,           // TYPE : 노드가 변경되었는지 여부
  pub is_overflow: bool,        // TYPE : 노드가 오버플로우 되었는지 여부
  pub is_underflow: bool,       // TYPE : 노드가 언더플로우 되었는지 여부


  parent_offset: u64,           // TYPE : 부모 노드의 오프셋
  offset: u64,                  // TYPE : GL(Value File) 내부 오프셋

  pub keys: Vec<I>,             // TYPE : 키 벡터
  pub values: Vec<V>,           // TYPE : 값 벡터
  pub children: Vec<u64>,       // TYPE : 자식 노드 벡터

  pub phantom: std::marker::PhantomData<(I, V)>,
}


// NOTE : 노드의 생성, 속성 부여에 대한 정의
impl<I: IndexTrait, V: ValueTrait> Node<I, V> {
  pub fn make_root(degree: usize) -> Self {
    Self {
      is_root: true,
      is_leaf: true,
      is_dirty: true,
      is_overflow: false,
      is_underflow: true,

      parent_offset: 0,
      offset: 0,

      keys: Vec::with_capacity(degree - 1),
      values: Vec::with_capacity(degree - 1),
      children: Vec::with_capacity(degree),

      phantom: std::marker::PhantomData,

    }
  }

  pub fn make_child(&self, degree: usize, file: &mut File) -> Self {
    Self {
      is_root: false,
      is_leaf: true,
      is_dirty: true,
      is_overflow: false,
      is_underflow: true,

      parent_offset: self.offset,
      offset: file.seek(SeekFrom::End(0)).unwrap(),

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


  pub fn get_index_size(&self) -> usize {
    I::get_size() as usize
  }

  pub fn get_value_size(&self) -> usize {
    I::get_size() as usize
  }

  pub fn get_degree(&self) -> usize {
    self.children.capacity()
  }

}

// NOTE : 노드의 생성, 속성 부여에 대한 정의


// NOTE : 노드의 파일 입출력에 대한 정의
impl<I: IndexTrait, V:ValueTrait> Node<I, V> {
  pub fn to_bytes(& self) -> Result<Vec<u8>> {
    let mut bytes: Vec<u8> = vec![0u8; self.get_bytes_size()];

    bytes.push(self.is_root as u8);                           // TYPE : 1 byte
    bytes.push(self.is_leaf as u8);                           // TYPE : 1 byte
    bytes.push(0u8);                                          // TYPE : 1 byte
    bytes.push(0u8);                                          // TYPE : 1 byte
    // NOTE : 1 byte padding

    bytes.extend_from_slice(&self.parent_offset.to_be_bytes()); // TYPE : 8 byte
    bytes.extend_from_slice(&self.offset.to_be_bytes());      // TYPE : 8 byte


    let degree = self.get_degree();
    let index_size = self.get_index_size();
    let value_size = self.get_value_size();
    

    for i in 0..(degree - 1) {
      if i < self.keys.len() {
        let raw = self.keys[i].to_bytes();
        bytes.extend_from_slice(&raw);
      } else {
        bytes.extend_from_slice(&vec![0u8; index_size]);
      }
    }                                                         // TYPE : (degree - 1) * index_size byte

    for i in 0..(degree - 1) {
      if i < self.values.len() {
        let raw = self.values[i].to_bytes();
        bytes.extend_from_slice(&raw);
      } else {
        bytes.extend_from_slice(&vec![0u8; value_size]);
      }
    }                                                         // TYPE : (degree - 1) * value_size byte

    for i in 0..degree {
      if i < self.children.len() {
        bytes.extend_from_slice(&self.children[i].to_be_bytes());
      } else {
        bytes.extend_from_slice(&0u64.to_be_bytes());
      }
    }                                                         // TYPE : degree * 8 byte


    Ok(bytes)
  }


  pub fn get_bytes_size(&self) -> usize {
    let degree = self.get_degree();
    let index_size = self.get_index_size();
    let value_size = self.get_value_size();
    4 + 16 + ((degree - 1) * index_size) + ((degree - 1) * value_size) + (degree * 8)
  }

  pub fn write(&mut self, node_file: &mut File) -> Result<u64> {
    node_file.seek(std::io::SeekFrom::Start(self.offset))?;
    let bytes = self.to_bytes()?;
    node_file.write_all(&bytes)?;
    self.is_dirty = false;
    Ok(self.offset)
  }

  pub fn read(&mut self, file: &mut File, offset: u64) -> Result<()> {
    file.seek(std::io::SeekFrom::Start(offset))?;
    let mut bytes: Vec<u8> = vec![0u8; self.get_bytes_size()];
    file.read_exact(&mut bytes)?;

    let mut cursor = std::io::Cursor::new(bytes);

    self.is_root = cursor.read_u8()? == 1;
    self.is_leaf = cursor.read_u8()? == 1;
    self.is_dirty = cursor.read_u8()? == 1;
    cursor.read_u8()?; // NOTE : 1 byte padding

    self.parent_offset = cursor.read_u64::<BigEndian>()?;
    self.offset = cursor.read_u64::<BigEndian>()?;

    let degree = self.get_degree();

    for _ in 0..(degree - 1) {
      let mut index_bytes: Vec<u8> = vec![0u8; self.get_index_size()];
      cursor.read_exact(&mut index_bytes)?;
      self.keys.push(I::from_bytes(&index_bytes));
    }

    for _ in 0..(degree - 1) {
      let mut value_bytes: Vec<u8> = vec![0u8; self.get_value_size()];
      cursor.read_exact(&mut value_bytes)?;
      self.values.push(V::from_bytes(&value_bytes));
    }

    for _ in 0..degree {
      let child_offset = cursor.read_u64::<BigEndian>()?;
      self.children.push(child_offset);
    }

    Ok(())
  }
}
// NOTE : 노드의 파일 입출력에 대한 정의


// NOTE : 노드의 데이터 추가, 삭제, 수정에 대한 정의
impl<I: IndexTrait, V:ValueTrait> Node<I, V> {
  pub fn insert(&mut self, file: &mut File, key: I, value: V) -> Result<()> {
    debug!("INSERT {:#?}<{:#?}>", key, value);
    debug!("SIZE: {:#?}", self.get_bytes_size());

    self.keys.push(key);
    self.values.push(value);
    self.set_dirty();

    if self.keys.len() >= self.get_degree() - 1 {
      self.is_overflow = true;
      debug!("OVERFLOW SETTED.");
    }

    

    self.write(file)?;

    Ok(())
  }

  pub fn update(&mut self, key: I, value: V) -> Result<()> {
    todo!("upate")
  }
}
// NOTE : 노드의 데이터 추가, 삭제, 수정에 대한 정의
