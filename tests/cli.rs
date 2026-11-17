use assert_cmd::Command;
use assert_fs::prelude::*;

macro_rules! construct_tree {
    ({ $($file_name:literal:$file_contents:literal,)* }) => {{
        let cwd = assert_fs::TempDir::new().unwrap();

        $(
            cwd.child($file_name).write_str($file_contents).unwrap();
        )*

        cwd
    }};
}

fn create_stylua() -> Command {
    Command::new(assert_cmd::cargo::cargo_bin!("stylua"))
}

fn nested_function_source(depth: usize, close: bool) -> String {
    let mut source = String::new();
    for i in 1..=depth {
        source.push_str(&format!("function F{i}()\n"));
    }
    if close {
        for _ in 0..depth {
            source.push_str("end\n");
        }
    }
    source
}

#[test]
fn deeply_nested_functions_check_and_format() {
    let cwd = assert_fs::TempDir::new().unwrap();
    cwd.child("nested.lua")
        .write_str(&nested_function_source(100, true))
        .unwrap();

    // Unformatted but valid: --check reports a diff (exit 1) and must not abort.
    create_stylua()
        .current_dir(cwd.path())
        .args(["--check", "nested.lua"])
        .assert()
        .code(1);

    create_stylua()
        .current_dir(cwd.path())
        .arg("nested.lua")
        .assert()
        .success();

    create_stylua()
        .current_dir(cwd.path())
        .args(["--check", "nested.lua"])
        .assert()
        .success();

    cwd.close().unwrap();
}

#[test]
fn deeply_unterminated_functions_are_a_parse_error() {
    let cwd = assert_fs::TempDir::new().unwrap();
    cwd.child("open.lua")
        .write_str(&nested_function_source(100, false))
        .unwrap();

    let output = create_stylua()
        .current_dir(cwd.path())
        .args(["--check", "open.lua"])
        .output()
        .unwrap();

    assert_eq!(
        output.status.code(),
        Some(2),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("error parsing"),
        "expected a parse error, got {}",
        stderr
    );

    cwd.close().unwrap();
}

#[cfg(feature = "luau")]
#[test]
fn interpolated_nested_functions_do_not_abort() {
    let mut body = "nil".to_string();
    for _ in 0..100 {
        body = format!("function() local a = {body} end");
    }
    let cwd = assert_fs::TempDir::new().unwrap();
    cwd.child("interp.lua")
        .write_str(&format!("local x = `{{{body}}}`\n"))
        .unwrap();

    let output = create_stylua()
        .current_dir(cwd.path())
        .args(["--syntax", "luau", "--check", "interp.lua"])
        .output()
        .unwrap();
    let code = output.status.code();
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        code == Some(0) || code == Some(1),
        "interpolated nesting should format or diff, got exit {:?} stderr {}",
        code,
        stderr
    );
    assert!(
        !stderr.contains("overflowed its stack"),
        "interpolated nesting aborted: {}",
        stderr
    );

    cwd.close().unwrap();
}

#[cfg(any(feature = "lua52", feature = "luau"))]
#[test]
fn z_escape_nested_functions_do_not_abort() {
    const SEGMENT: usize = 40;
    const SEGMENTS: usize = 4;
    let mut source = String::new();
    for segment in 0..SEGMENTS {
        for i in 0..SEGMENT {
            source.push_str(&format!("function S{segment}_{i}()\n"));
        }
        source.push_str("local s = \"\\z\n");
        for _ in 0..SEGMENT {
            source.push_str("end ");
        }
        source.push_str("\"\n");
    }
    for _ in 0..(SEGMENT * SEGMENTS) {
        source.push_str("end\n");
    }

    let cwd = assert_fs::TempDir::new().unwrap();
    cwd.child("zreset.lua").write_str(&source).unwrap();

    let output = create_stylua()
        .current_dir(cwd.path())
        .args(["--check", "zreset.lua"])
        .output()
        .unwrap();
    let code = output.status.code();
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        code == Some(0) || code == Some(1),
        "\\z nesting should format or diff, got exit {:?} stderr {}",
        code,
        stderr
    );
    assert!(
        !stderr.contains("overflowed its stack"),
        "\\z nesting aborted: {}",
        stderr
    );

    cwd.close().unwrap();
}

