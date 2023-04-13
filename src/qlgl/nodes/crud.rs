use super::*;
use std::fs::File;

impl<I: Index, V: Value> Node<I,V> {
  pub fn insert(&mut self, node_file: &mut File, index: I, value: V) -> Result<(), String> {
    debug!("{:#?}", self);
    todo!("INSERT!!");
    Ok(())
  }

}
