use std::fs::create_dir_all;
use std::io::Error;
use std::path::Path;

/// 初始化日志目录
pub fn init_log_dir(log_dir: &String) -> Result<(), Error> {
  let log_dir_path = Path::new(log_dir);
  if log_dir_path.exists() {
      Ok(())
  } else {
      create_dir_all(log_dir_path)
  }
}
