use super::*;
use std::fs::File;
use std::io::{SeekFrom, Seek};

impl<I: Index, V: Value> Node<I,V> {

  pub fn insert_kv(&mut self, index: I, value: V) -> Result<(), String> {
    // debug!("BEFORE INSERT {:#?}", self);
    let mut i = 0;
    for kidx in 0..self.keys.len() {
      if index < self.keys[kidx] {
        break;
      }
      i += 1;
    }

    self.set_dirty();
    // debug!("INSERT_CAPACITY: {}, AT: {}",self.keys.capacity(), i);
    self.keys.insert(i, index);
    self.values.insert(i, value);
    self.increase_count();
    // debug!("AFTER INSERT {:#?}\n\n", self);
    Ok(())
  }

  pub fn insert(&mut self, node_file: &mut File, index: I, value: V) -> Result<(), String> {
    // debug!("INDEX: {:?} VALUE: {:?}", index, value);

    let node_offset = self.search_node(node_file, index).unwrap();
    let mut node = Self::read(node_file, node_offset).unwrap();

    node.insert_kv(index, value)?;
    if node.is_overflow() {
      node.split_node(node_file, None)?;
    }
    node.write(node_file).unwrap();

    node.write(node_file).unwrap();
    // debug!("INSERTED.");
    Ok(())
  }

  pub fn split_node(&mut self, node_file: &mut File, down_to_up_index: Option<I>) -> Result<(), String> {
    self.set_internal();

    let mut new_right_node = Self::new();
    let mut new_left_node = Self::new();

    let offset = node_file.seek(SeekFrom::End(0)).unwrap();
    new_right_node.set_parent_offset(self.offset);
    new_right_node.set_offset(offset + Self::get_byte_size() as u64);
    new_right_node.set_dirty();
    new_right_node.set_leaf(None);

    new_left_node.set_parent_offset(self.offset);
    new_left_node.set_offset(offset);
    new_left_node.set_dirty();
    new_left_node.set_leaf(Some(new_right_node.offset));

    new_right_node.keys = self.keys.split_off(DEGREE / 2);
    new_left_node.keys = self.keys.clone();

    new_right_node.values = self.values.split_off(DEGREE / 2);
    new_left_node.values = self.values.clone();
    new_left_node.write(node_file).unwrap();
    new_right_node.write(node_file).unwrap();


    let target_index = new_right_node.keys[0];

    self.node_type = NodeType::Internal;
    self.set_used_count(1);
    self.keys = Vec::with_capacity(DEGREE - 1);
    self.keys.push(target_index);
    self.values = Vec::with_capacity(DEGREE - 1);
    self.children = Vec::with_capacity(DEGREE);
    self.children.push(new_left_node.offset);
    self.children.push(new_right_node.offset);

    self.write(node_file).unwrap();

    debug!("Split Node: {:#?}", self);

    if !self.is_root {
      panic!("Parent 노드로 올라가서 삽입");
    }

    Ok(())

  }

  pub fn search_node(&mut self, node_file: &mut File, index: I) -> Result<u64, String> {
    if matches!(self.node_type, NodeType::Leaf(_)) {
      return Ok(self.get_offset());
    }

    let mut i: usize = 0;
    for i in 0..self.keys.len() {
      if index < self.keys[i] {
        break;
      }
    }

    if i == self.keys.len() {
      i -= 1;
    }

    let child_offset = self.children[i];

    let mut child_node = Self::read(node_file, child_offset).unwrap();
    Ok(child_node.search_node(node_file, index).unwrap())
  }

}
