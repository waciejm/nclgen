use std::{
    collections::{BTreeMap, BTreeSet},
    fmt::Display,
    path::{Path, PathBuf},
};

use anyhow::Context;
use serde::Deserialize;

const CONFIG_PATH: &str = "config.toml";

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Config {
    /// Glob pattern of files which should be merged in the evaluaton for every target.
    ///
    /// Relative to project root.
    pub common_inputs: Vec<GlobPattern>,
    /// Glob pattern of files which will be excluded from `inputs` matches.
    ///
    /// Relative to project root.
    #[serde(default)]
    pub common_inputs_exclude: Vec<GlobPattern>,
    /// Glob pattern of dirs which should be added to import dirs for all targets.
    ///
    /// Relative to project root.
    #[serde(default)]
    pub import_dirs: Vec<GlobPattern>,
    /// Glob pattern of dirs which will be excluded from `import_dirs` matches.
    ///
    /// Relative to project root.
    #[serde(default)]
    pub import_dirs_exclude: Vec<GlobPattern>,
    /// Contracts that will be applied to the evaluation for the whole target.
    ///
    /// If list not empty, before executing `nickel export` to obtain target outputs,
    /// `nclgen` will execute `nickel eval` to evaluate the same inputs that will
    /// be used for `nickel export`, but will also apply these contracts to the
    /// value of the whole evaluation.
    ///
    /// If list empty, only the value of `outputs_field` for a given target
    /// will be evaluated. Other field might not be evaluated and have contracts
    /// applied that would fail if evaluated, but will be silently ignored instead.
    ///
    /// Relative to project root.
    #[serde(default)]
    pub eval_contracts: Vec<GlobPattern>,
    /// Glob pattern of files which will be excluded from `eval_contracts` matches.
    ///
    /// Relative to project root.
    #[serde(default)]
    pub eval_contracts_exclude: Vec<GlobPattern>,
    /// Targets for evaluating and generating resources.
    pub targets: BTreeMap<String, ConfigTarget>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ConfigTarget {
    /// Glob pattern of files which should be merged in the evaluaton for this target.
    ///
    /// Relative to project root.
    pub inputs: Vec<GlobPattern>,
    /// Glob pattern of files which will be excluded from `inputs` matches.
    ///
    /// Relative to project root.
    #[serde(default)]
    pub inputs_exclude: Vec<GlobPattern>,
    /// Which field in the evaluation will contain the outputs for this target.
    /// Passed as argument to `nickel export` via the `--field` flag.
    /// Empty string means no `--field` flag, treats the whole result as outputs.
    #[serde(default)]
    pub outputs_field: String,
    /// Path under which to place the outputs of this target.
    ///
    /// Relative to project root.
    pub outputs_dir: PathBuf,
}

#[derive(Debug, Deserialize)]
pub struct GlobPattern(pub String);

impl GlobPattern {
    pub fn resolve(&self) -> anyhow::Result<BTreeSet<PathBuf>> {
        let paths = glob::glob(&self.0)
            .with_context(|| format!("invalid glob pattern: {}", self.0))?
            .map(|pr| {
                pr.with_context(|| format!("failed to resolve glob pattern: {}", self.0))
                    .and_then(|path| {
                        path.canonicalize().with_context(|| {
                            format!("failed to canonicalize glob match: {:?}", path)
                        })
                    })
            })
            .collect::<Result<_, _>>()?;
        Ok(paths)
    }
}

impl Display for GlobPattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Config {
    pub fn read(project_dir: &Path) -> anyhow::Result<Self> {
        let config_path = project_dir.join(CONFIG_PATH);
        let config_file = std::fs::read(&config_path)
            .with_context(|| format!("failed to read config at {:?}", config_path))?;
        let config = toml::from_slice(&config_file)
            .with_context(|| format!("failed to parse config at {:?}", config_path))?;
        Ok(config)
    }
}
