//! Stack space for recursive parsing and formatting.
//!
//! `full_moon`'s `parse_stmt` is one function covering every statement kind, so
//! its frame stays live while a nested block is parsed. On an optimized build
//! that frame is about 13KiB, and a `function` body adds the declaration,
//! function-body, and block frames on top (about 23KiB per nested function).
//! The formatter then walks the same tree: `format_block` → `format_stmt` →
//! `format_function_*` → `format_block`, about 17KiB per level when optimized
//! and several times that in debug.
//!
//! The CLI runs each file on a `threadpool` worker. Those threads use Rust's
//! default 2MiB stack, so roughly 100 nested functions abort the process
//! (`thread '<unknown>' has overflowed its stack`, exit 134) instead of
//! formatting valid code or returning a parse error. Growing the stack here,
//! from the input's block depth, fixes that without asking the caller to set
//! `RUST_MIN_STACK`.

/// Optimized `parse_stmt` plus the function-body chain is ~23KiB per nested
/// block. 48KiB leaves room for the frames measured on the all-features build.
#[cfg(not(target_arch = "wasm32"))]
const PARSER_STACK_PER_LEVEL: usize = 48 * 1024;
/// Outer parse/format frames (`parse_fallible`, the formatter entry, printing).
#[cfg(not(target_arch = "wasm32"))]
const PARSER_STACK_BASE: usize = 1024 * 1024;
/// Stop a depth-scan glitch from requesting an unbounded mapping.
#[cfg(not(target_arch = "wasm32"))]
const PARSER_STACK_MAX: usize = 64 * 1024 * 1024;

/// Bytes that must still be free before entering another formatted block.
/// The statement/function frames between two `format_block` calls are ~20KiB
/// optimized and ~80KiB in debug; this stays above both.
#[cfg(not(target_arch = "wasm32"))]
const FORMAT_STACK_RED_ZONE: usize = 256 * 1024;
/// Fresh stack segment used when the red zone is exhausted. Larger than the
/// red zone so the check does not fire again on the new segment.
#[cfg(not(target_arch = "wasm32"))]
const FORMAT_STACK_SEGMENT: usize = 2 * 1024 * 1024;

/// Run `f` with enough stack for `full_moon` to recurse through `code`.
///
/// `stacker::grow` always allocates a fresh segment, so this only switches
/// when the current thread is shorter than the budget. Shallow files on a
/// normal thread stay put. A reparse that is already inside this guard sees
/// the grown segment and does not allocate another one.
pub(crate) fn with_parser_stack<R>(code: &str, f: impl FnOnce() -> R) -> R {
    #[cfg(not(target_arch = "wasm32"))]
    {
        let needed = parser_stack_budget(estimate_block_depth(code));
        let enough = matches!(stacker::remaining_stack(), Some(remaining) if remaining >= needed);
        if enough {
            f()
        } else {
            stacker::grow(needed, f)
        }
    }
    #[cfg(target_arch = "wasm32")]
    {
        let _ = code;
        f()
    }
}

