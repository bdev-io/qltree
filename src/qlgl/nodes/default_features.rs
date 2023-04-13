use super::*;


impl<I: Index, V: Value> Node<I, V> {
  pub fn new() -> Self {
    Self {
      node_type: NodeType::Internal,
      is_root: false,
      is_dirty: false,
      is_overflow: false,
      parent_offset: 0,
      offset: 0,
      used_count: 0,
      keys: Vec::new(),
      values: Vec::new(),
      children: Vec::new(),
      phantom: std::marker::PhantomData,
    }
  }

  pub fn set_leaf(&mut self, next_offset: Option<u64>) {
    self.node_type = NodeType::Leaf(next_offset);
  }

  pub fn set_internal(&mut self) {
    self.node_type = NodeType::Internal;
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

  pub fn set_used_count(&mut self, count: usize) {
    self.used_count = count;
  }

  pub fn set_dirty(&mut self) {
    self.is_dirty = true;
  }

  pub fn set_overflow(&mut self) {
    self.is_overflow = true;
  }

  pub fn is_root(&self) -> bool {
    self.is_root
  }

  pub fn is_dirty(&self) -> bool {
    self.is_dirty
  }

  pub fn is_overflow(&self) -> bool {
    self.is_overflow
  }

  pub fn get_parent_offset(&self) -> u64 {
    self.parent_offset
  }

  pub fn get_offset(&self) -> u64 {
    self.offset
  }

  pub fn get_used_count(&self) -> usize {
    self.used_count
  }

  pub fn get_node_type(&self) -> NodeType {
    self.node_type
  }

  pub fn get_keys(&self) -> &Vec<I> {
    &self.keys
  }

  pub fn get_values(&self) -> &Vec<V> {
    &self.values
  }

  pub fn get_children(&self) -> &Vec<u64> {
    &self.children
  }
}