#[test]
fn test_no_files_provided() {
    let mut cmd = create_stylua();
    cmd.assert()
        .failure()
        .code(2)
        .stderr("error: no files provided\n");
}

#[test]
fn test_format_stdin() {
    let mut cmd = create_stylua();
    cmd.arg("-")
        .write_stdin("local   x   = 1")
        .assert()
        .success()
        .stdout("local x = 1\n");
}

#[test]
fn test_format_file() {
    let cwd = construct_tree!({
        "foo.lua": "local   x    =   1",
    });

    let mut cmd = create_stylua();
    cmd.current_dir(cwd.path()).arg(".").assert().success();

    cwd.child("foo.lua").assert("local x = 1\n");

    cwd.close().unwrap();
}

#[test]
fn test_stylua_ignore() {
    let cwd = construct_tree!({
        ".styluaignore": "ignored/",
        "foo.lua": "local   x    =   1",
        "ignored/bar.lua": "local   x    =   1",
    });

    let mut cmd = create_stylua();
    cmd.current_dir(cwd.path()).arg(".").assert().success();

    cwd.child("foo.lua").assert("local x = 1\n");
    cwd.child("ignored/bar.lua").assert("local   x    =   1");

    cwd.close().unwrap();
}

#[test]
fn explicitly_provided_files_dont_check_ignores() {
    let cwd = construct_tree!({
        ".styluaignore": "foo.lua",
        "foo.lua": "local   x    =   1",
    });

    let mut cmd = create_stylua();
    cmd.current_dir(cwd.path())
        .arg("foo.lua")
        .assert()
        .success();

    cwd.child("foo.lua").assert("local x = 1\n");

    cwd.close().unwrap();
}

#[test]
fn explicitly_provided_files_dont_check_ignores_stdin() {
    let cwd = construct_tree!({
        ".styluaignore": "foo.lua",
    });

    let mut cmd = create_stylua();
    cmd.current_dir(cwd.path())
        .args(["--stdin-filepath", "foo.lua", "-"])
        .write_stdin("local   x    =   1")
        .assert()
        .success()
        .stdout("local x = 1\n");

    cwd.close().unwrap();
}

#[test]
fn explicitly_provided_files_not_in_cwd() {
    let cwd = construct_tree!({
        ".styluaignore": "foo.lua",
    });

    let another = construct_tree!({});

    let mut cmd = create_stylua();
    cmd.current_dir(cwd.path())
        .args([
            "--respect-ignores",
            "--stdin-filepath",
            another.child("foo.lua").to_str().unwrap(),
            "-",
        ])
        .write_stdin("local   x    =   1")
        .assert()
        .success()
        .stdout("local x = 1\n");

    cwd.close().unwrap();
}

#[test]
fn test_respect_ignores() {
    let cwd = construct_tree!({
        ".styluaignore": "foo.lua",
        "foo.lua": "local   x    =   1",
    });

    let mut cmd = create_stylua();
    cmd.current_dir(cwd.path())
        .args(["--respect-ignores", "foo.lua"])
        .assert()
        .success();

    cwd.child("foo.lua").assert("local   x    =   1");

    cwd.close().unwrap();
}

#[test]
fn test_respect_ignores_stdin() {
    let cwd = construct_tree!({
        ".styluaignore": "foo.lua",
    });

    let mut cmd = create_stylua();
    cmd.current_dir(cwd.path())
        .args(["--respect-ignores", "--stdin-filepath", "foo.lua", "-"])
        .write_stdin("local   x    =   1")
        .assert()
        .success()
        .stdout("local   x    =   1");

    cwd.close().unwrap();
}

#[test]
fn test_respect_ignores_directory_no_glob() {
    // https://github.com/JohnnyMorganz/StyLua/issues/845
    let cwd = construct_tree!({
        ".styluaignore": "build/",
        "build/foo.lua": "local   x    =   1",
    });

    let mut cmd = create_stylua();
    cmd.current_dir(cwd.path())
        .args(["--check", "--respect-ignores", "build/foo.lua"])
        .assert()
        .success();

    cwd.close().unwrap();
}

