use super::{ IndexTrait, ValueTrait, RootFile };
use std::error::Error;

impl<I: IndexTrait, V:ValueTrait> RootFile<I, V> {
  async fn new() -> Result<Self, Box<dyn Error>> {
    Ok(Self::new())
  }

}

