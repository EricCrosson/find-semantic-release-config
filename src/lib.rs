#![forbid(unsafe_code)]
#![deny(warnings)]

//! Locate a project's semantic-release configuration.
//!
//! The configuration rules, according to the [semantic-release readme]:
//!
//! semantic-release’s options, mode and plugins can be set via either:
//!
//! - A `.releaserc` file, written in YAML or JSON, with optional extensions: `.yaml`/`.yml`/`.json`/`.js`/`.cjs`
//! - A `release.config.(js|cjs)` file that exports an object
//! - A `release` key in the project's package.json file
//!
//! [semantic-release readme]:
//!   https://github.com/semantic-release/semantic-release/blob/master/docs/usage/configuration.md#configuration-file

use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};

use log::{debug, trace};

mod error;

pub use error::Error;

fn find_releaserc_file(directory: &Path) -> Option<PathBuf> {
    let basename = ".releaserc";
    let extensions: [&str; 6] = ["", ".yaml", ".yml", ".json", ".js", ".cjs"];

    for extension in extensions {
        let filename = format!("{basename}{extension}");
        let path = directory.join(filename);
        trace!("Looking for configuration file {}", path.display());
        if path.exists() {
            debug!("Found semantic-release configuration at {}", path.display());
            return Some(path);
        }
    }

    None
}

fn find_release_config_file(directory: &Path) -> Option<PathBuf> {
    let basename = "release.config";
    let extensions: [&str; 2] = ["js", "cjs"];

    for extension in extensions {
        let filename = format!("{basename}.{extension}");
        let path = directory.join(filename);
        trace!("Looking for configuration file {}", path.display());
        if path.exists() {
            debug!("Found semantic-release configuration at {}", path.display());
            return Some(path);
        }
    }

    None
}

fn does_package_manifest_have_release_property(directory: &Path) -> Result<bool, Error> {
    trace!("Looking for configuration in package.json");
    let package_manifest_path = directory.join("package.json");
    if !package_manifest_path.exists() {
        return Ok(false);
    }

    // Reading a file into a string before invoking Serde is faster than
    // invoking Serde from a BufReader, see
    // https://github.com/serde-rs/json/issues/160
    let mut string = String::new();
    File::open(&package_manifest_path)
        .map_err(|err| Error::file_open_error(err, &package_manifest_path))?
        .read_to_string(&mut string)
        .map_err(|err| Error::file_read_error(err, &package_manifest_path))?;
    let package_manifest: serde_json::Value = serde_json::from_str(&string)
        .map_err(|err| Error::file_parse_error(err, &package_manifest_path))?;

    Ok(package_manifest
        .as_object()
        .map(|manifest| manifest.contains_key("release"))
        .unwrap_or_default())
}

/// Locate a project's semantic-release configuration.
///
/// The configuration rules, according to the [semantic-release readme]:
///
/// semantic-release’s options, mode and plugins can be set via either:
///
/// - A `.releaserc` file, written in YAML or JSON, with optional extensions: `.yaml`/`.yml`/`.json`/`.js`/`.cjs`
/// - A `release.config.(js|cjs)` file that exports an object
/// - A `release` key in the project's package.json file
///
/// [semantic-release readme]:
///   https://github.com/semantic-release/semantic-release/blob/master/docs/usage/configuration.md#configuration-file
pub fn find_semantic_release_configuration(project_root: &Path) -> Result<Option<PathBuf>, Error> {
    if let Some(config) = find_releaserc_file(project_root) {
        return Ok(Some(config));
    }
    if let Some(config) = find_release_config_file(project_root) {
        return Ok(Some(config));
    }
    if does_package_manifest_have_release_property(project_root)? {
        return Ok(Some(project_root.join("package.json")));
    }

    trace!(
        "Did not find semantic-release configuration in {}",
        project_root.display()
    );
    Ok(None)
}
