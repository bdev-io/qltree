type NextLeafOffset = Option<u64>;

#[derive(Debug, Clone, Copy)]
pub enum NodeType{
  Internal,
  Leaf(NextLeafOffset),
}

impl NodeType {
  pub fn get_size() -> usize {
    16
  }
  pub fn to_bytes(&self) -> Vec<u8> {
    match self {
      NodeType::Internal => {
        let mut bytes: Vec<u8> = vec![];
        bytes.extend_from_slice(&(1u64.to_be_bytes()));
        bytes.extend_from_slice(&(0u64.to_be_bytes()));
        bytes
      },
      NodeType::Leaf(next) => {
        let mut bytes: Vec<u8> = vec![];
        bytes.extend_from_slice(&(2u64.to_be_bytes()));
        bytes.extend_from_slice(&(next.unwrap_or(0u64).to_be_bytes()));
        bytes
      }
    }
  }
  pub fn from_bytes(bytes: Vec<u8>) -> Self {
    let node_type = u64::from_be_bytes(bytes[0..8].try_into().unwrap());
    let next_offset = u64::from_be_bytes(bytes[8..16].try_into().unwrap());

    match node_type {
      1 => NodeType::Internal,
      2 => NodeType::Leaf(Some(next_offset)),
      _ => panic!("Invalid NodeType"),
    }
  }
}
