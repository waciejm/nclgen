use std::{
    collections::{BTreeMap, BTreeSet},
    path::{Component, PathBuf},
};

use anyhow::Context;
use walkdir::WalkDir;

use crate::project::target::ProjectTarget;

pub struct ProjectOutputs {
    target_outputs: BTreeMap<String, TargetOutputs>,
}

impl ProjectOutputs {
    pub fn build(targets: &BTreeMap<String, ProjectTarget>) -> anyhow::Result<Self> {
        // FIXME: validate that outputs dirs don't overlap
        let target_outputs = targets
            .iter()
            .map(|(target_name, target)| {
                let target_outputs = target.build_outputs().with_context(|| {
                    format!("failed to build outputs for target: {}", target_name)
                })?;
                Ok((target_name.clone(), target_outputs))
            })
            .collect::<Result<BTreeMap<_, _>, anyhow::Error>>()?;
        Ok(Self { target_outputs })
    }

    pub fn check(&self) -> anyhow::Result<()> {
        let stale_targets =
            self.target_outputs
                .iter()
                .filter_map(|(target_name, target_outputs)| {
                    match target_outputs.check().with_context(|| {
                        format!("failed to check outputs for target: {}", target_name)
                    }) {
                        Ok(current) => {
                            if current {
                                None
                            } else {
                                Some(Ok(target_name.as_str()))
                            }
                        }
                        Err(e) => Some(Err(e)),
                    }
                })
                .collect::<Result<Vec<_>, _>>()?;
        if stale_targets.is_empty() {
            Ok(())
        } else {
            anyhow::bail!("stale targets: {}", stale_targets.join(", "))
        }
    }

    pub fn write(&self) -> anyhow::Result<()> {
        for (target_name, target_outputs) in &self.target_outputs {
            target_outputs
                .write()
                .with_context(|| format!("failed to write target outputs: {}", target_name))?;
        }
        Ok(())
    }

    pub fn print(&self) {
        for target_outputs in self.target_outputs.values() {
            target_outputs.print();
        }
    }
}

pub struct TargetOutputs {
    outputs_dir: PathBuf,
    outputs: BTreeMap<PathBuf, String>,
}

impl TargetOutputs {
    pub fn build(outputs_dir: PathBuf, outputs: BTreeMap<PathBuf, String>) -> anyhow::Result<Self> {
        for output_path in outputs.keys() {
            let mut empty = true;
            for component in output_path.components() {
                match component {
                    Component::Normal(_) => {
                        empty = false;
                    }
                    Component::CurDir => {
                        anyhow::bail!("output path contains a /./ component: {:?}", output_path)
                    }
                    Component::ParentDir => {
                        anyhow::bail!("output path contains a /../ component: {:?}", output_path)
                    }
                    Component::RootDir | Component::Prefix(_) => {
                        anyhow::bail!("output path is not relative: {:?}", output_path)
                    }
                }
            }
            if empty {
                anyhow::bail!("output path is empty")
            }
        }
        Ok(Self {
            outputs_dir,
            outputs,
        })
    }

    pub fn check(&self) -> anyhow::Result<bool> {
        if !self.outputs_dir.exists() {
            return Ok(false);
        }

        let existing_paths = WalkDir::new(&self.outputs_dir)
            .into_iter()
            .map(|entry| {
                let entry = entry.with_context(|| {
                    format!("failed to walk outputs directory: {:?}", self.outputs_dir)
                })?;
                if entry.file_type().is_symlink() {
                    anyhow::bail!(
                        "outputs dir contains a symlink, which is impossilbe to generate: {:?}",
                        entry.path()
                    )
                }
                let entry_path = entry
                    .path()
                    .strip_prefix(&self.outputs_dir)
                    .with_context(|| {
                        format!(
                            "failed to strip outputs dir prefix from file path: {:?}, {:?}",
                            self.outputs_dir,
                            entry.path()
                        )
                    })?
                    .to_owned();
                Ok((entry_path, entry.file_type()))
            })
            .collect::<Result<Vec<_>, anyhow::Error>>()?
            .into_iter()
            .filter_map(|(entry_path, entry_type)| {
                if entry_type.is_file() {
                    Some(entry_path)
                } else {
                    None
                }
            })
            .collect::<BTreeSet<_>>();

        let generated_paths = self.outputs.keys().cloned().collect::<BTreeSet<_>>();

        let outputs_extra = existing_paths
            .difference(&generated_paths)
            .collect::<Vec<_>>();
        let outputs_missing = generated_paths
            .difference(&existing_paths)
            .collect::<Vec<_>>();
        let outputs_different = generated_paths
            .intersection(&existing_paths)
            .filter_map(|resource_path| {
                let expected_content = self
                    .outputs
                    .get(resource_path)
                    .expect("path taken from generated paths, should always be present");
                let existing_path = self.outputs_dir.join(resource_path);
                if existing_path.exists() {
                    match std::fs::read(&existing_path).with_context(|| {
                        format!("failed to read existing file: {:?}", existing_path)
                    }) {
                        Ok(c) => {
                            if c == expected_content.as_bytes() {
                                None
                            } else {
                                Some(Ok(resource_path))
                            }
                        }
                        Err(e) => Some(Err(e)),
                    }
                } else {
                    Some(Ok(resource_path))
                }
            })
            .collect::<Result<Vec<_>, _>>()?;

        if outputs_extra.is_empty() && outputs_missing.is_empty() && outputs_different.is_empty() {
            Ok(true)
        } else {
            Ok(false)
        }
    }

    pub fn write(&self) -> anyhow::Result<()> {
        if self.outputs_dir.exists() {
            std::fs::remove_dir_all(&self.outputs_dir).with_context(|| {
                format!("failed to remove old outputs dir: {:?}", self.outputs_dir)
            })?;
        }
        std::fs::create_dir(&self.outputs_dir)
            .with_context(|| format!("failed to create outputs dir: {:?}", self.outputs_dir))?;
        for (output_path, output_content) in &self.outputs {
            let full_output_path = self.outputs_dir.join(output_path);
            if let Some(output_parent) = full_output_path.parent()
                && !output_parent.exists()
            {
                std::fs::create_dir_all(output_parent).with_context(|| {
                    format!(
                        "failed to create parent directory for output: {:?}",
                        output_parent,
                    )
                })?;
            }
            std::fs::write(&full_output_path, output_content.as_bytes())
                .with_context(|| format!("failed to write output file: {:?}", full_output_path))?;
        }
        Ok(())
    }

    pub fn print(&self) {
        for (output_path, output_content) in &self.outputs {
            println!(">>> {:?} <<<", output_path.join(output_path));
            println!("{}", output_content);
        }
    }
}
