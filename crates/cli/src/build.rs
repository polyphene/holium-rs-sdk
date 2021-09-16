use crate::errors::CommonError;
use anyhow::Result;
use std::process::Command;
use std::{env, fs};

#[derive(serde::Deserialize)]
struct CompilerArtifact {
    reason: String,
    package_id: String,
    manifest_path: String,
    target: Target,
    filenames: Vec<String>,
}

#[derive(serde::Deserialize)]
struct Target {
    kind: Vec<String>,
    crate_types: Vec<String>,
    name: String,
    src_path: String,
}

/// Function meant to build a Wasm bytecode file from a Rust project
pub(crate) fn build(args: &clap::ArgMatches<'_>) -> Result<()> {
    use std::io::Read;
    use std::str::FromStr;

    let trailing_args: Vec<&str> = args.values_of("optional").unwrap_or_default().collect();

    // Prepare and run cargo build
    let mut cargo = Command::new("cargo");
    cargo
        .arg("build")
        .arg("--target")
        .arg("wasm32-unknown-unknown");
    cargo.arg("--message-format").arg("json-render-diagnostics");
    cargo.args(trailing_args);

    let mut process = cargo.stdout(std::process::Stdio::piped()).spawn()?;

    let mut output = String::new();

    process
        .stdout
        .take()
        .ok_or_else(|| {
            CommonError::WasmCompilationError("Compilation failed: no output".to_string())
        })?
        .read_to_string(&mut output)?;

    let status = process.wait()?;
    if !status.success() {
        return Err(CommonError::WasmCompilationError(format!(
            "Compilation failed with status {}",
            status
        ))
        .into());
    }

    // Fetch generated Wasm files from build output
    let mut wasms: Vec<String> = Vec::new();
    for line in output.lines() {
        if let Ok(CompilerArtifact { filenames, .. }) = serde_json::from_str(line) {
            wasms.extend(
                filenames
                    .into_iter()
                    .filter(|name| name.ends_with(".wasm"))
                    .collect::<Vec<_>>(),
            )
        }
    }

    if wasms.is_empty() {
        // it is possible to build a object file without Wasm artifacts
        return Ok(());
    }

    // Copy wasm artifacts to artifacts folder
    for wasm in wasms {
        let wasm_path = std::path::PathBuf::from(wasm);
        let mut path = env::current_dir()?;
        path.push("artifacts");
        if !path.exists() {
            fs::create_dir(&path)?;
        }
        if wasm_path.is_file() {
            path.push(&wasm_path.file_name().unwrap());
            fs::copy(&wasm_path, &path)?;
        }
    }

    Ok(())
}
