use crate::config::FilenameCase;
use std::fmt::Display;
use std::sync::Arc;
pub mod visitor;

pub struct Issue {
  pub filename: String,
  pub target: Arc<Vec<FilenameCase>>,
  pub path: String,
}

impl Display for Issue {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "Filename {} does not match any of the patterns: ", self.filename)?;
    let cases =
      self.target.iter().map(|target| target.to_string()).collect::<Vec<String>>().join(", ");
    write!(f, "{}", cases)?;
    Ok(())
  }
}

pub fn lint_files(
  files: Vec<String>,
  ext: String,
  patterns: &Arc<Vec<FilenameCase>>,
) -> Vec<Issue> {
  files.iter().filter_map(|path| lint_name(path, &patterns, &ext)).collect::<Vec<Issue>>()
}

fn lint_name(path: &str, patterns: &Arc<Vec<FilenameCase>>, ext: &str) -> Option<Issue> {
  let filename = path.split('/').last()?;
  // trim `ext` content
  let filename = filename.trim_end_matches(ext);
  for pattern in patterns.iter() {
    if pattern.matches(filename) {
      return None;
    }
  }
  Some(Issue { filename: filename.to_string(), target: patterns.clone(), path: path.to_string() })
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_issue_print() {
    let issue = Issue {
      filename: "hello-world.js".to_string(),
      target: Arc::new(vec![FilenameCase::Kebab, FilenameCase::Lower]),
      path: "src/linter/helloWorld.js".to_string(),
    };
    let expected =
      "Filename hello-world.js does not match any of the patterns: kebab-case, lowercase";
    assert_eq!(issue.to_string(), expected);
  }

  #[test]
  fn lint_none_case() {
    let patterns = Arc::new(vec![FilenameCase::Kebab, FilenameCase::Lower]);
    let no_issue = lint_name("src/linter/mod.rs", &patterns, ".rs").is_none();
    assert!(no_issue);
  }

  #[test]
  fn lint_kebab_case() {
    let patterns = Arc::new(vec![FilenameCase::Kebab]);
    let no_issue = lint_name("src/linter/hello-world.js", &patterns, ".js");
    assert!(no_issue.is_none());
    let camel = lint_name("src/linter/helloWorld.js", &patterns, ".js");
    assert!(camel.is_some());
    let pascal = lint_name("src/linter/HelloWorld.js", &patterns, ".js");
    assert!(pascal.is_some());
    let snake = lint_name("src/linter/hello_world.js", &patterns, ".js");
    assert!(snake.is_some());
  }

  #[test]
  fn lint_camel_case() {
    let patterns = Arc::new(vec![FilenameCase::Camel]);
    let always_good = lint_name("src/linter/mod.js", &patterns, ".js").is_none();
    assert!(always_good);
    let no_issue = lint_name("src/linter/helloWorld.js", &patterns, ".js").is_none();
    assert!(no_issue);
    let kebab = lint_name("src/linter/hello-world.js", &patterns, ".js");
    assert!(kebab.is_some());
    let pascal = lint_name("src/linter/HelloWorld.js", &patterns, ".js");
    assert!(pascal.is_some());
    let snake = lint_name("src/linter/hello_world.js", &patterns, ".js");
    assert!(snake.is_some());
  }

  #[test]
  fn lint_pascal_case() {
    let patterns = Arc::new(vec![FilenameCase::Pascal]);
    let no_issue = lint_name("src/linter/HelloWorld.js", &patterns, ".js").is_none();
    assert!(no_issue);
    let kebab = lint_name("src/linter/hello-world.js", &patterns, ".js");
    assert!(kebab.is_some());
    let camel = lint_name("src/linter/helloWorld.js", &patterns, ".js");
    assert!(camel.is_some());
    let snake = lint_name("src/linter/hello_world.js", &patterns, ".js");
    assert!(snake.is_some());
  }

  #[test]
  fn lint_snake_case() {
    let patterns = Arc::new(vec![FilenameCase::Snake]);
    let no_issue = lint_name("src/linter/hello_world.js", &patterns, ".js").is_none();
    assert!(no_issue);
    let kebab = lint_name("src/linter/hello-world.js", &patterns, ".js");
    assert!(kebab.is_some());
    let camel = lint_name("src/linter/helloWorld.js", &patterns, ".js");
    assert!(camel.is_some());
    let pascal = lint_name("src/linter/HelloWorld.js", &patterns, ".js");
    assert!(pascal.is_some());
  }

  #[test]
  fn lint_snake_files() {
    let patterns = Arc::new(vec![FilenameCase::Snake]);
    let files = vec![
      "src/linter/hello_world.js".to_string(),
      "src/linter/a_bC.js".to_string(),
      "src/linter/A_vbC.js".to_string(),
      "src/linter/helloWorld.js".to_string(),
      "src/linter/HelloWorld.js".to_string(),
      "src/linter/hello-world.js".to_string(),
    ];
    let issues = lint_files(files, ".js".to_string(), &patterns);
    assert_eq!(issues.len(), 5);
  }
}
