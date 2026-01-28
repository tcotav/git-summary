use super::Formatter;
use crate::git::GitSummaryData;

pub struct MarkdownFormatter;

/// Format ISO 8601 timestamp to a shorter readable format
fn format_timestamp(iso: &str) -> String {
    // Input: 2025-01-27T10:30:45-05:00
    // Output: 2025-01-27 10:30
    if let Some(t_pos) = iso.find('T') {
        let date = &iso[..t_pos];
        let time = &iso[t_pos + 1..];
        let short_time = if time.len() >= 5 { &time[..5] } else { time };
        format!("{} {}", date, short_time)
    } else {
        iso.to_string()
    }
}

impl Formatter for MarkdownFormatter {
    fn format(&self, data: &GitSummaryData, summary: &str, verbose: bool, quiet: bool) -> String {
        let mut output = String::new();

        if quiet {
            output.push_str(summary);
            output.push('\n');
            return output;
        }

        // Header
        output.push_str(&format!(
            "# Git Summary: {} ({})\n\n",
            data.date_range, data.branch
        ));

        output.push_str(&format!(
            "**{} commits** | **+{} -{}** lines\n\n",
            data.commits.len(),
            data.total_additions,
            data.total_deletions
        ));

        // Summary section
        output.push_str("## Summary\n\n");
        output.push_str(summary);
        output.push_str("\n\n");

        // By Area section
        if !data.area_stats.is_empty() {
            output.push_str("## By Area\n\n");
            output.push_str("| Path | Commits | Lines |\n");
            output.push_str("|------|---------|-------|\n");
            for area in &data.area_stats {
                output.push_str(&format!(
                    "| {} | {} | +{}/-{} |\n",
                    area.path, area.commit_count, area.additions, area.deletions
                ));
            }
            output.push('\n');
        }

        // Commits section
        output.push_str("## Commits\n\n");
        for commit in &data.commits {
            let date = format_timestamp(&commit.timestamp);
            if verbose {
                output.push_str(&format!(
                    "- `{}` `{}` {}\n",
                    date, commit.short_hash, commit.message
                ));
                for file in &commit.files_changed {
                    output.push_str(&format!(
                        "  - `{}` (+{}/-{})\n",
                        file.path, file.additions, file.deletions
                    ));
                }
            } else {
                output.push_str(&format!("- `{}` {}\n", date, commit.message));
            }
        }

        output
    }
}
