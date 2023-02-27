#[cfg(feature = "editorconfig")]
use {
    std::path::Path,
    stylua_lib::{editorconfig, format_code, Config, OutputVerification},
};

#[cfg(feature = "editorconfig")]
fn format(input: &str, directory: &str) -> String {
    let config = editorconfig::parse(
        Config::default(),
        &Path::new("tests")
            .join("inputs-editorconfig")
            .join(directory),
    )
    .unwrap();
    format_code(input, config, None, OutputVerification::None).unwrap()
}

// EditorConfig file with the `root` key set to true
#[test]
#[cfg(feature = "editorconfig")]
fn test_root() {
    insta::glob!("inputs-editorconfig/*.lua", |path| {
        let contents = std::fs::read_to_string(path).unwrap();
        insta::assert_snapshot!(format(&contents, path.to_str().unwrap()));
    })
}

// Subdirectory with an EditorConfig file
#[test]
#[cfg(feature = "editorconfig")]
fn test_local() {
    insta::glob!("inputs-editorconfig/local/*.lua", |path| {
        let contents = std::fs::read_to_string(path).unwrap();
        insta::assert_snapshot!(format(&contents, path.to_str().unwrap()));
    })
}

// Subdirectory without an EditorConfig file
#[test]
#[cfg(feature = "editorconfig")]
fn test_global() {
    insta::glob!("inputs-editorconfig/global/*.lua", |path| {
        let contents = std::fs::read_to_string(path).unwrap();
        insta::assert_snapshot!(format(&contents, path.to_str().unwrap()));
    })
}

// Subdirectory with an empty configuration should use defaults
#[test]
#[cfg(feature = "editorconfig")]
fn test_empty_root() {
    insta::glob!("inputs-editorconfig/empty-root/*.lua", |path| {
        let contents = std::fs::read_to_string(path).unwrap();
        insta::assert_snapshot!(format(&contents, path.to_str().unwrap()));
    })
}

// Subdirectory with a StyLua configuration file
#[test]
#[cfg(feature = "editorconfig")]
fn test_stylua_toml() {
    insta::glob!("inputs-editorconfig/stylua-toml/*.lua", |path| {
        // Save file contents
        let original = std::fs::read_to_string(path).unwrap();
        let result = std::panic::catch_unwind(|| {
            let mut cmd = assert_cmd::Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
            cmd.arg("--config-path")
                .arg("tests/inputs-editorconfig/stylua-toml/stylua.toml")
                .arg(path)
                .assert()
                .success();
            std::fs::read_to_string(path).unwrap()
        });
        // Restore file contents
        std::fs::write(path, original).unwrap();
        let contents = result.unwrap();
        insta::assert_snapshot!(&contents);
    })
}
