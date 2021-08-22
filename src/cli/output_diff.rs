// Based off https://github.com/mitsuhiko/similar/blob/main/examples/terminal-inline.rs
// Licensed under https://github.com/mitsuhiko/similar/blob/main/LICENSE
use crate::opt;
use anyhow::Result;
use console::{style, Style, Term};
use similar::{ChangeTag, TextDiff};
use std::fmt;
use std::io::Write;

struct Line(Option<usize>);

impl fmt::Display for Line {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.0 {
            None => write!(f, "    "),
            Some(idx) => write!(f, "{:<4}", idx + 1),
        }
    }
}

pub fn output_diff(
    old: &str,
    new: &str,
    context_size: usize,
    title: String,
    color: opt::Color,
) -> Result<Option<Vec<u8>>> {
    let diff = TextDiff::from_lines(old, new);
    let diff_opts = diff.grouped_ops(context_size);
    let mut iter = diff_opts.iter().enumerate().peekable();

    if iter.peek().is_none() {
        // There are no changes
        return Ok(None);
    }

    let mut buffer = Vec::new();

    let should_use_color = color.should_use_color();

    // Print out the header title
    writeln!(&mut buffer, "{}", title)?;

    for (idx, group) in iter {
        if idx > 0 {
            writeln!(&mut buffer, "{:-^1$}", "-", 80)?;
        }
        for op in group {
            for change in diff.iter_inline_changes(op) {
                let (sign, s) = match change.tag() {
                    ChangeTag::Delete => ("-", Style::new().red()),
                    ChangeTag::Insert => ("+", Style::new().green()),
                    ChangeTag::Equal => (" ", Style::new().dim()),
                };
                write!(
                    &mut buffer,
                    "{}{} |{}",
                    style(Line(change.old_index()))
                        .dim()
                        .force_styling(should_use_color),
                    style(Line(change.new_index()))
                        .dim()
                        .force_styling(should_use_color),
                    s.apply_to(sign).bold().force_styling(should_use_color),
                )?;
                for (emphasized, value) in change.iter_strings_lossy() {
                    if emphasized {
                        write!(
                            &mut buffer,
                            "{}",
                            s.apply_to(value)
                                .underlined()
                                .on_black()
                                .force_styling(should_use_color)
                        )?;
                    } else {
                        write!(
                            &mut buffer,
                            "{}",
                            s.apply_to(value).force_styling(should_use_color)
                        )?;
                    }
                }
                if change.missing_newline() {
                    writeln!(&mut buffer)?;
                }
            }
        }
    }

    Ok(Some(buffer))
}
