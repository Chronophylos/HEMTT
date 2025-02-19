#![deny(clippy::all, clippy::nursery)]
#![warn(clippy::pedantic)]
#![allow(clippy::cast_possible_truncation)]

use std::{io::BufRead, path::Path};

pub use thiserror;

use colored::Colorize;
use hemtt_tokens::{Position, Token};

#[derive(Debug)]
pub struct AppError {
    pub brief: String,
    pub details: Option<String>,
    pub help: Option<String>,
    pub source: Option<Box<Source>>,
    pub trace: Vec<Source>,
}

pub enum DisplayStyle {
    Info,
    Warning,
    Error,
}

impl AppError {
    #[must_use]
    pub fn short(&self) -> &str {
        &self.brief
    }

    #[must_use]
    pub fn long(&self, style: &DisplayStyle) -> String {
        format!(
            "{}{}\n\n{}{}{}",
            match style {
                DisplayStyle::Info => format!("{}: ", "info".bright_blue()).bold(),
                DisplayStyle::Warning => format!("{}: ", "warning".bright_yellow()).bold(),
                DisplayStyle::Error => format!("{}: ", "error".bright_red()).bold(),
            },
            self.brief.bold(),
            self.details.clone().unwrap_or_default(),
            self.source().unwrap_or_default(),
            {
                let help = self.help.clone().unwrap_or_default();
                if help.is_empty() {
                    String::new()
                } else {
                    format!("{}: {help}\n", "help".bright_yellow())
                }
            },
        )
    }

    #[must_use]
    pub fn source(&self) -> Option<String> {
        let source = self.source.as_ref()?;
        let trace = {
            let mut trace = String::new();
            for source in &self.trace {
                if let Some(line) = source.lines.first() {
                    trace.push_str(&format!(
                        "    {}  {}:{}:{}\n{: >3} {} {}\n",
                        "↓".bright_blue(),
                        source
                            .position
                            .path()
                            .replace('\\', "/")
                            .trim_start_matches('/')
                            .bold(),
                        source.position.start().1 .0,
                        source.position.start().1 .1,
                        (source.position.start().1 .0).to_string().bright_blue(),
                        "|".bright_blue(),
                        line
                    ));
                }
            }
            trace
        };
        Some(format!(
            "{}   {} {}:{}:{}\n{}\n",
            trace,
            "-->".blue(),
            source
                .position
                .path()
                .replace('\\', "/")
                .trim_start_matches('/')
                .bold(),
            source.position.start().1 .0,
            source.position.start().1 .1,
            {
                let bar = "    |".blue();
                let mut lines = String::new();
                for (i, line) in source.lines.iter().enumerate() {
                    let linenum = source.position.start().1 .0 + i;
                    lines.push_str(&format!(
                        "{: >3} {} {}\n",
                        linenum.to_string().blue(),
                        "|".blue(),
                        line
                    ));
                }
                lines.push_str(&format!(
                    "{} {:>offset$} {}",
                    bar,
                    "^".red(),
                    source.note.red(),
                    offset = source.position.start().1 .1
                ));
                format!("{bar}\n{lines}")
            }
        ))
    }
}

impl<E> From<E> for AppError
where
    E: PrettyError,
{
    fn from(e: E) -> Self {
        Self {
            brief: e.brief(),
            details: e.details(),
            help: e.help(),
            source: e.source(),
            trace: e.trace(),
        }
    }
}

pub trait PrettyError: ToString {
    fn brief(&self) -> String {
        self.to_string()
    }
    fn details(&self) -> Option<String> {
        None
    }
    fn help(&self) -> Option<String> {
        None
    }
    fn source(&self) -> Option<Box<Source>> {
        None
    }
    fn trace(&self) -> Vec<Source> {
        Vec::new()
    }
}

#[derive(Debug)]
pub struct Source {
    pub lines: Vec<String>,
    pub position: Position,
    pub note: String,
}

/// Read specific lines from a file
///
/// # Errors
/// if the file cannot be read
///
/// # Panics
/// if the lines are out of bounds
///
pub fn read_lines_from_file(
    path: &Path,
    start: usize,
    end: usize,
) -> Result<Vec<String>, std::io::Error> {
    let file = std::fs::File::open(path)?;
    let reader = std::io::BufReader::new(file);
    let mut lines = reader.lines();
    for _ in 1..start {
        lines.next().unwrap().unwrap();
    }
    let mut ret = Vec::new();
    for _ in 0..=(end - start) {
        if let Some(x) = lines.next() {
            ret.push(x.unwrap().trim_end().to_string());
        }
    }
    Ok(ret)
}

/// Create a source object from a token
///
/// # Errors
/// if the file cannot be read
pub fn make_source(token: &Token, note: String) -> Result<Source, std::io::Error> {
    Ok(Source {
        lines: read_lines_from_file(
            Path::new(
                token
                    .source()
                    .path()
                    .replace('\\', "/")
                    .trim_start_matches('/'),
            ),
            token.source().start().1 .0,
            token.source().end().1 .0,
        )?,
        position: token.source().clone(),
        note,
    })
}
