use std::fmt::Debug;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use super::{ Tree, IndexTrait, ValueTrait };

use crate::fm::roots::RootFile;


// NOTE : Implement
impl<I: IndexTrait, V: ValueTrait> Tree<I, V> {
  pub fn new(root_dir_path: PathBuf) -> Self {
    Self {
      root_dir_path,
      root: Arc::new(Mutex::new(Option::None)),
      is_open: false,
      phantom: std::marker::PhantomData,
    }
  }

  pub fn set_params(&mut self, root_id: Option<i64>, tree_name: Option<String>, key_size: Option<u64>) {
    let index_size = I::get_size();
    let value_size = V::get_size();

    let mut root = self.root.lock().unwrap();
    if root.is_some() {
      let root = root.as_mut().unwrap();

      if let Some(root_id) = root_id {
        root.set_root_id(root_id);
      }

      if let Some(tree_name) = tree_name {
        root.set_tree_name(tree_name);
      }

      if let Some(key_size) = key_size {
        root.set_key_size(key_size);
      }

      root.set_index_size(index_size);
      root.set_value_size(value_size);
    } else {
      warn!("Open First!")
    }
  }


  // TODO : Open Tree ( 초기화 없이 읽어오도록)
  pub fn open(&mut self) -> Result<(), String> {
    let root_file = RootFile::<I, V>::new();
    self.root = Arc::new(Mutex::new(Some(root_file)));
    Ok(())
  }


  // TODO : Close Tree
  pub fn close(&mut self) -> Result<(), String> {
    todo!("Close");
    Ok(())
  }
  
  // TODO : Insert
  pub fn insert(&self, _key: I, _value: V) -> Result<(), String> {
    let root = self.root.lock().unwrap().as_mut().unwrap();
    // root.try_into
    todo!("Insert");
    Ok(())
  }

  // TODO : Get
  pub fn get(&self, _key: I) -> Result<V, String> {
    todo!("Get");
    Ok(V::default())
  }

  // TODO : Update
  pub fn update(&self, _key: I, _value: V) -> Result<(), String> {
    todo!("Update");
    Ok(())
  }

  // TODO : Delete
  pub fn delete(&self, _key: I) -> Result<(), String> {
    todo!("Delete");
    Ok(())
  }



}

// NOTE : Debug
impl<I: IndexTrait, V: ValueTrait> Debug for Tree<I, V> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("Tree")
      .field("root_dir_path", &self.root_dir_path)
      .finish()
  }
}
