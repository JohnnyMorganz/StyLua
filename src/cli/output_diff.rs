// Based off https://github.com/mitsuhiko/similar/blob/main/examples/terminal-inline.rs
// Licensed under https://github.com/mitsuhiko/similar/blob/main/LICENSE
use crate::opt;
use anyhow::Result;
use console::{style, Style};
use serde::Serialize;
use similar::{ChangeTag, DiffOp, TextDiff};
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
    title: &str,
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

pub fn output_diff_unified(old: &str, new: &str) -> Result<Option<Vec<u8>>> {
    let text_diff = TextDiff::from_lines(old, new);

    // If there are no changes, return nothing
    if text_diff.ratio() == 1.0 {
        return Ok(None);
    }

    let mut buffer = Vec::new();
    write!(
        &mut buffer,
        "{}",
        text_diff.unified_diff().header("old", "new")
    )?;
    Ok(Some(buffer))
}

#[derive(Serialize)]
pub struct DiffMismatch {
    original_start_line: usize,
    original_end_line: usize,
    expected_start_line: usize,
    expected_end_line: usize,
    original: String,
    expected: String,
}

pub fn output_diff_json(old: &str, new: &str) -> Option<Vec<DiffMismatch>> {
    let text_diff = TextDiff::from_lines(old, new);
    let grouped_ops = text_diff.grouped_ops(0);

    if grouped_ops.is_empty() {
        return None;
    }

    let mut mismatches = Vec::with_capacity(grouped_ops.len());

    for group in grouped_ops {
        for op in group {
            match op {
                DiffOp::Replace {
                    old_index,
                    old_len,
                    new_index,
                    new_len,
                } => {
                    let original = text_diff
                        .iter_changes(&op)
                        .filter(|change| matches!(change.tag(), ChangeTag::Delete))
                        .map(|change| change.value())
                        .collect();

                    let expected = text_diff
                        .iter_changes(&op)
                        .filter(|change| matches!(change.tag(), ChangeTag::Insert))
                        .map(|change| change.value())
                        .collect();

                    mismatches.push(DiffMismatch {
                        original_start_line: old_index,
                        original_end_line: old_index + old_len - 1,
                        expected_start_line: new_index,
                        expected_end_line: new_index + new_len - 1,
                        original,
                        expected,
                    });
                }
                DiffOp::Delete {
                    old_index,
                    old_len,
                    new_index,
                } => {
                    let actual = text_diff
                        .iter_changes(&op)
                        .next()
                        .expect("no actual change present in diff/delete");

                    mismatches.push(DiffMismatch {
                        original_start_line: old_index,
                        original_end_line: old_index + old_len - 1,
                        expected_start_line: new_index,
                        expected_end_line: new_index,
                        original: actual.to_string(),
                        expected: "".to_string(),
                    })
                }
                DiffOp::Insert {
                    old_index,
                    new_index,
                    new_len,
                } => {
                    let expected = text_diff
                        .iter_changes(&op)
                        .next()
                        .expect("no actual change present in diff/insert");

                    mismatches.push(DiffMismatch {
                        original_start_line: old_index,
                        original_end_line: old_index,
                        expected_start_line: new_index,
                        expected_end_line: new_index + new_len - 1,
                        original: "".to_string(),
                        expected: expected.to_string(),
                    })
                }
                DiffOp::Equal { .. } => (), // Don't record an equals diff, its unnecessary
            }
        }
    }

    Some(mismatches)
}

#[cfg(test)]
mod tests {
    use crate::opt::Color;

    use super::*;

    #[test]
    fn test_no_diff() {
        let output = output_diff("local x = 1", "local x = 1", 0, "", Color::Auto).unwrap();
        assert!(output.is_none())
    }

    #[test]
    fn test_deletion_diff() {
        let output = output_diff("local x = 1", "", 0, "", Color::Never)
            .unwrap()
            .expect("expected change, found no diff");
        println!("{}", String::from_utf8(output.to_owned()).unwrap());
        assert_eq!(
            String::from_utf8(output).unwrap(),
            r#"
1        |-local x = 1
"#
        );
    }

    #[test]
    fn test_addition_diff() {
        let output = output_diff("", "local x = 1", 0, "", Color::Never)
            .unwrap()
            .expect("expected change, found no diff");
        assert_eq!(
            String::from_utf8(output).unwrap(),
            r#"
    1    |+local x = 1
"#
        );
    }

    #[test]
    fn test_change_diff() {
        let output = output_diff("local  x = 1", "local x = 1", 0, "", Color::Never)
            .unwrap()
            .expect("expected change, found no diff");
        assert_eq!(
            String::from_utf8(output).unwrap(),
            r#"
1        |-local  x = 1
    1    |+local x = 1
"#
        );
    }
}
