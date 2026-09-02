use std::path::PathBuf;

use anyhow::Context;

use crate::project::Project;

#[derive(Debug, clap::Args)]
pub struct ImportsCommand {}

impl ImportsCommand {
    pub fn exec(self, project: Option<PathBuf>) -> anyhow::Result<()> {
        let project = Project::resolve(project, None).context("failed to resolve project")?;
        let import_dirs_strings = project
            .get_import_dirs()
            .iter()
            .map(|path| match path.to_str() {
                Some(p) => Ok(String::from(p)),
                None => Err(anyhow::anyhow!("import dir path is not UTF-8: {:?}", path)),
            })
            .collect::<Result<Vec<_>, _>>()?;
        println!("{}", import_dirs_strings.join(":"));
        Ok(())
    }
}
