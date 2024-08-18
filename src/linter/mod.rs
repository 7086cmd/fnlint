use crate::config::FilenameCase;
use std::str::FromStr;
use std::sync::Arc;
pub mod visitor;
use walkdir::WalkDir;

pub struct Issue {
  pub filename: String,
  pub target: Arc<Vec<FilenameCase>>,
  pub actual: FilenameCase,
  pub path: String,
}

pub fn visit_files(
  ext: String,
  patterns: Arc<Vec<FilenameCase>>,
  ignores: Vec<String>,
) -> Vec<Issue> {
  let mut issues: Vec<Issue> = vec![];
  vec![]
}

fn lint_name(path: &str, patterns: Arc<Vec<FilenameCase>>) -> Option<Issue> {
  let filename = path.split('/').last().unwrap();
  let actual = FilenameCase::from_str(filename)
    .expect(format!("Failed to parse filename: {}", filename).as_str());
  for accepted in patterns.iter() {
    if accepted.matches(filename) {
      return None;
    }
  }
  Some(Issue {
    filename: filename.to_string(),
    target: patterns,
    actual,
    path: path.to_string(),
  })
}
