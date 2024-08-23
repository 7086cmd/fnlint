use anyhow::Result;
use regex::Regex;
use serde::de::Visitor;
use serde::{de, Deserialize, Deserializer};
use serde_json::Value;
use std::collections::HashMap;
use std::fmt::Display;
use std::path::Path;
use std::str::FromStr;
use std::sync::LazyLock;

#[derive(Debug, PartialEq, Copy, Clone, Deserialize)]
pub enum FilenameCase {
  Lower,
  Snake,
  Camel,
  Kebab,
  Pascal,
  Point,
  ScreamingSnake,
}

impl FromStr for FilenameCase {
  type Err = String;
  fn from_str(s: &str) -> anyhow::Result<Self, Self::Err> {
    match s {
      "lowercase" => Ok(FilenameCase::Lower),
      "snake_case" => Ok(FilenameCase::Snake),
      "camelCase" => Ok(FilenameCase::Camel),
      "kebab-case" => Ok(FilenameCase::Kebab),
      "Pascal" => Ok(FilenameCase::Pascal),
      "point.case" => Ok(FilenameCase::Point),
      "SCREAMING_SNAKE_CASE" => Ok(FilenameCase::ScreamingSnake),
      _ => Err(format!("Unknown filename case: {}", s)),
    }
  }
}

impl Display for FilenameCase {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      FilenameCase::Lower => write!(f, "lowercase"),
      FilenameCase::Snake => write!(f, "snake_case"),
      FilenameCase::Camel => write!(f, "camelCase"),
      FilenameCase::Kebab => write!(f, "kebab-case"),
      FilenameCase::Pascal => write!(f, "Pascal"),
      FilenameCase::Point => write!(f, "point.case"),
      FilenameCase::ScreamingSnake => write!(f, "SCREAMING_SNAKE_CASE"),
    }
  }
}

struct FilenamePatterns {
  snake_case: LazyLock<Regex>,
  camel_case: LazyLock<Regex>,
  kebab_case: LazyLock<Regex>,
  pascal_case: LazyLock<Regex>,
  lower_case: LazyLock<Regex>,
  point_case: LazyLock<Regex>,
  screaming_snake_case: LazyLock<Regex>,
  none_split: LazyLock<Regex>, // No any `.`, `_`, capital letter
}

static PATTERNS: FilenamePatterns = FilenamePatterns {
  snake_case: LazyLock::new(|| Regex::new(r"^[a-z0-9_]+$").unwrap()),
  camel_case: LazyLock::new(|| Regex::new(r"^[a-z]+([A-Z][a-z0-9]*)*$").unwrap()),
  kebab_case: LazyLock::new(|| Regex::new(r"^[a-z0-9]+(-[a-z0-9]+)*$").unwrap()),
  pascal_case: LazyLock::new(|| Regex::new(r"^[A-Z][a-z0-9]+([A-Z][a-z0-9]*)*$").unwrap()),
  lower_case: LazyLock::new(|| Regex::new(r"^[a-z0-9]+$").unwrap()),
  point_case: LazyLock::new(|| Regex::new(r"^[a-z0-9]+(\.[a-z0-9]+)*$").unwrap()),
  screaming_snake_case: LazyLock::new(|| Regex::new(r"^[A-Z0-9_]+$").unwrap()),
  none_split: LazyLock::new(|| Regex::new(r"^[a-z0-9]+$").unwrap()),
};

impl FilenameCase {
  pub(crate) fn matches(&self, filename: &str) -> bool {
    if PATTERNS.none_split.is_match(filename) {
      return true;
    }
    match self {
      FilenameCase::Lower
      | FilenameCase::Point
      | FilenameCase::Snake
      | FilenameCase::Kebab
      | FilenameCase::Camel
        if PATTERNS.none_split.is_match(filename) =>
      {
        true
      }
      FilenameCase::Snake => PATTERNS.snake_case.is_match(filename),
      FilenameCase::Camel => PATTERNS.camel_case.is_match(filename),
      FilenameCase::Kebab => PATTERNS.kebab_case.is_match(filename),
      FilenameCase::Pascal => PATTERNS.pascal_case.is_match(filename),
      FilenameCase::Lower => PATTERNS.lower_case.is_match(filename),
      FilenameCase::Point => PATTERNS.point_case.is_match(filename),
      FilenameCase::ScreamingSnake => PATTERNS.screaming_snake_case.is_match(filename),
    }
  }
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct FilenameLintConfig {
  #[serde(deserialize_with = "deserialize_map")]
  pub ls: HashMap<String, Vec<FilenameCase>>,
  pub ignore: Vec<String>,
}

fn deserialize_map<'de, D>(deserializer: D) -> Result<HashMap<String, Vec<FilenameCase>>, D::Error>
where
  D: Deserializer<'de>,
{
  struct MapVisitor;

  impl<'de> Visitor<'de> for MapVisitor {
    type Value = HashMap<String, Vec<FilenameCase>>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
      formatter.write_str("a map of strings to lists of filename cases")
    }

    fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
    where
      M: de::MapAccess<'de>,
    {
      let mut map = HashMap::new();

      while let Some((key, value)) = access.next_entry::<String, Vec<String>>()? {
        let cases: Vec<FilenameCase> = value
          .into_iter()
          .map(|s| FilenameCase::from_str(&s).map_err(de::Error::custom))
          .collect::<Result<_, _>>()?;
        map.insert(key, cases);
      }

      Ok(map)
    }
  }

  deserializer.deserialize_map(MapVisitor)
}

impl FilenameLintConfig {
  pub fn load_file() -> Result<Self> {
    let json_path = Path::new("./fnlint.config.json");
    if json_path.exists() {
      Self::load_json(json_path.to_str().unwrap().to_string())
    } else {
      let yaml_path = Path::new("./fnlint.config.yaml");
      if yaml_path.exists() {
        Self::load_yaml(yaml_path.to_str().unwrap().to_string())
      } else {
        let toml_path = Path::new("./fnlint.config.toml");
        if toml_path.exists() {
          Self::load_toml(toml_path.to_str().unwrap().to_string())
        } else {
          panic!("No configuration file found");
        }
      }
    }
  }

  fn load_json(path: String) -> Result<Self> {
    let config = std::fs::read_to_string(path)?;
    let config: Value = serde_json::from_str(&config)?;
    let config: Self = serde_json::from_value(config)?;
    Ok(config)
  }

  fn load_yaml(path: String) -> Result<Self> {
    let config = std::fs::read_to_string(path)?;
    let config: Self = serde_yml::from_str(&config)?;
    Ok(config)
  }

  fn load_toml(path: String) -> Result<Self> {
    let config = std::fs::read_to_string(path)?;
    let config: Self = toml::from_str(&config)?;
    Ok(config)
  }
}
