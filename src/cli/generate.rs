use anyhow::Context;

use crate::{cli::ProjectOptions, project::Project};

#[derive(Debug, clap::Args)]
pub struct GenerateCommand {
    #[command(flatten)]
    project: ProjectOptions,
    /// Preview what will be generated without modifying any files.
    #[arg(long)]
    preview: bool,
    /// Targets to generate. Will generate all targets if empty.
    #[arg()]
    targets: Vec<String>,
}

impl GenerateCommand {
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
        if self.preview {
            outputs.print();
        } else {
            outputs.write().context("failed to write outputs")?;
        }
        Ok(())
    }
}
