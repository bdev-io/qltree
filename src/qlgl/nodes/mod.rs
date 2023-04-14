use crate::DEGREE;

use super::{
  NodeType,
  Index, Value,
  BytesExtension,
};

pub struct Node<I: Index, V: Value> {
  pub node_type: NodeType,      // TYPE : 노드의 타입 (Internal, Leaf)
  pub is_root: bool,            // TYPE : 노드가 루트 노드인지 여부

  pub is_dirty: bool,           // TYPE : 노드가 변경되었는지 여부
  pub is_overflow: bool,        // TYPE : 노드가 오버플로우 되었는지 여부


  parent_offset: u64,           // TYPE : 부모 노드의 오프셋
  offset: u64,                  // TYPE : GL(Value File) 내부 오프셋

  used_count: usize,            // TYPE : 사용된 키의 개수
  pub keys: Vec<I>,             // TYPE : 키 벡터
  pub values: Vec<V>,           // TYPE : 값 벡터
  pub children: Vec<u64>,       // TYPE : 자식 노드 벡터

  pub phantom: std::marker::PhantomData<(I, V)>,
}


impl<I: Index, V: Value> std::fmt::Debug for Node<I, V> {
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
      .field("used_count", &self.used_count)
      .field("KEYS", &(self.keys.as_slice()))
      .finish()
  }
}


mod io;
mod default_features;
mod crud;
