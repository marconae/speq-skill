use std::fs;
use std::path::Path;
use std::process::Command;

fn read_mcp_template(filename: &str) -> String {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let path = Path::new(manifest_dir)
        .join("scripts/plugin")
        .join(filename);
    fs::read_to_string(path).unwrap_or_else(|e| panic!("failed to read {filename}: {e}"))
}

mod claude_code_config {
    use super::*;

    #[test]
    fn mcp_json_uses_project_from_cwd() {
        assert!(read_mcp_template("mcp.json").contains("--project-from-cwd"));
    }

    #[test]
    fn mcp_json_has_no_static_project_path() {
        assert!(!read_mcp_template("mcp.json").contains("${PWD}"));
    }

    #[test]
    fn mcp_json_has_claude_code_context() {
        assert!(read_mcp_template("mcp.json").contains("claude-code"));
    }

    #[test]
    fn built_mcp_json_uses_project_from_cwd() {
        let manifest_dir = env!("CARGO_MANIFEST_DIR");
        let build_sh = Path::new(manifest_dir).join("scripts/plugin/build.sh");

        let status = Command::new("bash")
            .arg(build_sh)
            .current_dir(manifest_dir)
            .status()
            .expect("failed to run build.sh");

        assert!(status.success(), "build.sh exited with failure: {status}");

        let output_path =
            Path::new(manifest_dir).join("dist/marketplace/plugins/speq-skill/.mcp.json");
        let content = fs::read_to_string(&output_path)
            .expect("failed to read dist/marketplace/plugins/speq-skill/.mcp.json");

        assert!(
            content.contains("--project-from-cwd"),
            "built .mcp.json should contain --project-from-cwd"
        );
        assert!(
            !content.contains("${PWD}"),
            "built .mcp.json should not contain ${{PWD}}"
        );
        assert!(
            content.contains("claude-code"),
            "built .mcp.json should contain claude-code"
        );
    }
}

mod codex_config {
    use super::*;

    #[test]
    fn mcp_codex_json_uses_project_from_cwd() {
        assert!(read_mcp_template("mcp-codex.json").contains("--project-from-cwd"));
    }

    #[test]
    fn mcp_codex_json_has_no_static_project_path() {
        assert!(!read_mcp_template("mcp-codex.json").contains("${PWD}"));
    }

    #[test]
    fn mcp_codex_json_has_codex_context() {
        assert!(read_mcp_template("mcp-codex.json").contains("codex"));
    }
}
