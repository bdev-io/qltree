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

impl<I: IndexTrait, V:ValueTrait> std::fmt::Debug for Node<I, V> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("Node")
      .field("is_root", &self.is_root)
      .field("is_leaf", &self.is_leaf)
      .field("is_dirty", &self.is_dirty)
      .field("is_overflow", &self.is_overflow)
      .field("is_underflow", &self.is_underflow)
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
  pub fn make_root(degree: usize) -> Self {
    Self {
      is_root: true,
      is_leaf: true,
      is_dirty: true,
      is_overflow: false,
      is_underflow: true,

      parent_offset: 0,
      offset: 0,

      keys: Vec::with_capacity(degree),
      values: Vec::with_capacity(degree),
      children: Vec::with_capacity(degree + 1),

      phantom: std::marker::PhantomData,

    }
  }

  pub fn make_child(&self, degree: usize, file: &mut File) -> Self {
    let mut temp = Self {
      is_root: false,
      is_leaf: true,
      is_dirty: true,
      is_overflow: false,
      is_underflow: true,

      parent_offset: self.offset,
      offset: file.seek(SeekFrom::End(0)).unwrap(),

      keys: Vec::with_capacity(degree),
      values: Vec::with_capacity(degree),
      children: Vec::with_capacity(degree + 1),

      phantom: std::marker::PhantomData,
    };
    temp.set_dirty();
    temp.write(file).unwrap();

    temp
  }

  pub fn make_from_file(degree: usize, file: &mut File, offset: u64) -> Self {
    let mut temp = Self {
      is_root: false,
      is_leaf: false,
      is_dirty: false,
      is_overflow: false,
      is_underflow: true,

      parent_offset: 0,
      offset,

      keys: Vec::with_capacity(degree),
      values: Vec::with_capacity(degree),
      children: Vec::with_capacity(degree + 1),

      phantom: std::marker::PhantomData,
    };
    temp.read(file, offset).unwrap();
    debug!("make_from_file : {:?}", temp);

   temp 
  }

  pub fn set_root(&mut self) {
    self.is_root = true;
  }

  pub fn set_leaf(&mut self) {
    self.is_leaf = true;
  }

  pub fn set_non_leaf(&mut self) {
    self.is_leaf = false;
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
    

    for i in 0..(degree) {
      if i < self.keys.len() {
        let raw = self.keys[i].to_bytes();
        bytes.extend_from_slice(&raw);
      } else {
        bytes.extend_from_slice(&vec![0u8; index_size]);
      }
    }                                                         // TYPE : (degree - 1) * index_size byte

    for i in 0..(degree) {
      if i < self.values.len() {
        let raw = self.values[i].to_bytes();
        bytes.extend_from_slice(&raw);
      } else {
        bytes.extend_from_slice(&vec![0u8; value_size]);
      }
    }                                                         // TYPE : (degree - 1) * value_size byte

    for i in 0..(degree + 1) {
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
    4 + 16 + (degree * index_size) + (degree * value_size) + ((degree + 1) * 8)
  }

  pub fn write(&mut self, node_file: &mut File) -> Result<u64> {
    if !self.is_dirty {
      return Ok(self.offset);
    }

    debug!("DIRTY NODE, Write! (OFFSET: {})", self.offset);

    node_file.seek(std::io::SeekFrom::Start(self.offset))?;
    let bytes = self.to_bytes()?;
    node_file.write_all(&bytes)?;
    node_file.flush()?;
    self.is_dirty = false;
    Ok(self.offset)
  }

  pub fn read(&mut self, file: &mut File, offset: u64) -> Result<()> {
    file.seek(std::io::SeekFrom::Start(offset))?;
    let mut bytes: Vec<u8> = vec![0u8; self.get_bytes_size()];
    file.read_exact(&mut bytes)?;
    debug!("READ_BYTES : {:?}", bytes);

    let mut cursor = std::io::Cursor::new(bytes);

    self.is_root = cursor.read_u8()? == 1;
    self.is_leaf = cursor.read_u8()? == 1;
    self.is_dirty = cursor.read_u8()? == 1;
    cursor.read_u8()?; // NOTE : 1 byte padding

    self.parent_offset = cursor.read_u64::<BigEndian>()?;
    self.offset = cursor.read_u64::<BigEndian>()?;

    let degree = self.get_degree();

    for _ in 0..(degree) {
      let mut index_bytes: Vec<u8> = vec![0u8; self.get_index_size()];
      cursor.read_exact(&mut index_bytes)?;
      self.keys.push(I::from_bytes(&index_bytes));
    }

    for _ in 0..(degree) {
      let mut value_bytes: Vec<u8> = vec![0u8; self.get_value_size()];
      cursor.read_exact(&mut value_bytes)?;
      self.values.push(V::from_bytes(&value_bytes));
    }

    for _ in 0..(degree + 1) {
      let child_offset = cursor.read_u64::<BigEndian>()?;
      self.children.push(child_offset);
    }

    debug!("READ NODE, offset : {}", offset);

    Ok(())
  }
}
// NOTE : 노드의 파일 입출력에 대한 정의


// NOTE : 노드의 데이터 추가, 삭제, 수정에 대한 정의
impl<I: IndexTrait, V:ValueTrait> Node<I, V> {

