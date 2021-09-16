use anyhow::Result;
use std::fs;

pub(crate) fn info(args: &clap::ArgMatches<'_>) -> Result<()> {
    use std::str::FromStr;

    // Safe because input is required
    let input_str = args.value_of("INPUT").unwrap();
    let input_path = fs::canonicalize(std::path::PathBuf::from_str(input_str)?)?;

    // Instantiate wasm module as walrus module
    let wasm_module = walrus::ModuleConfig::new().parse_file(input_path).unwrap();

    // Extract section value from walrus module
    let sdk_version = crate::utils::sections::extract_from_module(&wasm_module)?;

    // Print success message
    println!("Holium SDK Version: {}", sdk_version.to_string());

    Ok(())
}
