//! The speq plugins do not configure Serena or Context7. Serena comes from the
//! user's global install, Context7 is optional, and the skills find their
//! tools at run time.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// Every file under `dir`, recursively.
fn files_under(dir: &Path) -> Vec<PathBuf> {
    let mut files = Vec::new();
    for entry in fs::read_dir(dir).unwrap_or_else(|e| panic!("read {}: {e}", dir.display())) {
        let path = entry.unwrap().path();
        if path.is_dir() {
            files.extend(files_under(&path));
        } else {
            files.push(path);
        }
    }
    files
}

#[test]
fn built_plugins_ship_no_mcp_config_and_no_plugin_scoped_tool_names() {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let status = Command::new("bash")
        .arg("scripts/plugin/build.sh")
        .current_dir(manifest_dir)
        .output()
        .expect("failed to run scripts/plugin/build.sh")
        .status;
    assert!(status.success(), "build.sh exited with failure: {status}");

    let marketplace = Path::new(manifest_dir).join("dist/marketplace");
    let plugin_dirs = [
        marketplace.join("plugins/speq-skill"),
        marketplace.join("codex/plugins/speq-skill"),
    ];

    for plugin_dir in &plugin_dirs {
        assert!(
            !plugin_dir.join(".mcp.json").exists(),
            "{} must not ship an MCP config",
            plugin_dir.display()
        );

        // A hard-coded plugin namespace (`mcp__plugin_speq_serena__*`) stops
        // working once the servers come from the global install.
        for file in files_under(plugin_dir) {
            let Ok(content) = fs::read_to_string(&file) else {
                continue;
            };
            assert!(
                !content.contains("mcp__plugin_speq_"),
                "{} names a plugin-scoped MCP tool",
                file.display()
            );
        }
    }
}
