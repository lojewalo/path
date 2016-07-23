use path_sort::error::*;
use number_prefix::{Prefixed, Standalone, binary_prefix};
use path_sort::OsPathHandler;
use separator::Separatable;
use std::path::Path as OsPath;

pub struct View;

impl View {
  pub fn new() -> Self {
    View {}
  }

  pub fn get_view(&self, paths: &[String]) {
    for path in paths {
      match self.get_single_view(path) {
        Ok(x) => println!("{}\n  {}", path, x),
        Err(e) => {
          println!("{}\n  {}",
                   path,
                   e.iter().map(|e| e.to_string()).collect::<Vec<_>>().join("\n  "))
        }
      }
    }
  }

  pub fn get_single_view(&self, path: &str) -> Result<String> {
    let dir = try!(OsPath::new(path).read_dir());
    let (mut num_files, mut size_bytes) = (0, 0);
    for path in dir {
      let path = try!(path);
      let metadata = try!(path.metadata());
      size_bytes += metadata.len();
      num_files += 1;
    }
    let file_plural = if num_files == 1 {
      ""
    } else {
      "s"
    };
    let formatted_bytes = match binary_prefix(size_bytes as f64) {
      Standalone(bytes) => {
        format!("{} byte{}",
                bytes,
                if bytes == 1f64 {
                  ""
                } else {
                  "s"
                })
      }
      Prefixed(prefix, n) => format!("{:.2} {}B", n, prefix),
    };
    return Ok(format!("{} file{}\n  {}", num_files.separated_string(), file_plural, formatted_bytes));
  }

  pub fn print_view(&self) {
    match OsPathHandler::get_path() {
      Ok(paths) => self.get_view(&*paths),
      Err(e) => println!("could not read $PATH: {}", e),
    }
  }
}
