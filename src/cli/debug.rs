use std::{collections::BTreeSet, path::PathBuf};

use anyhow::Context;

use crate::project::Project;

#[derive(Debug, clap::Args)]
pub struct DebugCommand {
    /// Target to evaluate.
    #[arg()]
    target: String,
    /// Field to evaluate. Will evaluate the whole target if skipped.
    #[arg()]
    field: Option<String>,
}

impl DebugCommand {
    pub fn exec(self, project: Option<PathBuf>) -> anyhow::Result<()> {
        let output = Project::resolve(project, Some(&BTreeSet::from([self.target.clone()])))
            .context("failed to resolve project")?
            .debug_eval(&self.target, self.field.as_deref())
            .context("failed to build project outputs")?;
        println!("{}", output);
        Ok(())
    }
}
