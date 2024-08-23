use walkdir::{DirEntry, WalkDir};

fn is_ignored(entry: &DirEntry, ignore: &Vec<String>) -> bool {
  let path = entry.path();
  let path_str = path.to_str().unwrap();
  // 1. the folder: e.g. `node_modules` in ignore, so the folder of `node_modules` will be ignored
  // 2. the file: e.g. `*.log` in ignore, so the file of `server.log` will be ignored
  // 3. glob folder pattern: e.g. `**/config` in ignore, so the folder of `src/config` will be ignored
  ignore.iter().any(|pattern| {
    let levels = pattern.split('/').collect::<Vec<&str>>();
    // 1. any level contains the ignore pattern
    if levels.iter().any(|level| path_str.contains(level)) {
      return true;
    }
    // 2. check the file name
    if path.is_file() && path_str.contains(pattern) {
      return true;
    }
    // 3. handle glob folder pattern
    if pattern.contains("**") {
      let mut pattern = pattern.replace("**", "");
      if pattern.ends_with('/') {
        pattern.pop();
      }
      let pattern = format!("{}$", pattern);
      let re = regex::Regex::new(&pattern).unwrap();
      return re.is_match(path_str);
    }
    // 4. handle the glob file pattern
    if pattern.contains('*') {
      let filename = path.file_name().unwrap().to_str().unwrap();
      let re = regex::Regex::new(&pattern.replace("*", ".*")).unwrap();
      return re.is_match(filename);
    }
    false
  })
}

pub fn scan_dir(base: &str, ignore: &Vec<String>) -> Vec<String> {
  let walker = WalkDir::new(base).into_iter();
  walker
    .filter_map(Result::ok)
    .filter(|entry| !is_ignored(entry, ignore))
    .filter(|entry| entry.path().is_file())
    .map(|entry| entry.path().to_str().unwrap().to_string())
    .collect()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_scan_dir() {
    let files = scan_dir("src", &vec!["main.rs".to_string()]);
    assert!(files.contains(&"src/config/mod.rs".to_string()));
    assert!(!files.contains(&"src/main.rs".to_string()));
  }

  #[test]
  fn test_glob_no_config_folder() {
    let files = scan_dir("src", &vec!["config/**".to_string()]);
    assert!(!files.contains(&"src/config/mod.rs".to_string()));
    assert!(files.contains(&"src/main.rs".to_string()));
  }

  #[test]
  fn test_no_config_folder() {
    let files = scan_dir("src", &vec!["config".to_string()]);
    assert!(!files.contains(&"src/config/mod.rs".to_string()));
    assert!(files.contains(&"src/main.rs".to_string()));
  }

  #[test]
  fn test_glob_no_rs() {
    let files = scan_dir("src", &vec!["*.rs".to_string()]);
    assert!(!files.contains(&"src/config/mod.rs".to_string()));
    assert!(!files.contains(&"src/main.rs".to_string()));
  }
}
