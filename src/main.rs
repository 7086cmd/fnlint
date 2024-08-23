use crate::config::FilenameLintConfig;
use crate::linter::visitor::lint_filenames;
use crate::scan::scanner::scan_dir;
use anyhow::Result;

mod config;
mod linter;
mod scan;

fn main() -> Result<()> {
  let config = FilenameLintConfig::load_file()?;
  let files = scan_dir(".", &config.ignore);
  lint_filenames(&config, &files).iter().for_each(|issue| {
    println!("{}", issue);
  });
  Ok(())
}
