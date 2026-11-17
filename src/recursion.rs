//! Stack space for recursive parsing and formatting.
//!
//! `full_moon`'s `parse_stmt` keeps one large frame live while it parses a
//! nested block (about 23KiB per nested function on an optimized build with
//! every syntax feature). The CLI calls [`crate::format_code`] on a
//! `threadpool` worker whose stack is Rust's default 2MiB, so about 100 nested
//! functions abort the process. Debug builds of the formatter use several
//! times that much per level.
//!
//! [`with_sufficient_stack`] switches to one fixed segment when the caller has
//! less than that left. Release parsing of the reported 100–200 nested
//! functions needs only a few megabytes. The segment is 16MiB so a debug
//! build of the formatter — several times the release frame size — can also
//! run the 160-level regression without aborting. It does not grow with the
//! input, so a few thousand nested blocks can still overflow. Shallow calls
//! on a thread that already has this much free stay put.

/// Stack reserved for one `format_code` call.
const REQUIRED_STACK: usize = 16 * 1024 * 1024;

/// Run `f` with at least [`REQUIRED_STACK`] bytes of stack available.
pub(crate) fn with_sufficient_stack<R>(f: impl FnOnce() -> R) -> R {
    #[cfg(not(target_arch = "wasm32"))]
    {
        let enough =
            matches!(stacker::remaining_stack(), Some(remaining) if remaining >= REQUIRED_STACK);
        if enough {
            f()
        } else {
            stacker::grow(REQUIRED_STACK, f)
        }
    }
    #[cfg(target_arch = "wasm32")]
    {
        f()
    }
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    #[test]
    fn worker_thread_has_less_than_the_reserved_stack() {
        // The CLI pool uses this size. If the probe over-reports it, the
        // guard skips the switch and the overflow comes back.
        let remaining = std::thread::Builder::new()
            .stack_size(2 * 1024 * 1024)
            .spawn(stacker::remaining_stack)
            .unwrap()
            .join()
            .unwrap();
        let remaining = remaining.expect("stack bounds should be visible");
        assert!(
            remaining < super::REQUIRED_STACK,
            "2MiB thread reported {} bytes free",
            remaining
        );
    }
}
