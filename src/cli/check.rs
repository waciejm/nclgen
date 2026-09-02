use std::path::PathBuf;

use anyhow::Context;

use crate::project::Project;

#[derive(Debug, clap::Args)]
pub struct CheckCommand {
    /// Targets to check. Will check all targets if empty.
    #[arg()]
    targets: Vec<String>,
}

impl CheckCommand {
    pub fn exec(self, project: Option<PathBuf>) -> anyhow::Result<()> {
        let targets_filter = if self.targets.is_empty() {
            None
        } else {
            Some(self.targets.into_iter().collect())
        };
        let outputs = Project::resolve(project, targets_filter.as_ref())
            .context("failed to resolve project")?
            .build_outputs()
            .context("failed to build project outputs")?;
        outputs.check()?;
        Ok(())
    }
}
