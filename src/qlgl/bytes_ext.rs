use crate::{DEGREE, Value, Index};

pub trait BytesExtension where Self: Sized {
  fn get_byte_size() -> usize;
  fn to_bytes(&self) -> Vec<u8>;
  fn from_bytes(&mut self, bytes: Vec<u8>) -> Result<(), String>;
  fn new_from_bytes(bytes: Vec<u8>) -> Result<Self, String>;
}

impl BytesExtension for bool {
  fn get_byte_size() -> usize {
    1
  }
  fn to_bytes(&self) -> Vec<u8> {
    let mut bytes: Vec<u8> = vec![];
    bytes.push(*self as u8);
    bytes
  }
  fn from_bytes(&mut self, bytes: Vec<u8>) -> Result<(), String> {
    if bytes.len() != 1 {
      return Err(format!("Invalid byte size: {}", bytes.len()));
    }
    *self = bytes[0] != 0;
    Ok(())
  }
  fn new_from_bytes(bytes: Vec<u8>) -> Result<Self, String> {
    if bytes.len() != 1 {
      return Err(format!("Invalid byte size: {}", bytes.len()));
    }
    Ok(bytes[0] != 0)
  }
}

impl BytesExtension for i64 {
  fn get_byte_size() -> usize {
    8
  }
  fn to_bytes(&self) -> Vec<u8> {
    let mut bytes: Vec<u8> = vec![];
    bytes.extend_from_slice(&self.to_be_bytes());
    bytes
  }
  fn from_bytes(&mut self, bytes: Vec<u8>) -> Result<(), String> {
    if bytes.len() != 8 {
      return Err(format!("Invalid byte size: {}", bytes.len()));
    }
    *self = i64::from_be_bytes(bytes.try_into().unwrap());
    Ok(())
  }
  fn new_from_bytes(bytes: Vec<u8>) -> Result<Self, String> {
    if bytes.len() != 8 {
      return Err(format!("Invalid byte size: {}", bytes.len()));
    }
    Ok(i64::from_be_bytes(bytes.try_into().unwrap()))
  }
}

impl BytesExtension for u64 {
  fn get_byte_size() -> usize {
    8
  }
  fn to_bytes(&self) -> Vec<u8> {
    let mut bytes: Vec<u8> = vec![];
    bytes.extend_from_slice(&self.to_be_bytes());
    bytes
  }
  fn from_bytes(&mut self, bytes: Vec<u8>) -> Result<(), String> {
    if bytes.len() != 8 {
      return Err(format!("Invalid byte size: {}", bytes.len()));
    }
    *self = u64::from_be_bytes(bytes.try_into().unwrap());
    Ok(())
  }
  fn new_from_bytes(bytes: Vec<u8>) -> Result<Self, String> {
    if bytes.len() != 8 {
      return Err(format!("Invalid byte size: {}", bytes.len()));
    }
    Ok(u64::from_be_bytes(bytes.try_into().unwrap()))
  }
}

impl BytesExtension for usize {
  fn get_byte_size() -> usize {
    8
  }
  fn to_bytes(&self) -> Vec<u8> {
    let mut bytes: Vec<u8> = vec![];
    bytes.extend_from_slice(&self.to_be_bytes());
    bytes
  }
  fn from_bytes(&mut self, bytes: Vec<u8>) -> Result<(), String> {
    if bytes.len() != 8 {
      return Err(format!("Invalid byte size: {}", bytes.len()));
    }
    *self = usize::from_be_bytes(bytes.try_into().unwrap());
    Ok(())
  }
  fn new_from_bytes(bytes: Vec<u8>) -> Result<Self, String> {
    if bytes.len() != 8 {
      return Err(format!("Invalid byte size: {}", bytes.len()));
    }
    Ok(usize::from_be_bytes(bytes.try_into().unwrap()))
  }
}

impl<I> BytesExtension for Vec<I> where I: BytesExtension {
  fn get_byte_size() -> usize { (DEGREE - 1) * I::get_byte_size() }

  fn to_bytes(&self) -> Vec<u8> {
    let mut bytes: Vec<u8> = vec![];
    for i in 0..(DEGREE-1) {
      if i < self.len() {
        bytes.extend_from_slice(&self[i].to_bytes());
      } else {
        bytes.extend_from_slice(&vec![0u8; I::get_byte_size()]);
      }
    }
    bytes
  }

  fn new_from_bytes(bytes: Vec<u8>) -> Result<Self, String> {
    let mut keys: Vec<I> = vec![];
    for i in 0..(DEGREE-1) {
      let start = i * I::get_byte_size();
      let end = start + I::get_byte_size();
      let key_bytes = bytes[start..end].to_vec();
      keys.push(I::new_from_bytes(key_bytes)?);
    }
    Ok(keys)
  }

  fn from_bytes(&mut self, bytes: Vec<u8>) -> Result<(), String> {
    for i in 0..(DEGREE-1) {
      let start = i * I::get_byte_size();
      let end = start + I::get_byte_size();
      let key_bytes = bytes[start..end].to_vec();
      self[i] = I::new_from_bytes(key_bytes)?;
    }
    Ok(())
  }
}

impl Index for i64 {}
impl Value for i64 {}
impl Index for u64 {}
impl Value for u64 {}

