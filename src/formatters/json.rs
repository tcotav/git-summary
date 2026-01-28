use super::Formatter;
use crate::git::GitSummaryData;
use serde::Serialize;

pub struct JsonFormatter;

#[derive(Serialize)]
struct JsonOutput<'a> {
    branch: &'a str,
    date_range: &'a str,
    total_commits: usize,
    total_additions: i32,
    total_deletions: i32,
    summary: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    area_stats: Option<&'a [crate::git::AreaStats]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    commits: Option<&'a [crate::git::Commit]>,
}

#[derive(Serialize)]
struct QuietJsonOutput<'a> {
    summary: &'a str,
}

impl Formatter for JsonFormatter {
    fn format(&self, data: &GitSummaryData, summary: &str, verbose: bool, quiet: bool) -> String {
        if quiet {
            let output = QuietJsonOutput { summary };
            serde_json::to_string_pretty(&output).unwrap_or_default()
        } else {
            let output = JsonOutput {
                branch: &data.branch,
                date_range: &data.date_range,
                total_commits: data.commits.len(),
                total_additions: data.total_additions,
                total_deletions: data.total_deletions,
                summary,
                area_stats: Some(&data.area_stats),
                commits: if verbose {
                    Some(&data.commits)
                } else {
                    None
                },
            };
            serde_json::to_string_pretty(&output).unwrap_or_default()
        }
    }
}
