//! Module used to handle sections that can be fetched from a `walrus::Module`

use crate::utils::errors::CommonError;
use anyhow::Result;
use semver::Version;
use std::borrow::Cow;
use walrus::{IdsToIndices, Module};

// TODO the section name could this could be shared with sdk crate
const SDK_VERSION_SECTION: &'static str = "__holium_sdk_version";

/// Extract from wasm module the holium sdk version that was used
pub fn extract_from_module(wasm_module: &Module) -> Result<Version> {
    // Get sections from name
    let sections = extract_custom_sections_by_name(&wasm_module, SDK_VERSION_SECTION);

    if sections.is_empty() {
        return Err(CommonError::SectionReadError(format!(
            "no sections found in given holium module"
        ))
        .into());
    }
    // Isolate sdk version section
    let section = try_as_one_section(&sections)?;

    // Convert from bytes to Version
    let version = match section {
        Cow::Borrowed(bytes) => as_semver(bytes)?,
        Cow::Owned(vec) => as_semver(&vec)?,
    };

    Ok(version)
}

/// Extracts sections from a wasm module based on a given name
pub fn extract_custom_sections_by_name<'w>(
    wasm_module: &'w Module,
    section_name: &str,
) -> Vec<Cow<'w, [u8]>> {
    let default_ids = IdsToIndices::default();

    // Filter sections by name and fetch data
    let sections = wasm_module
        .customs
        .iter()
        .filter(|(_, section)| section.name() == section_name)
        .map(|s| s.1.data(&default_ids))
        .collect::<Vec<_>>();
    sections
}

/// Convert Vec to single element if there is only one
pub fn try_as_one_section<T: Sized>(sections: &[T]) -> Result<&T> {
    let sections_count = sections.len();

    if sections_count > 1 {
        return Err(CommonError::SectionReadError(format!(
            "found multiple sections with name: {}",
            SDK_VERSION_SECTION
        ))
        .into());
    }

    if sections_count == 0 {
        return Err(CommonError::SectionReadError(format!(
            "found no sections with name: {}",
            SDK_VERSION_SECTION
        ))
        .into());
    }

    Ok(&sections[0])
}

/// Convert version from bytes into `semver::Version`
fn as_semver(version_as_bytes: &[u8]) -> Result<Version> {
    use std::str::FromStr;

    match std::str::from_utf8(version_as_bytes) {
        Ok(str) => Ok(semver::Version::from_str(str)?),
        Err(e) => Err(CommonError::SectionReadError(format!(
            "could not convert version from bytes: {}",
            e
        ))
        .into()),
    }
}
