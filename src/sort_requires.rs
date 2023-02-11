//! Sort Requires CodeMod
//! This is an optional extension which will firstly sort all requires within a file before formatting the file
//!
//! The following assumptions are made when using this codemod:
//! - All requires are pure and have no side effects: resorting the requires is not an issue
//! - Only requires at the top level block are to be sorted
//! - Requires are of the form `local NAME = require(REQUIRE)`, with only a single require per local assignment
//!
//! Requires sorting works in the following way:
//! - We group consecutive requires into a "block".
//!   If we hit a line which is a non-require or empty, we close the old block and start a new one.
//! - Requires are sorted only within their block.
//!   This allows us to solve the assumption of depending on local variables
//!   (if there is a local variable in between requires, it would split them into two separate blocks,
//!   so a require will always be after any local variable it uses)
//! - Blocks remain in-place in the file.
use serde::Deserialize;

#[derive(Copy, Clone, Debug, Default, Deserialize)]
pub struct SortRequiresConfig {
    /// Whether the sort requires codemod is enabled
    enabled: bool,
}

impl SortRequiresConfig {
    pub fn enabled(&self) -> bool {
        self.enabled
    }
    pub fn set_enabled(&self, enabled: bool) -> Self {
        Self { enabled }
    }
}
