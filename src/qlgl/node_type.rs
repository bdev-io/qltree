use super::bytes_ext::BytesExtension;

type NextLeafOffset = Option<u64>;

#[derive(Debug, Clone, Copy)]
pub enum NodeType{
  Internal,
  Leaf(NextLeafOffset),
}

impl BytesExtension for NodeType {
  fn get_byte_size() -> usize {
    16
  }
  fn to_bytes(&self) -> Vec<u8> {
    match self {
      NodeType::Internal => {
        let mut bytes: Vec<u8> = vec![0u8; 16];
        bytes[0..8].copy_from_slice(&(1u64.to_be_bytes()));
        bytes[8..16].copy_from_slice(&(0u64.to_be_bytes()));
        bytes
      },
      NodeType::Leaf(next) => {
        let mut bytes: Vec<u8> = vec![0u8; 16];
        bytes[0..8].copy_from_slice(&(2u64.to_be_bytes()));
        bytes[8..16].copy_from_slice(&(next.unwrap_or(0u64).to_be_bytes()));
        // debug!("BYTES_SIZE: {}", bytes.len());
        bytes
      }
    }
  }
  fn new_from_bytes(bytes: Vec<u8>) -> Result<Self, String> {
    let node_type = u64::from_be_bytes(bytes[0..8].try_into().unwrap());
    let next_offset = u64::from_be_bytes(bytes[8..16].try_into().unwrap());

    // debug!("node_type: {}, next_offset: {}", node_type, next_offset);

    match node_type {
      1 => Ok(NodeType::Internal),
      2 => Ok(NodeType::Leaf(Some(next_offset))),
      _ => Err("Invalid NodeType".to_string()),
    }
  }

  fn from_bytes(&mut self, bytes: Vec<u8>) -> Result<(), String> {
    let node_type = u64::from_be_bytes(bytes[0..8].try_into().unwrap());
    let next_offset = u64::from_be_bytes(bytes[8..16].try_into().unwrap());

    // debug!("node_type: {}, next_offset: {}", node_type, next_offset);

    match node_type {
      1 => {
        *self = NodeType::Internal;
        Ok(())
      },
      2 => {
        *self = NodeType::Leaf(Some(next_offset));
        Ok(())
      },
      _ => Err("Invalid NodeType".to_string()),
    }
  }
}
