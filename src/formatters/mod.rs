mod json;
mod markdown;
mod pretty;

pub use json::JsonFormatter;
pub use markdown::MarkdownFormatter;
pub use pretty::PrettyFormatter;

use crate::git::GitSummaryData;

pub trait Formatter {
    fn format(&self, data: &GitSummaryData, summary: &str, verbose: bool, quiet: bool) -> String;
}
