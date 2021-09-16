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

#[cfg(test)]
mod tests {
    use super::*;
    use walrus::CustomSection;

    // Utils
    #[derive(Debug, Clone)]
    pub(super) struct VersionCustomSection(String);

    impl CustomSection for VersionCustomSection {
        fn name(&self) -> &str {
            SDK_VERSION_SECTION
        }

        fn data(&self, _ids_to_indices: &IdsToIndices) -> Cow<'_, [u8]> {
            Cow::Borrowed(self.0.as_bytes())
        }
    }

    // Embed a section in a module. In our case it's a version section.
    pub fn embed_from_module(
        mut wasm_module: walrus::Module,
        version: &semver::Version,
    ) -> walrus::Module {
        let custom = VersionCustomSection(version.to_string());
        wasm_module.customs.add(custom);

        wasm_module
    }

    // Tests

    #[test]
    fn cannot_isolate_element_from_multiple_elements_slice() {
        let vector: Vec<u32> = vec![10, 20];

        let result = try_as_one_section(&vector);

        assert!(result.is_err());
    }

    #[test]
    fn cannot_isolate_element_from_no_elements_slice() {
        let vector: Vec<u32> = vec![];

        let result = try_as_one_section(&vector);

        assert!(result.is_err());
    }

    #[test]
    fn can_isolate_element_from_one_element_slice() {
        let vector: Vec<u32> = vec![10];

        let element = try_as_one_section(&vector).unwrap();

        assert_eq!(&vector[0], element);
    }

    #[test]
    fn cannot_convert_non_utf8_bytes_to_version() {
        let malformed_slice: Vec<u8> = vec![0, 159, 146, 150];

        let conversion = as_semver(&malformed_slice);

        assert!(conversion.is_err());
    }

    #[test]
    fn cannot_convert_malformed_bytes_to_version() {
        let malformed_str = "I am str !";

        let conversion = as_semver(malformed_str.as_bytes());

        assert!(conversion.is_err());
    }

    #[test]
    fn can_convert_bytes_to_version() {
        use std::str::FromStr;

        let version_str = "1.0.0-alpha";
        let version = semver::Version::from_str(&version_str).unwrap();

        let conversion = as_semver(version_str.as_bytes()).unwrap();

        assert_eq!(version, conversion);
    }

    #[test]
    fn can_get_sections_from_module() {
        use std::str::FromStr;

        let module = Module::default();
        let module_w_section =
            embed_from_module(module, &semver::Version::from_str("1.0.0-alpha").unwrap());

        let sections = extract_custom_sections_by_name(&module_w_section, SDK_VERSION_SECTION);

        assert_eq!(sections.len(), 1);
    }

    #[test]
    fn can_get_version_from_module() {
        use std::str::FromStr;

        let module = Module::default();
        let version = semver::Version::from_str("1.0.0-alpha").unwrap();
        let module_w_section = embed_from_module(module, &version);

        let extracted_version = extract_from_module(&module_w_section).unwrap();

        assert_eq!(version, extracted_version);
    }
}
