use super::{ IndexTrait, ValueTrait, RootFile };
use std::marker::PhantomData;

impl<I: IndexTrait, V: ValueTrait> RootFile<I, V> {
  #[must_use]
  pub fn new() -> Self {
    Self {
      key_size: 0,
      root_id: 0,
      tree_name: String::new(),
      name_len: 0,
      index_size: 0,
      value_size: 0,
      leaf_offsets: Vec::new(),
      _phantom: PhantomData,
    }
  }

  // NOTE : Set Root Id
  pub fn set_root_id(&mut self, root_id: i64) {
    self.root_id = root_id;
  }

  // NOTE : Set Tree Name & Name Length
  pub fn set_tree_name(&mut self, tree_name: String) {
    self.tree_name = tree_name.clone();
    self.name_len = tree_name.len() as u64;
  }

  // NOTE : Set Key Size
  pub fn set_key_size(&mut self, key_size: u64) {
    self.key_size = key_size;
  }

  // NOTE : Set Index Size
  pub fn set_index_size(&mut self, index_size: u64) {
    self.index_size = index_size;
  }

  // NOTE : Set Value Size
  pub fn set_value_size(&mut self, value_size: u64) {
    self.value_size = value_size;
  }


}