#[test]
fn test_formatting_respects_gitignore() {
    let cwd = construct_tree!({
        ".git/dummy.txt": "", // Need a .git folder for .gitignore lookup
        ".gitignore": "foo.lua",
        "foo.lua": "local   x    =   1",
    });

    let mut cmd = create_stylua();
    cmd.current_dir(cwd.path()).args(["."]).assert().success();

    cwd.child("foo.lua").assert("local   x    =   1");

    cwd.close().unwrap();
}

#[test]
fn test_formatting_still_formats_gitignore_files_if_requested() {
    let cwd = construct_tree!({
        ".git/dummy.txt": "", // Need a .git folder for .gitignore lookup
        ".gitignore": "foo.lua",
        "foo.lua": "local   x    =   1",
    });

    let mut cmd = create_stylua();
    cmd.current_dir(cwd.path())
        .args(["--no-ignore-vcs", "."])
        .assert()
        .success();

    cwd.child("foo.lua").assert("local x = 1\n");

    cwd.close().unwrap();
}

#[test]
fn test_stdin_filepath_respects_cwd_configuration_next_to_file() {
    let cwd = construct_tree!({
        "stylua.toml": "quote_style = 'AutoPreferSingle'",
    });

    let mut cmd = create_stylua();
    cmd.current_dir(cwd.path())
        .args(["--stdin-filepath", "foo.lua", "-"])
        .write_stdin("local x = \"hello\"")
        .assert()
        .success()
        .stdout("local x = 'hello'\n");

    cwd.close().unwrap();
}

#[test]
fn test_stdin_filepath_respects_cwd_configuration_for_nested_file() {
    let cwd = construct_tree!({
        "stylua.toml": "quote_style = 'AutoPreferSingle'",
    });

    let mut cmd = create_stylua();
    cmd.current_dir(cwd.path())
        .args(["--stdin-filepath", "build/foo.lua", "-"])
        .write_stdin("local x = \"hello\"")
        .assert()
        .success()
        .stdout("local x = 'hello'\n");

    cwd.close().unwrap();
}

#[test]
fn test_cwd_configuration_respected_when_formatting_from_stdin() {
    let cwd = construct_tree!({
        "stylua.toml": "quote_style = 'AutoPreferSingle'",
        "foo.lua": "local x = \"hello\"",
    });

    let mut cmd = create_stylua();
    cmd.current_dir(cwd.path())
        .arg("-")
        .write_stdin("local x = \"hello\"")
        .assert()
        .success()
        .stdout("local x = 'hello'\n");

    cwd.close().unwrap();
}

#[test]
fn test_cwd_configuration_respected_for_file_in_cwd() {
    let cwd = construct_tree!({
        "stylua.toml": "quote_style = 'AutoPreferSingle'",
        "foo.lua": "local x = \"hello\"",
    });

    let mut cmd = create_stylua();
    cmd.current_dir(cwd.path())
        .arg("foo.lua")
        .assert()
        .success();

    cwd.child("foo.lua").assert("local x = 'hello'\n");

    cwd.close().unwrap();
}

#[test]
fn test_cwd_configuration_respected_for_nested_file() {
    let cwd = construct_tree!({
        "stylua.toml": "quote_style = 'AutoPreferSingle'",
        "build/foo.lua": "local x = \"hello\"",
    });

    let mut cmd = create_stylua();
    cmd.current_dir(cwd.path())
        .arg("build/foo.lua")
        .assert()
        .success();

    cwd.child("build/foo.lua").assert("local x = 'hello'\n");

    cwd.close().unwrap();
}

#[test]
fn test_configuration_is_not_used_outside_of_cwd() {
    let cwd = construct_tree!({
        "stylua.toml": "quote_style = 'AutoPreferSingle'",
        "build/foo.lua": "local x = \"hello\"",
    });

    let mut cmd = create_stylua();
    cmd.current_dir(cwd.child("build").path())
        .arg("foo.lua")
        .assert()
        .success();

    cwd.child("build/foo.lua").assert("local x = \"hello\"\n");

    cwd.close().unwrap();
}

