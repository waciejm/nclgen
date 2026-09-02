use std::path::PathBuf;

use anyhow::Context;

use crate::project::Project;

#[derive(Debug, clap::Args)]
pub struct TargetsCommand {}

impl TargetsCommand {
    pub fn exec(self, project: Option<PathBuf>) -> anyhow::Result<()> {
        let project = Project::resolve(project, None).context("failed to resolve project")?;
        let targets = project
            .get_targets()
            .into_iter()
            .collect::<Vec<_>>()
            .join(", ");
        println!("{}", targets);
        Ok(())
    }
}
