type PrevLeafOffset = Option<u64>;
type NextLeafOffset = Option<u64>;

pub enum NodeType{
  Internal,
  Leaf(PrevLeafOffset, NextLeafOffset),
}

impl NodeType {
  pub fn to_bytes(&self) -> Vec<u8> {
    match self {
      NodeType::Internal => {
        let mut bytes: Vec<u8> = vec![];
        bytes.extend_from_slice(&(0u64.to_be_bytes()));
        bytes.extend_from_slice(&(0u64.to_be_bytes()));
        bytes.extend_from_slice(&(0u64.to_be_bytes()));
        bytes
      },
      NodeType::Leaf(prev, next) => {
        let mut bytes: Vec<u8> = vec![];
        bytes.extend_from_slice(&(0u64.to_be_bytes()));
        bytes.extend_from_slice(&(prev.unwrap_or(0u64).to_be_bytes()));
        bytes.extend_from_slice(&(next.unwrap_or(0u64).to_be_bytes()));
        bytes
      }
    }
  }
}