#[test]
fn test_configuration_used_outside_of_cwd_when_search_parent_directories_is_enabled() {
    let cwd = construct_tree!({
        "stylua.toml": "quote_style = 'AutoPreferSingle'",
        "build/foo.lua": "local x = \"hello\"",
    });

    let mut cmd = create_stylua();
    cmd.current_dir(cwd.child("build").path())
        .args(["--search-parent-directories", "foo.lua"])
        .assert()
        .success();

    cwd.child("build/foo.lua").assert("local x = 'hello'\n");

    cwd.close().unwrap();
}

#[test]
fn test_configuration_is_searched_next_to_file() {
    let cwd = construct_tree!({
        "build/stylua.toml": "quote_style = 'AutoPreferSingle'",
        "build/foo.lua": "local x = \"hello\"",
    });

    let mut cmd = create_stylua();
    cmd.current_dir(cwd.path())
        .arg("build/foo.lua")
        .assert()
        .success();

    cwd.child("build/foo.lua").assert("local x = 'hello'\n");

    cwd.close().unwrap();
}

#[test]
fn test_configuration_is_used_closest_to_the_file() {
    let cwd = construct_tree!({
        "stylua.toml": "quote_style = 'AutoPreferDouble'",
        "build/stylua.toml": "quote_style = 'AutoPreferSingle'",
        "build/foo.lua": "local x = \"hello\"",
    });

    let mut cmd = create_stylua();
    cmd.current_dir(cwd.path())
        .arg("build/foo.lua")
        .assert()
        .success();

    cwd.child("build/foo.lua").assert("local x = 'hello'\n");

    cwd.close().unwrap();
}

#[test]
fn test_respect_config_path_override() {
    let cwd = construct_tree!({
        "stylua.toml": "quote_style = 'AutoPreferDouble'",
        "build/stylua.toml": "quote_style = 'AutoPreferSingle'",
        "foo.lua": "local x = \"hello\"",
    });

    let mut cmd = create_stylua();
    cmd.current_dir(cwd.path())
        .args(["--config-path", "build/stylua.toml", "foo.lua"])
        .assert()
        .success();
}

#[test]
fn test_respect_config_path_override_for_stdin_filepath() {
    let cwd = construct_tree!({
        "stylua.toml": "quote_style = 'AutoPreferDouble'",
        "build/stylua.toml": "quote_style = 'AutoPreferSingle'",
        "foo.lua": "local x = \"hello\"",
    });

    let mut cmd = create_stylua();
    cmd.current_dir(cwd.path())
        .args(["--config-path", "build/stylua.toml", "-"])
        .write_stdin("local x = \"hello\"")
        .assert()
        .success()
        .stdout("local x = 'hello'\n");

    cwd.close().unwrap();
}

#[test]
fn test_uses_cli_overrides_instead_of_default_configuration() {
    let cwd = construct_tree!({
        "foo.lua": "local x = \"hello\"",
    });

    let mut cmd = create_stylua();
    cmd.current_dir(cwd.path())
        .args(["--quote-style", "AutoPreferSingle", "."])
        .assert()
        .success();

    cwd.child("foo.lua").assert("local x = 'hello'\n");

    cwd.close().unwrap();
}

#[test]
fn test_uses_cli_overrides_instead_of_default_configuration_stdin_filepath() {
    let cwd = construct_tree!({
        "foo.lua": "local x = \"hello\"",
    });

    let mut cmd = create_stylua();
    cmd.current_dir(cwd.path())
        .args(["--quote-style", "AutoPreferSingle", "-"])
        .write_stdin("local x = \"hello\"")
        .assert()
        .success()
        .stdout("local x = 'hello'\n");

    cwd.close().unwrap();
}

#[test]
fn test_uses_cli_overrides_instead_of_found_configuration() {
    let cwd = construct_tree!({
        "stylua.toml": "quote_style = 'AutoPreferDouble'",
        "foo.lua": "local x = \"hello\"",
    });

    let mut cmd = create_stylua();
    cmd.current_dir(cwd.path())
        .args(["--quote-style", "AutoPreferSingle", "."])
        .assert()
        .success();

    cwd.child("foo.lua").assert("local x = 'hello'\n");

    cwd.close().unwrap();
}

