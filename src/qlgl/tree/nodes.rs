use super::{ IndexTrait, ValueTrait };

const DEGREE: usize = 3;

pub struct Node<I: IndexTrait, V: ValueTrait> {

  pub keys: Vec<I>,
  pub values: Vec<V>,
  pub children: Vec<Node<I, V>>,

  pub is_leaf: bool,
  pub is_root: bool,


  pub phantom: std::marker::PhantomData<(I, V)>,
}



impl<I: IndexTrait, V: ValueTrait> Default for Node<I, V> {
  fn default() -> Self {
    Self::new()
  }
}


// === 본 구현체 ===

impl<I: IndexTrait, V: ValueTrait> Node<I, V> {
  pub fn new() -> Self {
    Self {
      keys: Vec::new(),
      values: Vec::new(),
      children: Vec::new(),
      is_leaf: true,
      is_root: false,
      phantom: std::marker::PhantomData,
    }
  }

  pub fn is_full(&self) -> bool {
    self.keys.len() == DEGREE - 1
  }

  pub fn is_empty(&self) -> bool {
    self.keys.is_empty()
  }

  pub fn is_underflow(&self) -> bool {
    self.keys.len() < (DEGREE - 1) / 2
  }

  pub fn is_overflow(&self) -> bool {
    self.keys.len() > DEGREE - 1
  }

  pub fn is_root(&self) -> bool {
    self.is_root
  }

  pub fn is_leaf(&self) -> bool {
    self.is_leaf
  }

  pub fn is_internal(&self) -> bool {
    !self.is_leaf
  }

  pub fn is_valid(&self) -> bool {
    self.keys.len() == self.values.len()
  }

  pub fn append(&mut self, key: I, value: V) {
    self.keys.push(key);
    self.values.push(value);
  }

  pub fn append_child(&mut self, child: Node<I, V>) {
    self.children.push(child);
  }



}
