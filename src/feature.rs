use std::fs;
use std::path::Path;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FeaturePath {
    pub domain: String,
    pub feature: String,
}

impl FeaturePath {
    pub fn new(domain: impl Into<String>, feature: impl Into<String>) -> Self {
        Self {
            domain: domain.into(),
            feature: feature.into(),
        }
    }

    pub fn spec_path(&self, base: &Path) -> std::path::PathBuf {
        base.join(&self.domain).join(&self.feature).join("spec.md")
    }
}

impl std::fmt::Display for FeaturePath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}", self.domain, self.feature)
    }
}

pub fn discover_domains(base: &Path) -> Vec<String> {
    let Ok(entries) = fs::read_dir(base) else {
        return Vec::new();
    };

    let mut domains: Vec<String> = entries
        .filter_map(Result::ok)
        .filter(|e| e.path().is_dir())
        .filter_map(|e| e.file_name().into_string().ok())
        .filter(|name| !name.starts_with('_') && !name.starts_with('.'))
        .collect();

    domains.sort();
    domains
}

pub fn discover_features_in_domain(base: &Path, domain: &str) -> Vec<FeaturePath> {
    let domain_path = base.join(domain);
    let Ok(entries) = fs::read_dir(&domain_path) else {
        return Vec::new();
    };

    let mut features: Vec<FeaturePath> = entries
        .filter_map(Result::ok)
        .filter(|e| e.path().is_dir())
        .filter(|e| e.path().join("spec.md").exists())
        .filter_map(|e| e.file_name().into_string().ok())
        .map(|name| FeaturePath::new(domain, name))
        .collect();

    features.sort_by(|a, b| a.feature.cmp(&b.feature));
    features
}

pub fn discover_features(base: &Path) -> Vec<FeaturePath> {
    let domains = discover_domains(base);
    let mut all_features = Vec::new();

    for domain in domains {
        all_features.extend(discover_features_in_domain(base, &domain));
    }

    all_features
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    fn setup_test_hierarchy() -> TempDir {
        let tmp = TempDir::new().unwrap();
        let base = tmp.path();

        // Create domain/feature structure with spec.md files
        fs::create_dir_all(base.join("cli/validate")).unwrap();
        fs::write(base.join("cli/validate/spec.md"), "# Feature: Validate").unwrap();

        fs::create_dir_all(base.join("validation/document-structure")).unwrap();
        fs::write(
            base.join("validation/document-structure/spec.md"),
            "# Feature: Doc",
        )
        .unwrap();

        fs::create_dir_all(base.join("validation/rfc2119-keywords")).unwrap();
        fs::write(
            base.join("validation/rfc2119-keywords/spec.md"),
            "# Feature: RFC",
        )
        .unwrap();

        // Create _plans directory (should be ignored)
        fs::create_dir_all(base.join("_plans/some-plan")).unwrap();
        fs::write(base.join("_plans/some-plan/spec.md"), "# Plan").unwrap();

        tmp
    }

    #[test]
    fn discovers_domains_excluding_underscore_prefixed() {
        let tmp = setup_test_hierarchy();
        let domains = discover_domains(tmp.path());

        assert_eq!(domains, vec!["cli", "validation"]);
    }

    #[test]
    fn discovers_features_in_domain() {
        let tmp = setup_test_hierarchy();
        let features = discover_features_in_domain(tmp.path(), "validation");

        assert_eq!(features.len(), 2);
        assert_eq!(features[0].feature, "document-structure");
        assert_eq!(features[1].feature, "rfc2119-keywords");
    }

    #[test]
    fn discovers_all_features() {
        let tmp = setup_test_hierarchy();
        let features = discover_features(tmp.path());

        assert_eq!(features.len(), 3);
        assert!(features.contains(&FeaturePath::new("cli", "validate")));
        assert!(features.contains(&FeaturePath::new("validation", "document-structure")));
        assert!(features.contains(&FeaturePath::new("validation", "rfc2119-keywords")));
    }

    #[test]
    fn feature_path_display() {
        let fp = FeaturePath::new("domain", "feature");
        assert_eq!(format!("{}", fp), "domain/feature");
    }

    #[test]
    fn feature_path_spec_path() {
        let fp = FeaturePath::new("cli", "validate");
        let base = Path::new("/specs");
        assert_eq!(fp.spec_path(base), Path::new("/specs/cli/validate/spec.md"));
    }

    #[test]
    fn empty_dir_returns_empty_vec() {
        let tmp = TempDir::new().unwrap();
        let domains = discover_domains(tmp.path());
        assert!(domains.is_empty());
    }

    #[test]
    fn nonexistent_dir_returns_empty_vec() {
        let domains = discover_domains(Path::new("/nonexistent/path"));
        assert!(domains.is_empty());
    }
}
