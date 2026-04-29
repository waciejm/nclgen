use anyhow::Context;

use crate::{cli::ProjectOptions, project::Project};

#[derive(Debug, clap::Args)]
pub struct ImportsCommand {
    #[command(flatten)]
    project: ProjectOptions,
}

impl ImportsCommand {
    pub fn exec(self) -> anyhow::Result<()> {
        let project =
            Project::resolve(self.project.project, None).context("failed to resolve project")?;
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
