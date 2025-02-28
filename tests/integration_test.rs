use std::path::PathBuf;
use find_semantic_release_config::find_semantic_release_configuration;

fn test_dir(name: &str) -> PathBuf {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("tests");
    path.push("test_data");
    path.push(name);
    path
}

#[test]
fn test_find_releaserc_no_extension() {
    let dir = test_dir("releaserc_no_extension");
    let result = find_semantic_release_configuration(&dir).unwrap();
    assert_eq!(result, Some(dir.join(".releaserc")));
}

#[test]
fn test_find_releaserc_yaml() {
    let dir = test_dir("releaserc_yaml");
    let result = find_semantic_release_configuration(&dir).unwrap();
    assert_eq!(result, Some(dir.join(".releaserc.yaml")));
}

#[test]
fn test_find_releaserc_yml() {
    let dir = test_dir("releaserc_yml");
    let result = find_semantic_release_configuration(&dir).unwrap();
    assert_eq!(result, Some(dir.join(".releaserc.yml")));
}

#[test]
fn test_find_releaserc_json() {
    let dir = test_dir("releaserc_json");
    let result = find_semantic_release_configuration(&dir).unwrap();
    assert_eq!(result, Some(dir.join(".releaserc.json")));
}

#[test]
fn test_find_releaserc_js() {
    let dir = test_dir("releaserc_js");
    let result = find_semantic_release_configuration(&dir).unwrap();
    assert_eq!(result, Some(dir.join(".releaserc.js")));
}

#[test]
fn test_find_releaserc_cjs() {
    let dir = test_dir("releaserc_cjs");
    let result = find_semantic_release_configuration(&dir).unwrap();
    assert_eq!(result, Some(dir.join(".releaserc.cjs")));
}

#[test]
fn test_find_release_config_js() {
    let dir = test_dir("release_config_js");
    let result = find_semantic_release_configuration(&dir).unwrap();
    assert_eq!(result, Some(dir.join("release.config.js")));
}

#[test]
fn test_find_release_config_cjs() {
    let dir = test_dir("release_config_cjs");
    let result = find_semantic_release_configuration(&dir).unwrap();
    assert_eq!(result, Some(dir.join("release.config.cjs")));
}

#[test]
fn test_package_json_with_release() {
    let dir = test_dir("package_json_with_release");
    let result = find_semantic_release_configuration(&dir).unwrap();
    assert_eq!(result, Some(dir.join("package.json")));
}

#[test]
fn test_package_json_without_release() {
    let dir = test_dir("package_json_without_release");
    let result = find_semantic_release_configuration(&dir).unwrap();
    assert_eq!(result, None);
}

#[test]
fn test_no_config() {
    let dir = test_dir("no_config");
    let result = find_semantic_release_configuration(&dir).unwrap();
    assert_eq!(result, None);
}

#[test]
fn test_package_json_invalid() {
    let dir = test_dir("package_json_invalid");
    let result = find_semantic_release_configuration(&dir);
    assert!(result.is_err());
}

#[test]
fn test_precedence_releaserc_over_others() {
    let dir = test_dir("precedence_releaserc");
    let result = find_semantic_release_configuration(&dir).unwrap();
    assert_eq!(result, Some(dir.join(".releaserc")));
}

#[test]
fn test_precedence_config_over_package() {
    let dir = test_dir("precedence_config");
    let result = find_semantic_release_configuration(&dir).unwrap();
    assert_eq!(result, Some(dir.join("release.config.js")));
}