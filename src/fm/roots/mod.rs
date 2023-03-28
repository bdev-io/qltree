use std::marker::PhantomData;
use crate::qlgl::{IndexTrait, ValueTrait};

pub struct RootFile<I, V> where I: IndexTrait, V: ValueTrait {
  root_id: i64,                 // TYPE : 음수 : 사용 안함, 0: Un initialized, 양수 : 사용중

  tree_name: String,            // TYPE : 트리 이름
  name_len: u64,                // TYPE : 파일 이름의 길이

  page_size: u64,               // TYPE : 페이지 크기
  page_count: u64,              // TYPE : 페이지 개수


  index_size: u64,              // TYPE : 인덱스 크기
  value_size: u64,              // TYPE : 값 크기

  leaf_offsets: Vec<u64>,       // TYPE : 리프 페이지의 오프셋들

  _phantom: PhantomData<(I, V)> // TYPE : 더미 데이터
}


// NOTE : SYNC로 구현된 모듈
#[cfg(feature = "sync")]
mod sync;

// NOTE : ASYNC로 구현된 모듈
#[cfg(featue = "async")]
mod r#async;
