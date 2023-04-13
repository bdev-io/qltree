use super::*;
use std::fs::File;
use std::io::{SeekFrom, Seek};

impl<I: Index, V: Value> Node<I,V> {

  pub fn insert_kv(&mut self, index: I, value: V) -> Result<(), String> {
    let mut i = 0;
    for kidx in 0..self.keys.len() {
      if index < self.keys[kidx] {
        break;
      }
      i += 1;
    }

    self.set_dirty();
    self.keys.insert(i, index);
    self.values.insert(i, value);
    self.increase_count();
    Ok(())
  }
  pub fn insert(&mut self, node_file: &mut File, index: I, value: V) -> Result<(), String> {
    // debug!("{:#?}", self);

    // B+ Tree
    // 1. 루트 노드가 리프 노드인지 확인
    // 2. 리프 노드라면, 리프 노드에 삽입
    // 3. 리프 노드가 아니라면, 리프 노드를 찾아서 삽입
    // 4. 리프 노드가 오버플로우 되었다면, 리프 노드를 분할
    // 5. 리프 노드가 오버플로우 되지 않았다면, 삽입 완료
    // 6. 리프 노드가 분할되었다면, 부모 노드에 삽입
    // 7. 부모 노드가 오버플로우 되었다면, 부모 노드를 분할
    // 8. 부모 노드가 오버플로우 되지 않았다면, 삽입 완료
    // 9. 부모 노드가 분할되었다면, 부모 노드의 부모 노드에 삽입
    // 10. 부모 노드의 부모 노드가 오버플로우 되었다면, 부모 노드의 부모 노드를 분할
    // 11. 부모 노드의 부모 노드가 오버플로우 되지 않았다면, 삽입 완료
    // 12. 부모 노드의 부모 노드가 분할되었다면, 부모 노드의 부모 노드의 부모 노드에 삽입

    let mut node: Box<&mut Self> = self.search_node(index).unwrap();

    node.insert_kv(index, value)?;
    if node.is_overflow() {
      node.split_node(node_file, None)?;
    }

    node.write(node_file).unwrap();
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
    self.keys = Vec::with_capacity(DEGREE - 1);
    self.keys.push(target_index);
    self.values = Vec::with_capacity(DEGREE - 1);
    self.children = Vec::with_capacity(DEGREE);
    self.children.push(new_left_node.offset);
    self.children.push(new_right_node.offset);

    self.write(node_file).unwrap();

    if !self.is_root {
      panic!("Parent 노드로 올라가서 삽입");
    }

    Ok(())

  }

  pub fn search_node(&mut self, index: I) -> Result<Box<&mut Self>, String> {
    if matches!(self.node_type, NodeType::Leaf(_)) {
      return Ok(Box::new(self));
    }

    Err("Not implemented".to_string())
  }

}
