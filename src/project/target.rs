use std::{
    collections::{BTreeMap, BTreeSet},
    path::{Path, PathBuf},
    rc::Rc,
};

use anyhow::{self, Context};

use crate::{
    external::nickel::{nickel_eval, nickel_export},
    project::{config::ConfigTarget, outputs::TargetOutputs},
};

#[derive(Debug)]
pub struct ProjectTarget {
    common_inputs: Rc<BTreeSet<PathBuf>>,
    import_dirs: Rc<BTreeSet<PathBuf>>,
    eval_contracts: Rc<BTreeSet<PathBuf>>,
    inputs: BTreeSet<PathBuf>,
    outputs_field: Option<String>,
    outputs_dir: PathBuf,
}

impl ProjectTarget {
    pub fn resolve(
        project_root: &Path,
        common_inputs: Rc<BTreeSet<PathBuf>>,
        import_dirs: Rc<BTreeSet<PathBuf>>,
        eval_contracts: Rc<BTreeSet<PathBuf>>,
        config: &ConfigTarget,
    ) -> anyhow::Result<Self> {
        let initial_current_dir = std::env::current_dir().context("failed to get current dir")?;
        std::env::set_current_dir(project_root).with_context(|| {
            format!(
                "failed to change current dir to project root: {:?}",
                project_root
            )
        })?;
        let restore_current_dir_guard = scopeguard::guard((), |()| {
            let _ = std::env::set_current_dir(&initial_current_dir);
        });

        let inputs_before_exclude = config
            .inputs
            .iter()
            .map(|i| {
                i.resolve()
                    .with_context(|| format!("failed to resolve input: {}", i))
            })
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .flatten()
            .collect::<BTreeSet<_>>();
        let inputs_exclude = config
            .inputs_exclude
            .iter()
            .map(|i| {
                i.resolve()
                    .with_context(|| format!("failed to resolve input exclude: {}", i))
            })
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .flatten()
            .collect::<BTreeSet<_>>();
        let inputs = inputs_before_exclude
            .difference(&inputs_exclude)
            .cloned()
            .collect::<BTreeSet<_>>();

        drop(restore_current_dir_guard);

        Ok(Self {
            common_inputs,
            import_dirs,
            eval_contracts,
            inputs,
            outputs_field: if config.outputs_field.is_empty() {
                None
            } else {
                Some(config.outputs_field.clone())
            },
            outputs_dir: project_root.join(&config.outputs_dir),
        })
    }

    pub fn build_outputs(&self) -> anyhow::Result<TargetOutputs> {
        if !self.eval_contracts.is_empty() {
            nickel_eval(
                self.import_dirs.iter().map(PathBuf::as_path),
                self.common_inputs
                    .iter()
                    .map(PathBuf::as_path)
                    .chain(self.inputs.iter().map(PathBuf::as_path)),
                self.eval_contracts.iter().map(PathBuf::as_path),
                None,
            )
            .context("failed to evaluate before generating outputs")?;
        }
        let nickel_export_output = nickel_export(
            self.import_dirs.iter().map(PathBuf::as_path),
            self.common_inputs
                .iter()
                .map(PathBuf::as_path)
                .chain(self.inputs.iter().map(PathBuf::as_path)),
            self.outputs_field.as_deref(),
        )
        .context("failed to generate outputs with nickel export")?;
        let raw_outputs = serde_json::from_slice::<BTreeMap<PathBuf, String>>(
            &nickel_export_output,
        )
        .context(
            "failed to parse nickel export output, expecting a record of strings ({ _ | String })",
        )?;
        let outputs = TargetOutputs::build(self.outputs_dir.clone(), raw_outputs)
            .context("failed to build target outputs")?;
        Ok(outputs)
    }

    pub fn debug_eval(&self, field: Option<&str>) -> anyhow::Result<String> {
        let output = nickel_eval(
            self.import_dirs.iter().map(PathBuf::as_path),
            self.common_inputs
                .iter()
                .map(PathBuf::as_path)
                .chain(self.inputs.iter().map(PathBuf::as_path)),
            [],
            field,
        )
        .context("debug evalutaion failed")?;
        Ok(String::from_utf8_lossy(&output).into_owned())
    }
}
