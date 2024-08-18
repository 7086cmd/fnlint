use glob::Pattern as GlobPattern;
use regex::Regex as RegexPattern;

pub enum Pattern {
    Glob(GlobPattern),
    Regex(RegexPattern),
    Plain(String),
}

impl Pattern {
    pub fn new(pattern: &str) -> Self {
        if pattern.contains('*') || pattern.contains('?') {
            Pattern::Glob(GlobPattern::new(pattern).unwrap())
        } else if pattern.starts_with('^') && pattern.ends_with('$') {
            Pattern::Regex(RegexPattern::new(&pattern[1..pattern.len() - 1]).unwrap())
        } else {
            Pattern::Plain(pattern.to_string())
        }
    }
    pub fn matches(&self, field: &str) -> bool {
        match self {
            Pattern::Glob(pattern) => pattern.matches(field),
            Pattern::Regex(pattern) => pattern.is_match(field),
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

    #[test]
    fn test_regex_pattern() {
        let pattern = Pattern::new(r"^src/.*\.rs$");
        assert!(matches!(pattern, Pattern::Regex(_)));
        assert!(pattern.matches("src/main.rs"));
        assert!(pattern.matches("src/lib.rs"));
        assert!(!pattern.matches("Cargo.toml"));
    }
}