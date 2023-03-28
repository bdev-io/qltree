use super::{ IndexTrait, ValueTrait };

pub struct NodeFile<I: IndexTrait, V: ValueTrait> {
  node_id: i64,             // TYPE : 음수 : 사용 안함, 0: Un initialized, 양수 : 사용중
  parent_id: i64,           // TYPE : 부모 노드의 아이디
  node_type: NodeType,      // TYPE : 노드 타입
  
  key_size: u64,            // TYPE : 키 크기 ( 최대 인덱스의 개수 )
  index_size: u64,          // TYPE : 인덱스 크기
  value_size: u64,          // TYPE : 값 크기


  index_list: Vec<I>,     // TYPE : 인덱스 리스트
  index_count: u64,         // TYPE : 인덱스 개수

  child_liset: Vec<u64>,    // TYPE : 자식 노드 리스트

  value_list: Vec<V>,     // TYPE : 값 리스트
  value_count: u64,         // TYPE : 값 개수
}


pub enum NodeType {
  Root,
  Internal,
  Leaf,
}

#[cfg(feature = "sync")]
mod sync;

//#[cfg(feature = "async")]
// mod async;