  fn check_overflow(&mut self) -> bool {
    self.is_overflow = (self.keys.len() >= self.get_degree()) || (self.values.len() >= self.get_degree());
    self.is_overflow
  }

  pub fn push_kv(&mut self, key: I, val: V) -> usize {
    self.set_dirty();
    // TODO : 적절한 자리를 찾아서 key, val를 삽입한다.
    // NOTE : 반환값은 삽입된 위치를 의미한다.

    let mut i = 0;
    while i < self.keys.len() {
      if self.keys[i] > key {
        break;
      }
      i += 1;
    }

    self.keys.insert(i, key);
    self.values.insert(i, val);
    i
  }

  pub fn split(&mut self, file: &mut File) -> Result<()> {
    if !self.check_overflow() {
      return Ok(());
    }
    let mid = self.keys.len() / 2;

    // TODO : 부모가 있다면,
    // 1. 부모에게 mid 값을 넘겨주고, self 노드를 오른쪽 노드로, left_new_node 노드를 왼쪽 노드로 설정한다.
    // 2. 만약, 부모가 없고 현재 노드가 루트 노드라면,
    //    새로운 노드를 만들고, self 노드를 루트 노드로 하여 mid 값만 갖고 왼쪽 오른쪽 노드를 만들고 부모로 설정한다. 

    if !self.is_root && self.is_leaf {
      debug!("부모가 있는 Leaf 노드를 분할함!");
      self.set_non_leaf();
      let mut left_new_node = self.make_child(self.get_degree(), file);
      left_new_node.set_leaf();

      let mut parent_node = Self::make_from_file(self.get_degree(), file, self.parent_offset);
      let parent_push_position = parent_node.push_kv(self.keys[mid], self.values[mid]);
      parent_node.children.insert(parent_push_position, left_new_node.offset);
      parent_node.children.insert(parent_push_position + 1, self.offset);

      for i in 0..mid {
        let pushed_index = left_new_node.push_kv(self.keys[i], self.values[i]);
        left_new_node.children.insert(pushed_index, self.children[i]);
      }

      for _ in 0..mid {
        self.keys.remove(0);
        self.values.remove(0);
        self.children.remove(0);
      }

      left_new_node.write(file)?; // NOTE : 왼쪽 노드 저장
      self.write(file)?; // NOTE : 오른쪽 노드 저장
      parent_node.split(file)?; // NOTE : 부모 노드 분할 해야하는지 체크하고 분할
      parent_node.write(file)?; // NOTE : 부모 노드 저장
    }

    if self.is_root {
      debug!("루트 노드를 분할함!");
      self.set_non_leaf();
      let mut left_new_node = self.make_child(self.get_degree(), file);
      left_new_node.set_leaf();

      let mut right_new_node = self.make_child(self.get_degree(), file);
      right_new_node.set_leaf();

      left_new_node.set_dirty();
      right_new_node.set_dirty();

      for i in 0..mid {
        let pushed_index = left_new_node.push_kv(self.keys[i], self.values[i]);
        if i < self.children.len() {
          left_new_node.children[pushed_index] = self.children[i];
          left_new_node.children[pushed_index + 1] = self.children[i + 1];
        }
      }

      for i in mid+1..self.keys.len() {
        let pushed_index = right_new_node.push_kv(self.keys[i], self.values[i]);
        if i < self.children.len() {
          right_new_node.children[pushed_index] = self.children[i];
          right_new_node.children[pushed_index + 1] = self.children[i + 1];
        }
      }

      self.keys = vec![self.keys[mid]];
      self.values = vec![self.values[mid]];
      self.children = vec![left_new_node.offset, right_new_node.offset];

      self.set_dirty();

      left_new_node.write(file)?; // NOTE : 왼쪽 노드 저장
      right_new_node.write(file)?; // NOTE : 오른쪽 노드 저장

      info!("ROOT: {:#?}, LEFT: {:#?}, RIGHT: {:#?}", self, left_new_node, right_new_node);
    }



    Ok(())



  }

  pub fn insert(&mut self, file: &mut File, key: I, value: V) -> Result<()> {
    self.set_dirty();
    debug!("[OFFSET: {:?}] INSERT {:#?}<{:#?}>", self.offset, key, value);
    // debug!("SIZE: {:#?}", self.get_bytes_size());

    if self.is_leaf {
      let pi = self.push_kv(key, value);
      debug!("[OFFSET: {:?}] PUSHED AT: {:#?}", self.offset, pi);
    } else {
      for i in 0..self.keys.len() {
        if self.keys[i] > key {
          debug!("CHILD: {:#?}, INDEX: {:#?} > {:#?}, OFFSET: {}", i, self.keys[i], key, self.children[i]);
          let mut child_node = Self::make_from_file(self.get_degree(), file, self.children[i]);
          debug!("CHILD_NODE: {:#?}", child_node);
          child_node.insert(file, key, value)?;
          child_node.split(file)?;
          break;
        }
      }
    }
    self.split(file)?;

    self.write(file)?;

    Ok(())
  }

  pub fn update(&mut self, key: I, value: V) -> Result<()> {
    todo!("upate")
  }
}
// NOTE : 노드의 데이터 추가, 삭제, 수정에 대한 정의
