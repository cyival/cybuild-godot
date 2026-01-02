use anyhow::Result;
use std::ffi::OsStr;
use std::path::PathBuf;
use std::process::Command;

/// Name of the cybuild executable.
const EXECUTABLE_NAME: &str = "cybuild";

/// Wrapper for invoking the cybuild CLI.
pub struct CliWrapper {
    executable_path: Option<PathBuf>,
    manifest_path: Option<PathBuf>,
    output_path: Option<PathBuf>,
}

impl CliWrapper {
    /// Creates a new CLI wrapper with no paths set.
    pub fn new() -> Self {
        Self {
            executable_path: None,
            manifest_path: None,
            output_path: None,
        }
    }

    /// Sets the path to the cybuild executable.
    pub fn executable<P: AsRef<OsStr>>(&mut self, path: P) -> &mut Self {
        self.executable_path = Some(PathBuf::from(path.as_ref()));
        self
    }

    /// Sets the manifest path.
    pub fn manifest<P: AsRef<OsStr>>(&mut self, path: P) -> &mut Self {
        self.manifest_path = Some(PathBuf::from(path.as_ref()));
        self
    }

    /// Sets the output path.
    pub fn output<P: AsRef<OsStr>>(&mut self, path: P) -> &mut Self {
        self.output_path = Some(PathBuf::from(path.as_ref()));
        self
    }

    /// Builds the default target.
    pub fn build_default(&self) -> Result<bool> {
        self.build_target("")
    }

    /// Builds all targets.
    pub fn build_all(&self) -> Result<bool> {
        self.build_target("all")
    }

    /// Builds a specific target.
    pub fn build_target(&self, target: &str) -> Result<bool> {
        let mut cybuild = self.prepare_command();

        cybuild.arg("build");

        if let Some(manifest) = &self.manifest_path {
            cybuild.arg(manifest);
        }
        if let Some(output) = &self.output_path {
            cybuild.arg("-o").arg(output);
        }

        cybuild.arg("-t").arg(target);

        let status = cybuild.status()?;
        Ok(status.success())
    }

    /// Builds only dependencies for a target.
    pub fn build_dependencies(&self, target: &str) -> Result<bool> {
        let mut cybuild = self.prepare_command();

        cybuild.arg("build");

        if let Some(manifest) = &self.manifest_path {
            cybuild.arg(manifest);
        }
        if let Some(output) = &self.output_path {
            cybuild.arg("-o").arg(output);
        }

        cybuild.arg("-t").arg(target);
        cybuild.arg("--dep-only");

        let status = cybuild.status()?;
        Ok(status.success())
    }

    /// Prepares the Command for cybuild, using the executable path if set.
    fn prepare_command(&self) -> Command {
        if let Some(exe) = &self.executable_path {
            Command::new(exe)
        } else {
            Command::new(EXECUTABLE_NAME)
        }
    }
}
