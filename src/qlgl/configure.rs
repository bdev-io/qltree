use std::path::PathBuf;

#[derive(Debug)]
#[must_use]
pub struct Config {
  dir_path: PathBuf,
  key_size: u64,
}


impl Config {
  pub fn new(dir_path: PathBuf, key_size: u64) -> Self {
    if dir_path.is_dir() && key_size > 0 && key_size % 2 == 1 {
      Self {
        dir_path,
        key_size,
      }
    } else {
      if key_size <= 0 {
        panic!("Key Size must be greater than 0 and ODD NUMBER");
      }
      if key_size % 2 == 0 {
        panic!("Key Size must be ODD NUMBER");
      }

      if dir_path.is_file() {
        panic!("{} is not a directory", dir_path.display());
      }

      panic!("ERRRRRRRR");
    }
  }

  // TODO : Load Configure from dir_path
  pub fn load(dir_path: PathBuf) -> Self {
    todo!("Load Configure from dir Path")
  }

  pub fn get_dir_path(&self) -> &PathBuf {
    &self.dir_path
  }

  // TODO : Set Key Size
  pub fn set_key_size(&mut self, key_size: usize) {
    todo!("Set Key Size")
  }
}
// NOTE : Building Configure


impl Config {

}
