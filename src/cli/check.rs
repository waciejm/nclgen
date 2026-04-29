use anyhow::Context;

use crate::{cli::ProjectOptions, project::Project};

#[derive(Debug, clap::Args)]
pub struct CheckCommand {
    #[command(flatten)]
    project: ProjectOptions,
    /// Targets to check. Will check all targets if empty.
    #[arg()]
    targets: Vec<String>,
}

impl CheckCommand {
    pub fn exec(self) -> anyhow::Result<()> {
        let targets_filter = if self.targets.is_empty() {
            None
        } else {
            Some(self.targets.into_iter().collect())
        };
        let outputs = Project::resolve(self.project.project, targets_filter.as_ref())
            .context("failed to resolve project")?
            .build_outputs()
            .context("failed to build project outputs")?;
        outputs.check()?;
        Ok(())
    }
}
