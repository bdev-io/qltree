use std::fmt::Debug;

pub trait ValueTrait: Debug + Send + Sync + Clone + Default {
  fn to_bytes(&self) -> Vec<u8>;         // 인덱스를 바이트로 변환
  fn get_size() -> u64;             // 인덱스의 크기를 반환
}

impl ValueTrait for i64 {
  fn get_size() -> u64 {
    8
  }

  fn to_bytes(&self) -> Vec<u8> {
    let mut bytes = Vec::new();
    bytes.extend_from_slice(&self.to_le_bytes());
    bytes
  }

}

