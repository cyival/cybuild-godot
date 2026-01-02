use std::path::Path;

use godot::global::{godot_print, godot_print_rich, godot_warn};

/// Checks if the `cybuild` CLI tool is installed and accessible in the system's PATH.
/// Prints the detected version if found, or a warning if not.
///
/// # Returns
/// `true` if `cybuild` is installed, `false` otherwise.
pub fn check_if_installed(path: &Path) -> bool {
    match std::process::Command::new(path).arg("--version").output() {
        Ok(output) if output.status.success() => {
            let version =
                String::from_utf8(output.stdout).unwrap_or_else(|_| "UNKNOWN".to_string());
            godot_print!("Detected cybuild installed: {}", version.trim());
            true
        }
        Ok(output) => {
            godot_warn!("cybuild returned a non-zero exit code: {}", output.status);
            godot_print_rich!(
                "Check [url=https://github.com/cyival/Cyival.Build]Cybuild repository[/url] for more information."
            );
            false
        }
        Err(err) => {
            godot_warn!("No cybuild installation found! Error: {}", err);
            godot_print_rich!(
                "Check [url=https://github.com/cyival/Cyival.Build]Cybuild repository[/url] for more information."
            );
            false
        }
    }
}
