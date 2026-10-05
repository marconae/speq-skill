//! Helpers shared by the integration test crates.

use std::path::Path;
use std::process::Command;
use std::sync::OnceLock;

static MODEL_CACHED: OnceLock<()> = OnceLock::new();

/// Provision the embedding model into the cache directory the binary reads,
/// once per test process.
///
/// Runs the installer's own `provision_embedding_model`, so the pinned model
/// revision lives in `install.sh` only. The files persist in the cache
/// directory, so later runs download nothing while the stamp matches the pin.
pub fn ensure_model_cached() {
    MODEL_CACHED.get_or_init(|| {
        let install_script = Path::new(env!("CARGO_MANIFEST_DIR")).join("install.sh");
        let command = format!(
            "source {} && provision_embedding_model",
            install_script.display()
        );
        let output = Command::new("bash")
            .args(["-c", &command])
            .env("SPEQ_CACHE_DIR", speq_skill::search::get_cache_path())
            .output()
            .expect("run install.sh provision_embedding_model");
        assert!(
            output.status.success(),
            "provisioning the embedding model failed:\n{}{}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
    });
}
