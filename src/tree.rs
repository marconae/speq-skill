use crate::feature::FeaturePath;
use std::collections::BTreeMap;

pub fn render_tree(features: &[FeaturePath]) -> String {
    if features.is_empty() {
        return String::from("No features found.");
    }

    let mut domains: BTreeMap<&str, Vec<&str>> = BTreeMap::new();

    for fp in features {
        domains.entry(&fp.domain).or_default().push(&fp.feature);
    }

    let mut output = String::new();
    let domain_count = domains.len();

    for (idx, (domain, features)) in domains.iter().enumerate() {
        let is_last_domain = idx == domain_count - 1;
        let domain_prefix = if is_last_domain {
            "└── "
        } else {
            "├── "
        };
        let child_prefix = if is_last_domain { "    " } else { "│   " };

        output.push_str(&format!("{}{}/\n", domain_prefix, domain));

        let feature_count = features.len();
        for (fidx, feature) in features.iter().enumerate() {
            let is_last_feature = fidx == feature_count - 1;
            let feature_prefix = if is_last_feature {
                "└── "
            } else {
                "├── "
            };
            output.push_str(&format!("{}{}{}\n", child_prefix, feature_prefix, feature));
        }
    }

    output
}

pub fn render_domain_tree(domain: &str, features: &[FeaturePath]) -> String {
    if features.is_empty() {
        return format!("No features found in domain '{}'.", domain);
    }

    let mut output = format!("{}/\n", domain);
    let feature_count = features.len();

    for (idx, fp) in features.iter().enumerate() {
        let is_last = idx == feature_count - 1;
        let prefix = if is_last { "└── " } else { "├── " };
        output.push_str(&format!("{}{}\n", prefix, fp.feature));
    }

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn renders_empty_tree() {
        let output = render_tree(&[]);
        assert_eq!(output, "No features found.");
    }

    #[test]
    fn renders_single_domain_single_feature() {
        let features = vec![FeaturePath::new("cli", "validate")];
        let output = render_tree(&features);

        assert_eq!(output, "└── cli/\n    └── validate\n");
    }

    #[test]
    fn renders_multiple_domains() {
        let features = vec![
            FeaturePath::new("cli", "validate"),
            FeaturePath::new("validation", "document-structure"),
            FeaturePath::new("validation", "rfc2119-keywords"),
        ];
        let output = render_tree(&features);

        let expected = "\
├── cli/
│   └── validate
└── validation/
    ├── document-structure
    └── rfc2119-keywords
";
        assert_eq!(output, expected);
    }

    #[test]
    fn renders_domain_tree_empty() {
        let output = render_domain_tree("cli", &[]);
        assert_eq!(output, "No features found in domain 'cli'.");
    }

    #[test]
    fn renders_domain_tree_with_features() {
        let features = vec![
            FeaturePath::new("validation", "document-structure"),
            FeaturePath::new("validation", "rfc2119-keywords"),
        ];
        let output = render_domain_tree("validation", &features);

        let expected = "\
validation/
├── document-structure
└── rfc2119-keywords
";
        assert_eq!(output, expected);
    }
}
