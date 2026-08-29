use std::{
    collections::{BTreeMap, BTreeSet},
    path::PathBuf,
    rc::Rc,
};

use anyhow::Context;

use crate::project::{config::Config, outputs::ProjectOutputs, target::ProjectTarget};

mod config;
mod outputs;
mod target;

const PROJECT_DIR: &str = "ncl.gen";

#[derive(Debug)]
pub struct Project {
    import_dirs: Rc<BTreeSet<PathBuf>>,
    targets: BTreeMap<String, ProjectTarget>,
}

impl Project {
    pub fn resolve(
        project_root: Option<PathBuf>,
        targets_filter: Option<&BTreeSet<String>>,
    ) -> anyhow::Result<Self> {
        // FIXME: validate that all targets in targets_filter exist

        let project_root = if let Some(path) = project_root {
            path.canonicalize().with_context(|| {
                format!("failed to canonicalize provided project root {:?}", path)
            })?
        } else {
            find_project().context("failed to find project root")?
        };
        let project_dir = project_root.join(PROJECT_DIR);
        let config = Config::read(&project_dir).context("failed to read project config")?;

        let initial_current_dir = std::env::current_dir().context("failed to get current dir")?;
        std::env::set_current_dir(&project_root).with_context(|| {
            format!(
                "failed to change current dir to project root: {:?}",
                project_root
            )
        })?;
        let restore_current_dir_guard = scopeguard::guard((), |()| {
            let _ = std::env::set_current_dir(&initial_current_dir);
        });

        let common_inputs_before_exclude = config
            .common_inputs
            .iter()
            .map(|i| {
                i.resolve()
                    .with_context(|| format!("failed to resolve common input: {}", i))
            })
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .flatten()
            .collect::<BTreeSet<_>>();
        let common_inputs_exclude = config
            .common_inputs_exclude
            .iter()
            .map(|i| {
                i.resolve()
                    .with_context(|| format!("failed to resolve common input exclude: {}", i))
            })
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .flatten()
            .collect::<BTreeSet<_>>();
        let common_inputs = Rc::new(
            common_inputs_before_exclude
                .difference(&common_inputs_exclude)
                .cloned()
                .collect::<BTreeSet<_>>(),
        );
        let import_dirs_before_exclude = config
            .import_dirs
            .iter()
            .map(|i| {
                i.resolve()
                    .with_context(|| format!("failed to resolve import dir: {}", i))
            })
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .flatten()
            .collect::<BTreeSet<_>>();
        let import_dirs_exclude = config
            .import_dirs_exclude
            .iter()
            .map(|i| {
                i.resolve()
                    .with_context(|| format!("failed to resolve import dir exclude: {}", i))
            })
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .flatten()
            .collect::<BTreeSet<_>>();
        let import_dirs = Rc::new(
            import_dirs_before_exclude
                .difference(&import_dirs_exclude)
                .cloned()
                .collect::<BTreeSet<_>>(),
        );

        let eval_contracts_before_exclude = config
            .eval_contracts
            .iter()
            .map(|i| {
                i.resolve()
                    .with_context(|| format!("failed to resolve eval contracts: {}", i))
            })
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .flatten()
            .collect::<BTreeSet<_>>();
        let eval_contracts_exclude = config
            .eval_contracts_exclude
            .iter()
            .map(|i| {
                i.resolve()
                    .with_context(|| format!("failed to resolve eval contracts exclude: {}", i))
            })
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .flatten()
            .collect::<BTreeSet<_>>();
        let eval_contracts = Rc::new(
            eval_contracts_before_exclude
                .difference(&eval_contracts_exclude)
                .cloned()
                .collect::<BTreeSet<_>>(),
        );

        drop(restore_current_dir_guard);

        let targets = config
            .targets
            .iter()
            .filter(|(target_name, _)| {
                if let Some(targets_filter) = targets_filter {
                    targets_filter.contains(*target_name)
                } else {
                    true
                }
            })
            .map(|(target_name, target_config)| {
                Ok((
                    target_name.clone(),
                    ProjectTarget::resolve(
                        &project_root,
                        Rc::clone(&common_inputs),
                        Rc::clone(&import_dirs),
                        Rc::clone(&eval_contracts),
                        target_config,
                    )
                    .with_context(|| format!("failed to resolve target {}", target_name))?,
                ))
            })
            .collect::<Result<_, anyhow::Error>>()?;

        Ok(Self {
            import_dirs,
            targets,
        })
    }

    pub fn build_outputs(&self) -> anyhow::Result<ProjectOutputs> {
        ProjectOutputs::build(&self.targets)
    }

    pub fn get_import_dirs(&self) -> &BTreeSet<PathBuf> {
        &self.import_dirs
    }
}

fn find_project() -> anyhow::Result<PathBuf> {
    let start = std::fs::canonicalize(".").context("failed to get current dir")?;
    let mut candidate = start.as_path();
    loop {
        let nclgen_path = candidate.join(PROJECT_DIR);
        if nclgen_path.is_dir() {
            return Ok(candidate.to_owned());
        } else if let Some(parent) = candidate.parent() {
            candidate = parent;
            continue;
        } else {
            anyhow::bail!("reached root without finding project dir");
        };
    }
}
