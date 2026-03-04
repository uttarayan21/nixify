use nixify::parse_yaml;
use std::fs;
use std::path::PathBuf;

#[test]
fn test_config_yaml_parseable() {
    let path = PathBuf::from("tests/config.yaml");

    assert!(path.exists(), "config.yaml should exist");

    let content = fs::read_to_string(&path).expect("Failed to read config.yaml");

    let result = parse_yaml(&content);

    assert!(
        result.is_ok(),
        "config.yaml should be parseable; {:?}",
        result.err()
    );
}
