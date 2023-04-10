use std::fmt::Debug;

pub trait ValueTrait: Debug + Send + Sync + Clone + Copy + Default {
  fn to_bytes(&self) -> Vec<u8>;         // 값을 바이트로 변환
  fn get_size() -> u64;             // 값의 크기를 반환
  fn from_bytes(bytes: &[u8]) -> Self; // 바이트를 값으로 변환
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

  fn from_bytes(bytes: &[u8]) -> i64 {
    i64::from_be_bytes(bytes.try_into().unwrap())
  }

}

