use std::{ffi::OsString, path::Path, process::Command};

use anyhow::Context;

pub fn nickel_eval<'a>(
    import_paths: impl IntoIterator<Item = &'a Path>,
    inputs: impl IntoIterator<Item = &'a Path>,
    apply_contracts: impl IntoIterator<Item = &'a Path>,
) -> anyhow::Result<Vec<u8>> {
    let cmd = nickel_cmd();

    let mut command = Command::new(&cmd);
    command.arg("eval");

    for import_path in import_paths {
        command.arg("--import-path");
        command.arg(import_path);
    }

    for input in inputs {
        command.arg(input);
    }

    for apply_contract in apply_contracts {
        command.arg("--apply-contract");
        command.arg(apply_contract);
    }

    let nickel_result = command
        .output()
        .with_context(|| format!("failed to run {:?} eval", cmd))?;

    if !nickel_result.status.success() {
        eprintln!(
            "nickel eval error: {}",
            String::from_utf8_lossy(&nickel_result.stderr),
        );
        if let Some(exit_code) = nickel_result.status.code() {
            anyhow::bail!("nickel eval failed with exit code {}", exit_code);
        } else {
            anyhow::bail!("nickel eval failed without exit code")
        };
    }

    Ok(nickel_result.stdout)
}

pub fn nickel_export<'a>(
    import_paths: impl IntoIterator<Item = &'a Path>,
    inputs: impl IntoIterator<Item = &'a Path>,
    output_field: Option<&str>,
) -> anyhow::Result<Vec<u8>> {
    let cmd = nickel_cmd();

    let mut command = Command::new(&cmd);
    command.arg("export");

    for import_path in import_paths {
        command.arg("--import-path");
        command.arg(import_path);
    }

    for input in inputs {
        command.arg(input);
    }

    if let Some(output_field) = output_field {
        command.arg("--field");
        command.arg(output_field);
    }

    let nickel_result = command
        .output()
        .with_context(|| format!("failed to run {:?} export", cmd))?;

    if !nickel_result.status.success() {
        eprintln!(
            "nickel export error: {}",
            String::from_utf8_lossy(&nickel_result.stderr),
        );
        if let Some(exit_code) = nickel_result.status.code() {
            anyhow::bail!("nickel export failed with exit code {}", exit_code);
        } else {
            anyhow::bail!("nickel export failed without exit code")
        };
    }

    Ok(nickel_result.stdout)
}

fn nickel_cmd() -> OsString {
    if let Some(cmd) = std::env::var_os("NCLGEN_NICKEL_PATH") {
        cmd
    } else {
        "nickel".into()
    }
}
