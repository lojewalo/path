use std::collections::{BTreeMap, HashSet};
use std::env;
use std::path::Path;

fn get_path_segments() -> HashSet<String> {
  let path = match env::var("PATH") {
    Ok(p) => p,
    Err(_) => String::new()
  };
  path.split(':').map(|s| s.to_owned()).collect()
}

fn sort_by_path_segment_length(segments: &mut HashSet<String>) -> Vec<String> {
  let mut length_map: BTreeMap<usize, Vec<String>> = BTreeMap::new();
  for segment in segments.iter() {
    let len = Path::new(segment).components().collect::<Vec<_>>().len();
    let entries = length_map.entry(len).or_insert(Vec::new());
    entries.push(segment.to_owned());
  }
  let mut sorted = Vec::new();
  for (_, entries) in length_map.iter_mut() {
    entries.sort();
    for entry in entries {
      sorted.push(entry.to_owned());
    }
  }
  sorted
}

fn join_segments_to_path(segments: &Vec<String>) -> String {
  segments.join(":")
}

fn homebrew(path: &mut Vec<String>) {
  path.retain(|s| s != "/usr/local/sbin" && s != "/usr/local/bin");
  path.insert(0, String::from("/usr/local/sbin"));
  path.insert(0, String::from("/usr/local/bin"));
}

fn main() {
  let args: Vec<String> = env::args().collect();
  let mut segments = get_path_segments();
  let mut sorted = sort_by_path_segment_length(&mut segments);
  if args.len() > 1 && args[1] == "--homebrew" {
    homebrew(&mut sorted);
  }
  let path = join_segments_to_path(&sorted);
  println!("{}", path);
}
