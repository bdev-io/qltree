use std::fs::File;
use std::io::{SeekFrom, Seek, Write};

use crate::PAGE_SIZE;

use super::*;

impl<I: Index, V: Value> BytesExtension for Node<I, V> {
  fn get_byte_size() -> usize {
    let mut size = 0;
    size += NodeType::get_byte_size();
    size += bool::get_byte_size();
    size += bool::get_byte_size();
    size += bool::get_byte_size();

    size += usize::get_byte_size();
    size += usize::get_byte_size();

    size += Vec::<I>::get_byte_size();
    size += Vec::<V>::get_byte_size();
    size += Vec::<u64>::get_byte_size() + usize::get_byte_size(); // NOTE : Children의 경우 DEGREE 만큼임.

    ((size / PAGE_SIZE) + 1) * PAGE_SIZE // NOTE : 페이지 단위로 맞춰줌.
  }
  fn to_bytes(&self) -> Vec<u8> {
    let mut bytes: Vec<u8> = vec![];
    bytes.extend_from_slice(&self.node_type.to_bytes());
    bytes.extend_from_slice(&self.is_root.to_bytes());
    bytes.extend_from_slice(&self.is_dirty.to_bytes());
    bytes.extend_from_slice(&self.is_overflow.to_bytes());

    bytes.extend_from_slice(&self.parent_offset.to_bytes());
    bytes.extend_from_slice(&self.offset.to_bytes());


    bytes.extend_from_slice(&self.keys.to_bytes());
    bytes.extend_from_slice(&self.values.to_bytes());
    bytes.extend_from_slice(&self.children.to_bytes());

    bytes.resize(Self::get_byte_size(), 0); // NOTE : 페이지 단위로 맞춰줌.

    bytes
  }

  fn from_bytes(&mut self, bytes: Vec<u8>) -> Result<(), String> {
    if bytes.len() != Self::get_byte_size() {
      return Err(format!("Invalid byte size: {}", bytes.len()));
    }

    let mut cursor = 0;
    let mut bytes = bytes;

    let node_type = NodeType::new_from_bytes(bytes[cursor..cursor + NodeType::get_byte_size()].to_vec())?;
    cursor += NodeType::get_byte_size();

    let is_root = bool::new_from_bytes(bytes[cursor..cursor + bool::get_byte_size()].to_vec())?;
    cursor += bool::get_byte_size();

    let is_dirty = bool::new_from_bytes(bytes[cursor..cursor + bool::get_byte_size()].to_vec())?;
    cursor += bool::get_byte_size();

    let is_overflow = bool::new_from_bytes(bytes[cursor..cursor + bool::get_byte_size()].to_vec())?;
    cursor += bool::get_byte_size();

    let parent_offset = u64::new_from_bytes(bytes[cursor..cursor + u64::get_byte_size()].to_vec())?;
    cursor += u64::get_byte_size();

    let offset = u64::new_from_bytes(bytes[cursor..cursor + u64::get_byte_size()].to_vec())?;
    cursor += u64::get_byte_size();

    let keys = Vec::<I>::new_from_bytes(bytes[cursor..cursor + Vec::<I>::get_byte_size()].to_vec())?;
    cursor += Vec::<I>::get_byte_size();

    let values = Vec::<V>::new_from_bytes(bytes[cursor..cursor + Vec::<V>::get_byte_size()].to_vec())?;
    cursor += Vec::<V>::get_byte_size();

    let children = Vec::<u64>::new_from_bytes(bytes[cursor..cursor + Vec::<u64>::get_byte_size() + u64::get_byte_size()].to_vec())?;
    cursor += Vec::<u64>::get_byte_size() + u64::get_byte_size();

    self.node_type = node_type;
    self.is_root = is_root;
    self.is_dirty = is_dirty;
    self.is_overflow = is_overflow;
    self.parent_offset = parent_offset;
    self.offset = offset;
    self.keys = keys;
    self.values = values;
    self.children = children;

    Ok(())
  }

  fn new_from_bytes(bytes: Vec<u8>) -> Result<Self, String> {
    let mut new_node = Self::new();
    new_node.from_bytes(bytes)?;
    Ok(new_node)
  }
}

impl<I: Index, V: Value> Node<I,V> {
  pub fn write(&mut self, file: &mut File) -> Result<(), Box<std::io::Error>> {
    file.seek(SeekFrom::Start(self.offset))?;
    file.write_all(&self.to_bytes())?;
    self.set_clean();
    Ok(())
  }
}
