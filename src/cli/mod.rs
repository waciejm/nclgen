use std::path::PathBuf;

mod check;
mod debug;
mod generate;
mod imports;
mod targets;

/// Module-based resource generation with Nickel.
#[derive(Debug, clap::Parser)]
#[command(version)]
pub struct Cli {
    /// Path to `nclgen` project root.
    ///
    /// If not specified `nclgen` will search the current and then parent directories
    /// until it finds a directory containing a `ncl.gen` subdirectory.
    #[arg(long, short = 'p', global = true)]
    project: Option<PathBuf>,
    #[command(subcommand)]
    subcommand: Subcommand,
}

#[derive(Debug, clap::Subcommand)]
pub enum Subcommand {
    /// Generate resources.
    Generate(generate::GenerateCommand),
    /// Check if generated resources and are up-to-date.
    Check(check::CheckCommand),
    /// Print a colon-separated list of import directories.
    Imports(imports::ImportsCommand),
    // Print a list of available targets.
    Targets(targets::TargetsCommand),
    /// Evaluate and print a target.
    Debug(debug::DebugCommand),
}

impl Cli {
    pub fn exec(self) -> anyhow::Result<()> {
        match self.subcommand {
            Subcommand::Generate(generate) => generate.exec(self.project),
            Subcommand::Check(check) => check.exec(self.project),
            Subcommand::Imports(imports) => imports.exec(self.project),
            Subcommand::Targets(targets) => targets.exec(self.project),
            Subcommand::Debug(debug) => debug.exec(self.project),
        }
    }
}
