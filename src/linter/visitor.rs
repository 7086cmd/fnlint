use crate::config::{FilenameCase, FilenameLintConfig};
use crate::linter::{lint_files, Issue};
use std::sync::Arc;

pub fn lint_filenames(config: &Arc<FilenameLintConfig>, file_list: Vec<String>) -> Vec<Issue> {
  let mut result = vec![];
  config.ls.iter().for_each(|(ext, patterns)| {
    let patterns: Arc<Vec<FilenameCase>> = Arc::new(patterns.into_iter().map(|case| *case).collect());
    let files = file_list.iter().filter(|file| file.ends_with(ext)).cloned().collect();
    let issues = lint_files(files, ext.to_string(), &patterns);
    issues.into_iter().for_each(|issue| result.push(issue));
  });
  result
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::config::FilenameCase;
  use std::collections::HashMap;

  #[test]
  fn test_lint_filenames() {
    let config = Arc::new(FilenameLintConfig {
      ls: {
        let mut map = HashMap::new();
        map.insert(".rs".to_string(), vec![FilenameCase::Snake]);
        map
      },
      ignore: vec![],
    });
    let files = vec!["src/main.rs".to_string(), "src/linter/mod.rs".to_string(), "src/linter/hello-world.rs".to_string()];
    let issues = lint_filenames(&config, files);
    assert_eq!(issues.len(), 1);
  }
}