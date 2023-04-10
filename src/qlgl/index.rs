use std::fmt::Debug;

pub trait IndexTrait: Debug + Send + Sync + PartialOrd + PartialEq {

  fn to_bytes(&self) -> Vec<u8>;         // 인덱스를 바이트로 변환
  fn get_size() -> u64;             // 인덱스의 크기를 반환
}

impl IndexTrait for i64 {
  fn get_size() -> u64 {
    8
  }

  fn to_bytes(&self) -> Vec<u8> {
    let mut bytes = Vec::new();
    bytes.extend_from_slice(&self.to_be_bytes());
    bytes
  }
}

// impl PartialEq for i64 {
//   fn eq(&self, other: &Self) -> bool {
//     self.cmp(other) == std::cmp::Ordering::Equal
//   }
// }
//
// impl PartialOrd for i64 {
//   fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
//     Some(self.cmp(other))
//   }
// }

impl IndexTrait for u64 {
  fn get_size() -> u64 {
    8
  }

  fn to_bytes(&self) -> Vec<u8> {
    let mut bytes = Vec::new();
    bytes.extend_from_slice(&self.to_be_bytes());
    bytes
  }
}

