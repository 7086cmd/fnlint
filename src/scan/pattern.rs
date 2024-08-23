use glob::Pattern as GlobPattern;
use regex::Regex as RegexPattern;

pub enum Pattern {
  Glob(GlobPattern),
  Plain(String),
}

impl Pattern {
  pub fn new(pattern: &str) -> Self {
    if pattern.contains('*') || pattern.contains('?') {
      Pattern::Glob(GlobPattern::new(pattern).unwrap())
    } else {
      Pattern::Plain(pattern.to_string())
    }
  }
  pub fn matches(&self, field: &str) -> bool {
    match self {
      Pattern::Glob(pattern) => pattern.matches(field),
      Pattern::Plain(pattern) => pattern == field,
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_glob_pattern() {
    let pattern = Pattern::new("*.rs");
    assert!(pattern.matches("main.rs"));
    assert!(pattern.matches("lib.rs"));
    assert!(!pattern.matches("Cargo.toml"));
  }
}
