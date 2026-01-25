use thiserror::Error;

#[derive(Debug)]
pub struct ValidationResult {
    pub errors: Vec<ValidationError>,
    pub warnings: Vec<ValidationWarning>,
}

impl ValidationResult {
    pub fn new() -> Self {
        Self {
            errors: Vec::new(),
            warnings: Vec::new(),
        }
    }

    pub fn is_success(&self) -> bool {
        self.errors.is_empty()
    }

    pub fn add_error(&mut self, error: ValidationError) {
        self.errors.push(error);
    }

    pub fn add_warning(&mut self, warning: ValidationWarning) {
        self.warnings.push(warning);
    }
}

impl Default for ValidationResult {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Error, PartialEq)]
pub enum ValidationError {
    #[error("File not found: {path}")]
    FileNotFound { path: String },

    #[error("Missing feature description")]
    MissingFeatureDescription,

    #[error("Missing Background section")]
    MissingBackgroundSection,

    #[error("Missing Scenarios section")]
    MissingScenariosSection,

    #[error("No scenarios defined")]
    NoScenarios,

    #[error("Scenario '{scenario}' is missing a GIVEN step")]
    ScenarioMissingGiven { scenario: String },

    #[error("Scenario '{scenario}' is missing a WHEN step")]
    ScenarioMissingWhen { scenario: String },

    #[error("Scenario '{scenario}' is missing a THEN step")]
    ScenarioMissingThen { scenario: String },

    #[error("Step in scenario '{scenario}' is missing RFC 2119 keyword: {step}")]
    StepMissingRfc2119Keyword { scenario: String, step: String },
}

#[derive(Debug, PartialEq)]
pub enum ValidationWarning {
    TooManyAndSteps { scenario: String, count: usize },
}

impl std::fmt::Display for ValidationWarning {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ValidationWarning::TooManyAndSteps { scenario, count } => {
                write!(
                    f,
                    "Scenario '{scenario}' has {count} AND steps (recommended: 3 or fewer)"
                )
            }
        }
    }
}
