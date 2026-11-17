//! Regression tests for deeply nested blocks.
//!
//! About 100 nested `function` bodies used to abort the process (exit 134) on
//! the CLI's 2MiB worker threads. These tests call the library on a thread
//! with that same stack size, so a passing run does not depend on the main
//! thread's larger stack or on `RUST_MIN_STACK`.

use stylua_lib::{format_code, Config, Error, LuaVersion, OutputVerification};

const WORKER_STACK: usize = 2 * 1024 * 1024;

fn on_worker_stack<T: Send + 'static>(f: impl FnOnce() -> T + Send + 'static) -> T {
    std::thread::Builder::new()
        .name("stylua-worker".to_string())
        .stack_size(WORKER_STACK)
        .spawn(f)
        .expect("failed to spawn worker thread")
        .join()
        .expect("worker thread panicked")
}

fn nested_functions(depth: usize) -> String {
    let mut code = String::new();
    for i in 1..=depth {
        code.push_str(&format!("function F{i}()\n"));
    }
    for _ in 0..depth {
        code.push_str("end\n");
    }
    code
}

fn unterminated_functions(depth: usize) -> String {
    let mut code = String::new();
    for i in 1..=depth {
        code.push_str(&format!("function F{i}()\n"));
    }
    code
}

fn assert_formats_idempotently(source: String) -> String {
    let formatted = on_worker_stack(move || {
        format_code(&source, Config::default(), None, OutputVerification::None)
            .expect("valid nested code should format")
    });
    let formatted_again = formatted.clone();
    let reformatted = on_worker_stack(move || {
        format_code(
            &formatted_again,
            Config::default(),
            None,
            OutputVerification::Full,
        )
        .expect("formatted nested code should reformat")
    });
    assert_eq!(formatted, reformatted);
    assert!(formatted.ends_with("end\n"));
    formatted
}

#[test]
fn deeply_nested_functions_format_on_a_worker_stack() {
    for depth in [1, 20, 50, 100] {
        let formatted = assert_formats_idempotently(nested_functions(depth));
        assert!(formatted.contains(&format!("function F{depth}()")));
    }
}

#[test]
fn deeply_nested_do_blocks_format_on_a_worker_stack() {
    let mut code = String::new();
    for _ in 0..100 {
        code.push_str("do\n");
    }
    for _ in 0..100 {
        code.push_str("end\n");
    }
    assert_formats_idempotently(code);
}

#[test]
fn deeply_unterminated_functions_are_parse_errors() {
    for depth in [1, 20, 50, 100] {
        let source = unterminated_functions(depth);
        let result = on_worker_stack(move || {
            format_code(&source, Config::default(), None, OutputVerification::None)
        });
        assert!(
            matches!(&result, Err(Error::ParseError(_))),
            "depth {}: {:?}",
            depth,
            result
        );
        let message = result.unwrap_err().to_string();
        assert!(
            message.contains("error parsing"),
            "depth {}: {}",
            depth,
            message
        );
    }
}

/// One-line Luau interpolation whose expression is `depth` nested functions.
/// A backtick-skipping depth scan treats this as a string and does not grow
/// the stack, so 100 levels aborted on a 2MiB worker (exit 134).
fn luau_interpolated_nested_functions(depth: usize) -> String {
    let mut body = "nil".to_string();
    for _ in 0..depth {
        body = format!("function() local a = {body} end");
    }
    format!("local x = `{{{body}}}`\n")
}

/// 4×40 nested functions with a `\z` string full of the word `end` between
/// segments. Those `end`s are inside the string for full-moon, but a newline
/// scan counts them as real closers, underestimates depth, and previously
/// SIGSEGV'd (exit 139).
fn z_escape_underestimated_nesting() -> String {
    const SEGMENT: usize = 40;
    const SEGMENTS: usize = 4;
    let mut code = String::new();
    for segment in 0..SEGMENTS {
        for i in 0..SEGMENT {
            code.push_str(&format!("function S{segment}_{i}()\n"));
        }
        code.push_str("local s = \"\\z\n");
        for _ in 0..SEGMENT {
            code.push_str("end ");
        }
        code.push_str("\"\n");
    }
    for _ in 0..(SEGMENT * SEGMENTS) {
        code.push_str("end\n");
    }
    code
}

#[cfg(feature = "luau")]
#[test]
fn interpolated_nested_functions_format_on_a_worker_stack() {
    let source = luau_interpolated_nested_functions(100);
    let formatted = on_worker_stack(move || {
        let config = Config {
            syntax: LuaVersion::Luau,
            ..Config::default()
        };
        format_code(&source, config, None, OutputVerification::None)
            .expect("nested functions inside a Luau interpolated string should format")
    });
    assert!(formatted.contains("function()"));
    assert!(formatted.contains("local x"));
}

#[cfg(any(feature = "lua52", feature = "luau"))]
#[test]
fn z_escape_nested_functions_format_on_a_worker_stack() {
    let formatted = assert_formats_idempotently(z_escape_underestimated_nesting());
    assert!(formatted.contains("function S3_39()"));
    assert!(formatted.contains("\\z"));
}

#[cfg(feature = "luau")]
#[test]
fn deeply_unterminated_luau_method_signatures_are_parse_errors() {
    let mut code = String::new();
    for i in 1..=100 {
        code.push_str(&format!("function F{i}(self): number\n"));
    }
    let result = on_worker_stack(move || {
        let config = Config {
            syntax: LuaVersion::Luau,
            ..Config::default()
        };
        format_code(&code, config, None, OutputVerification::None)
    });
    assert!(
        matches!(result, Err(Error::ParseError(_))),
        "unterminated Luau functions should be a parse error, got {:?}",
        result
    );
}