/// Ensure the next nested formatted block has room for its own frames.
pub(crate) fn with_block_stack<R>(f: impl FnOnce() -> R) -> R {
    #[cfg(not(target_arch = "wasm32"))]
    {
        stacker::maybe_grow(FORMAT_STACK_RED_ZONE, FORMAT_STACK_SEGMENT, f)
    }
    #[cfg(target_arch = "wasm32")]
    {
        f()
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn parser_stack_budget(depth: usize) -> usize {
    let scaled = depth.saturating_mul(PARSER_STACK_PER_LEVEL);
    PARSER_STACK_BASE
        .saturating_add(scaled)
        .min(PARSER_STACK_MAX)
}

/// Maximum block nesting `full_moon` will recurse through.
///
/// Counts `function` / `do` / `if` / `while` / `for` / `repeat` outside of
/// strings and comments, and matches them against `end` / `until`. This
/// over-counts some forms (`for`/`while` also contain a `do`) which only
/// reserves extra stack. It must not under-count real nesting: an unclosed
/// `function` still contributes, because that is the invalid input that used
/// to abort.
#[cfg(not(target_arch = "wasm32"))]
fn estimate_block_depth(code: &str) -> usize {
    let bytes = code.as_bytes();
    let mut i = 0;
    let mut depth = 0usize;
    let mut max_depth = 0usize;

    while i < bytes.len() {
        let byte = bytes[i];
        if byte.is_ascii_whitespace() {
            i += 1;
            continue;
        }

        // `--` line comment, or `--[[` / `--[=[...` long comment.
        if byte == b'-' && bytes.get(i + 1) == Some(&b'-') {
            i += 2;
            if let Some(level) = long_bracket_level(bytes, i) {
                i = skip_long_bracket(bytes, i + level + 2, level);
            } else {
                while i < bytes.len() && bytes[i] != b'\n' {
                    i += 1;
                }
            }
            continue;
        }

        if byte == b'"' || byte == b'\'' || byte == b'`' {
            i = skip_short_string(bytes, i, byte);
            continue;
        }

        if byte == b'[' {
            if let Some(level) = long_bracket_level(bytes, i) {
                i = skip_long_bracket(bytes, i + level + 2, level);
                continue;
            }
        }

        if byte.is_ascii_alphabetic() || byte == b'_' {
            let start = i;
            i += 1;
            while i < bytes.len() && (bytes[i].is_ascii_alphanumeric() || bytes[i] == b'_') {
                i += 1;
            }
            match &code[start..i] {
                "function" | "do" | "if" | "while" | "for" | "repeat" => {
                    depth += 1;
                    max_depth = max_depth.max(depth);
                }
                "end" | "until" => {
                    depth = depth.saturating_sub(1);
                }
                _ => {}
            }
            continue;
        }

        i += 1;
    }

    max_depth
}

/// `[[`, `[=[`, `[==[`, ... at `i`, returning the number of `=` signs.
#[cfg(not(target_arch = "wasm32"))]
fn long_bracket_level(bytes: &[u8], i: usize) -> Option<usize> {
    if bytes.get(i) != Some(&b'[') {
        return None;
    }
    let mut j = i + 1;
    while bytes.get(j) == Some(&b'=') {
        j += 1;
    }
    if bytes.get(j) == Some(&b'[') {
        Some(j - (i + 1))
    } else {
        None
    }
}

/// `i` points just past the opening long bracket. Returns the index past the close.
#[cfg(not(target_arch = "wasm32"))]
fn skip_long_bracket(bytes: &[u8], mut i: usize, level: usize) -> usize {
    while i < bytes.len() {
        if bytes[i] == b']' {
            let mut j = i + 1;
            let mut eqs = 0;
            while eqs < level && bytes.get(j) == Some(&b'=') {
                j += 1;
                eqs += 1;
            }
            if eqs == level && bytes.get(j) == Some(&b']') {
                return j + 1;
            }
        }
        i += 1;
    }
    bytes.len()
}

/// `i` points at the opening quote. An unescaped newline ends the literal so a
/// broken string cannot swallow the rest of the file and hide real blocks.
#[cfg(not(target_arch = "wasm32"))]
fn skip_short_string(bytes: &[u8], mut i: usize, quote: u8) -> usize {
    i += 1;
    while i < bytes.len() {
        match bytes[i] {
            b'\\' => i += 2,
            b'\n' => return i,
            c if c == quote => return i + 1,
            _ => i += 1,
        }
    }
    bytes.len()
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use super::*;

    #[test]
    fn depth_counts_nested_functions_and_ignores_strings_and_comments() {
        assert_eq!(estimate_block_depth("local x = 1\n"), 0);
        assert_eq!(estimate_block_depth("function F()\nend\n"), 1);
        assert_eq!(
            estimate_block_depth("function F()\nfunction G()\nend\nend\n"),
            2
        );
        assert_eq!(estimate_block_depth("do do end end\n"), 2);
        assert_eq!(
            estimate_block_depth("-- function\nlocal s = \"function end\"\nfunction F() end\n"),
            1
        );
        assert_eq!(
            estimate_block_depth("--[=[\nfunction\n]=]\nfunction F() end\n"),
            1
        );
        // Unclosed functions keep their depth. `for`/`while` also contain `do`,
        // so those are counted twice; that only reserves extra stack.
        assert_eq!(
            estimate_block_depth("function F()\nfunction G()\nfunction H()\n"),
            3
        );
        assert_eq!(estimate_block_depth("for i = 1, 10 do\nend\n"), 2);
    }

    #[test]
    fn worker_thread_reports_a_small_remaining_stack() {
        // The CLI pool uses this stack size. If the probe over-reports it, the
        // parser guard would skip growing and the overflow would come back.
        let remaining = std::thread::Builder::new()
            .stack_size(2 * 1024 * 1024)
            .spawn(stacker::remaining_stack)
            .unwrap()
            .join()
            .unwrap();
        let remaining = remaining.expect("stack bounds should be visible");
        assert!(
            remaining < 3 * 1024 * 1024,
            "2MiB thread reported {} bytes free",
            remaining
        );
        assert!(
            remaining > 256 * 1024,
            "2MiB thread reported only {} bytes free",
            remaining
        );
    }

    #[test]
    fn parser_budget_scales_with_depth_and_is_capped() {
        assert_eq!(parser_stack_budget(0), PARSER_STACK_BASE);
        assert!(parser_stack_budget(100) > parser_stack_budget(10));
        assert_eq!(parser_stack_budget(usize::MAX), PARSER_STACK_MAX);
    }
}