#[test]
fn test_uses_cli_overrides_instead_of_found_configuration_stdin_filepath() {
    let cwd = construct_tree!({
        "stylua.toml": "quote_style = 'AutoPreferDouble'",
        "foo.lua": "local x = \"hello\"",
    });

    let mut cmd = create_stylua();
    cmd.current_dir(cwd.path())
        .args(["--quote-style", "AutoPreferSingle", "-"])
        .write_stdin("local x = \"hello\"")
        .assert()
        .success()
        .stdout("local x = 'hello'\n");

    cwd.close().unwrap();
}

#[test]
#[cfg(feature = "editorconfig")]
fn test_editorconfig_child_without_root_merges_with_parent() {
    let cwd = construct_tree!({
        ".editorconfig": "root = true\n\n[*.lua]\nindent_style = space\nindent_size = 2\n",
        "child/.editorconfig": "[*.lua]\nquote_type = single\n",
        "child/foo.lua": "local foo = {\n\ta = 1,\n}\n\nlocal bar = \"\"\n",
    });

    let mut cmd = create_stylua();
    cmd.current_dir(cwd.path())
        .arg("child/foo.lua")
        .assert()
        .success();

    // indent from parent, quotes from child
    cwd.child("child/foo.lua")
        .assert("local foo = {\n  a = 1,\n}\n\nlocal bar = ''\n");

    cwd.close().unwrap();
}

#[test]
#[cfg(feature = "editorconfig")]
fn test_editorconfig_root_true_stops_upward_search() {
    let cwd = construct_tree!({
        ".editorconfig": "root = true\n\n[*.lua]\nindent_style = space\nindent_size = 2\nquote_type = single\n",
        "child/.editorconfig": "root = true\n\n[*.lua]\nindent_style = space\nindent_size = 4\n",
        "child/foo.lua": "local foo = {\n\ta = 1,\n}\n\nlocal bar = \"\"\n",
    });

    let mut cmd = create_stylua();
    cmd.current_dir(cwd.path())
        .arg("child/foo.lua")
        .assert()
        .success();

    // indent_size=4 from child; parent's quote_type=single must NOT be inherited
    cwd.child("child/foo.lua")
        .assert("local foo = {\n    a = 1,\n}\n\nlocal bar = \"\"\n");

    cwd.close().unwrap();
}

#[test]
#[cfg(feature = "editorconfig")]
fn test_editorconfig_closer_config_takes_precedence() {
    let cwd = construct_tree!({
        ".editorconfig": "root = true\n\n[*.lua]\nindent_style = space\nindent_size = 2\nquote_type = single\n",
        "child/.editorconfig": "[*.lua]\nindent_size = 4\n",
        "child/foo.lua": "local foo = {\n\ta = 1,\n}\n\nlocal bar = \"\"\n",
    });

    let mut cmd = create_stylua();
    cmd.current_dir(cwd.path())
        .arg("child/foo.lua")
        .assert()
        .success();

    // indent_size=4 from child overrides parent's 2; quote_type=single inherited from parent
    cwd.child("child/foo.lua")
        .assert("local foo = {\n    a = 1,\n}\n\nlocal bar = ''\n");

    cwd.close().unwrap();
}

#[test]
#[cfg(feature = "editorconfig")]
fn test_editorconfig_merges_across_three_directory_levels() {
    let cwd = construct_tree!({
        ".editorconfig": "root = true\n\n[*.lua]\nindent_style = space\nindent_size = 2\n",
        "middle/.editorconfig": "[*.lua]\nquote_type = single\n",
        "middle/inner/.editorconfig": "[*.lua]\nmax_line_length = 80\n",
        "middle/inner/foo.lua": "local foo = {\n\ta = 1,\n}\n\nlocal bar = \"\"\n",
    });

    let mut cmd = create_stylua();
    cmd.current_dir(cwd.path())
        .arg("middle/inner/foo.lua")
        .assert()
        .success();

    // indent from root, quotes from middle, max_line_length from inner
    cwd.child("middle/inner/foo.lua")
        .assert("local foo = {\n  a = 1,\n}\n\nlocal bar = ''\n");

    cwd.close().unwrap();
}
