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
