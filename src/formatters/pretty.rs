use colored::Colorize;

use super::Formatter;
use crate::git::GitSummaryData;

pub struct PrettyFormatter;

/// Format ISO 8601 timestamp to a shorter readable format
fn format_timestamp(iso: &str) -> String {
    // Input: 2025-01-27T10:30:45-05:00
    // Output: 2025-01-27 10:30
    if let Some(t_pos) = iso.find('T') {
        let date = &iso[..t_pos];
        let time = &iso[t_pos + 1..];
        // Take just HH:MM from time
        let short_time = if time.len() >= 5 { &time[..5] } else { time };
        format!("{} {}", date, short_time)
    } else {
        iso.to_string()
    }
}

impl Formatter for PrettyFormatter {
    fn format(&self, data: &GitSummaryData, summary: &str, verbose: bool, quiet: bool) -> String {
        let mut output = String::new();

        if quiet {
            // Just the summary
            output.push_str(summary);
            output.push('\n');
            return output;
        }

        // Header
        let header_line = "═".repeat(60);
        output.push_str(&header_line.cyan().to_string());
        output.push('\n');

        let title = format!(
            "  Git Summary: {} ({})",
            data.date_range, data.branch
        );
        output.push_str(&title.cyan().bold().to_string());
        output.push('\n');

        let stats = format!(
            "  {} commits | +{} -{} lines",
            data.commits.len(),
            data.total_additions,
            data.total_deletions
        );
        output.push_str(&stats.cyan().to_string());
        output.push('\n');

        output.push_str(&header_line.cyan().to_string());
        output.push_str("\n\n");

        // Summary section
        output.push_str(&"## Summary\n".yellow().bold().to_string());
        output.push_str(summary);
        output.push_str("\n\n");

        // By Area section
        if !data.area_stats.is_empty() {
            output.push_str(&"## By Area\n".yellow().bold().to_string());
            for area in &data.area_stats {
                let line = format!(
                    "  {:20} {:3} commits, {:>+5}/-{:<5} lines\n",
                    area.path, area.commit_count, area.additions, area.deletions
                );
                output.push_str(&line);
            }
            output.push('\n');
        }

        // Commits section
        output.push_str(&"## Commits\n".yellow().bold().to_string());
        for commit in &data.commits {
            let date = format_timestamp(&commit.timestamp);
            if verbose {
                output.push_str(&format!(
                    "  {} {} {}\n",
                    date.dimmed(),
                    commit.short_hash.dimmed(),
                    commit.message
                ));
                for file in &commit.files_changed {
                    output.push_str(&format!(
                        "              {:>+4}/-{:<4} {}\n",
                        file.additions,
                        file.deletions,
                        file.path.dimmed()
                    ));
                }
            } else {
                output.push_str(&format!(
                    "  {} {}\n",
                    date.dimmed(),
                    commit.message
                ));
            }
        }

        output
    }
}
