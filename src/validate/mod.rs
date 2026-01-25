pub mod parser;
pub mod report;
pub mod rules;

use std::path::Path;

use report::{ValidationError, ValidationResult};

pub fn run(path: &Path) -> Result<ValidationResult, ValidationError> {
    let content = std::fs::read_to_string(path).map_err(|_| ValidationError::FileNotFound {
        path: path.display().to_string(),
    })?;

    let spec = parser::parse(&content)?;
    Ok(rules::validate(&spec))
}
