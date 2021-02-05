mod representation;
pub mod view;

use anyhow::Context;
use crate::Result;
use crate::path_sort::representation::{Path, SortFile};
use std::collections::BTreeMap;
use std::fs::File;
use std::io::Read;
use std::path::Path as OsPath;
use std::env;

const DEFAULT_RULES: &'static str = r#"{
  "rules": {
    "order": {
      "paths": [{"default": true}],
      "sort": {
        "alphabetical": "yes",
        "path_component_length": "yes"
      }
    },
    "must_exist": true,
    "must_be_unique": true
  }
}
"#;

#[derive(Debug)]
pub struct OsPathHandler<'a> {
  paths: Vec<String>,
  sort_file: &'a SortFile
}

impl<'a> OsPathHandler<'a> {
  pub fn new(sort_file: &'a SortFile) -> Result<Self> {
    let mut paths: Vec<String> = OsPathHandler::get_path()?;
    if let Some(must_exist) = sort_file.rules.must_exist {
      if must_exist {
        paths.retain(|p| OsPath::new(&p).exists());
      }
    }
    if let Some(must_be_absolute) = sort_file.rules.must_be_absolute {
      if must_be_absolute {
        paths.retain(|p| OsPath::new(&p).is_absolute());
      }
    }
    if let Some(must_be_unique) = sort_file.rules.must_be_unique {
      if must_be_unique {
        paths = paths.into_iter().fold(Vec::new(), |mut v, p| {
          if !v.contains(&p) {
            v.push(p)
          };
          v
        });
        // let set: HashSet<String> = paths.into_iter().collect();
        // paths = set.into_iter().collect();
      }
    }
    Ok(OsPathHandler {
      paths: paths,
      sort_file: sort_file
    })
  }

  pub fn get_path() -> Result<Vec<String>> {
    let path_var = env::var("PATH")?;
    Ok(path_var.split(':').map(|o| o.to_owned()).collect())
  }

  pub fn create_full_path(&self) -> Option<FullPath> {
    let mut sys_paths = self.paths.clone();
    if let Some(ref order) = self.sort_file.rules.order {
      if let Some(ref paths) = order.paths {
        let mut processed_paths: Vec<PathElements> = paths.iter()
          .flat_map(|x| match *x {
            Path::Exact(ref p) => {
              if let Some(pos) = sys_paths.iter().position(|x| x == p) {
                sys_paths.remove(pos);
                Some(PathElements { elements: vec![p.clone()] })
              } else {
                None
              }
            }
            Path::Contains(ref p) => {
              let matches: Vec<_> = sys_paths.iter().filter(|x| x.contains(p)).cloned().collect();
              {
                let str_matches: Vec<_> = matches.iter().collect();
                sys_paths.retain(|x| !str_matches.contains(&x));
              }
              if matches.is_empty() {
                None
              } else {
                Some(PathElements { elements: matches })
              }
            }
            Path::Default(x) => {
              if !x {
                return None;
              }
              Some(PathElements { elements: vec!["\0default\0".to_string()] })
            }
          })
          .collect();
        {
          let def = processed_paths.iter_mut()
            .map(|e| &mut e.elements)
            .filter(|e| e.len() == 1 && e[0] == "\0default\0")
            .take(1)
            .next();
          if let Some(x) = def {
            x.clear();
            x.append(&mut sys_paths);
          }
        }
        return Some(FullPath { paths: processed_paths });
      }
    }
    None
  }
}

#[derive(Debug)]
pub struct FullPath {
  paths: Vec<PathElements>
}

impl FullPath {
  pub fn sort(&mut self, sort_file: &SortFile) {
    for path in &mut self.paths {
      path.sort(sort_file);
    }
  }

  pub fn to_string(&self) -> String {
    self.paths.iter().flat_map(|x| x.elements.clone()).collect::<Vec<_>>().join(":")
  }
}

#[derive(Debug)]
pub struct PathElements {
  elements: Vec<String>
}

impl PathElements {
  fn sort(&mut self, sort_file: &SortFile) {
    // Get the rules for sorting alphabetically and by path component length
    let mut sort_alpha = 0;
    let mut sort_component = 0;
    if let Some(ref order) = sort_file.rules.order {
      if let Some(ref sort) = order.sort {
        if let Some(alpha) = sort.alphabetical {
          sort_alpha = alpha;
        }
        if let Some(component) = sort.path_component_length {
          sort_component = component;
        }
      }
    }
    // If we're not sorting at all
    if sort_alpha == 0 && sort_component == 0 {
      // Don't sort!
      return;
    }
    // If we're sorting alphabetically but not by path component length
    if sort_alpha != 0 && sort_component == 0 {
      // Sort alphabetically
      self.elements.sort();
      // If we're sorting in reverse
      if sort_alpha < 0 {
        // Reverse sort
        self.elements.reverse();
      }
      return;
    }
    // Otherwise, if we're sorting by path component length
    if sort_component != 0 {
      // Sorted map of path segment length to paths with that length
      let mut length_map: BTreeMap<usize, Vec<String>> = BTreeMap::new();
      for path in &self.elements {
        // Get the number of segments in the path
        let len = OsPath::new(&path).components().collect::<Vec<_>>().len();
        let entries = length_map.entry(len).or_insert_with(Vec::new);
        // Append this path to the list of paths with this length
        entries.push(path.to_owned());
      }
      // Clear out the old list
      self.elements.clear();
      let mut iter: Vec<(usize, Vec<String>)> = if sort_component > 0 {
        // If we're sorting by component length, get a forward iterator and collect it
        length_map.into_iter().collect()
      } else {
        // But if we're sorting in reverse, get a reversed iterator and collect it
        length_map.into_iter().rev().collect()
      };
      // Iterate over whatever we have
      for &mut (_, ref mut entries) in &mut iter {
        // Sort the paths alphabetically for each length group if necessary
        if sort_alpha != 0 {
          entries.sort();
          // If we're sorting in reverse
          if sort_alpha < 0 {
            // Reverse the sort
            entries.reverse();
          }
        }
        for entry in entries {
          // Add them all into one list
          self.elements.push(entry.to_owned());
        }
      }
    }
  }
}

pub struct PathSort;

impl PathSort {
  pub fn new() -> Self {
    PathSort {}
  }

  pub fn get_sort_file(&self, path: Option<&str>) -> Result<SortFile> {
    if let Some(path) = path {
      let os_path = OsPath::new(path);
      if !os_path.exists() || !os_path.is_file() {
        anyhow::bail!("{} does not exist or is not a file", path);
      }
      let mut file = match File::open(os_path) {
        Ok(x) => x,
        Err(e) => anyhow::bail!("error opening {}: {}", path, e),
      };
      let mut sort_file_str = String::new();
      file.read_to_string(&mut sort_file_str).with_context(|| format!("error reading {}", path))?;
      serde_json::from_str(&sort_file_str).with_context(|| format!("error creating rules from {}", path))
    } else {
      serde_json::from_str(DEFAULT_RULES).context("error creating default rules")
    }
  }
}
