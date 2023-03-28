use super::{ IndexTrait, ValueTrait, RootFile };
use std::marker::PhantomData;

impl<I: IndexTrait, V: ValueTrait> RootFile<I, V> {
  fn new() -> Self {
    Self {
      root_id: 0,
      tree_name: String::new(),
      name_len: 0,
      page_size: 0,
      page_count: 0,
      index_size: 0,
      value_size: 0,
      leaf_offsets: Vec::new(),
      _phantom: PhantomData,
    }
  }

}

