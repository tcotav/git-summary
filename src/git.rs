use anyhow::{Context, Result};
use serde::Serialize;
use std::process::Command;

#[derive(Debug, Clone, Serialize)]
pub struct Commit {
    pub hash: String,
    pub short_hash: String,
    pub message: String,
    pub timestamp: String,
    pub files_changed: Vec<FileChange>,
}

#[derive(Debug, Clone, Serialize)]
pub struct FileChange {
    pub path: String,
    pub additions: i32,
    pub deletions: i32,
}

#[derive(Debug, Clone, Serialize)]
pub struct AreaStats {
    pub path: String,
    pub commit_count: usize,
    pub additions: i32,
    pub deletions: i32,
}

#[derive(Debug, Serialize)]
pub struct GitSummaryData {
    pub branch: String,
    pub date_range: String,
    pub commits: Vec<Commit>,
    pub area_stats: Vec<AreaStats>,
    pub total_additions: i32,
    pub total_deletions: i32,
}

pub struct GitRepo {
    path: String,
}

impl GitRepo {
    pub fn new(path: &str) -> Self {
        Self {
            path: path.to_string(),
        }
    }

    fn run_git(&self, args: &[&str]) -> Result<String> {
        let output = Command::new("git")
            .args(["-C", &self.path])
            .args(args)
            .output()
            .context("Failed to execute git command")?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("Git command failed: {}", stderr);
        }

        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }

    pub fn get_current_branch(&self) -> Result<String> {
        let output = self.run_git(&["rev-parse", "--abbrev-ref", "HEAD"])?;
        Ok(output.trim().to_string())
    }

    pub fn get_commits(
        &self,
        branch: &str,
        since: Option<&str>,
        until: Option<&str>,
    ) -> Result<Vec<Commit>> {
        // Build git log command
        // Format: hash|short_hash|message|timestamp
        let format_arg = format!("--format=%H|%h|%s|%aI");
        let mut args = vec!["log", branch, &format_arg];

        let since_arg = since.map(|s| format!("--since={}", s));
        if let Some(ref s) = since_arg {
            args.push(s);
        }

        let until_arg = until.map(|u| format!("--until={}", u));
        if let Some(ref u) = until_arg {
            args.push(u);
        }

        let output = self.run_git(&args)?;

        let mut commits = Vec::new();
        for line in output.lines() {
            if line.is_empty() {
                continue;
            }

            let parts: Vec<&str> = line.splitn(4, '|').collect();
            if parts.len() < 4 {
                continue;
            }

            let hash = parts[0].to_string();
            let files_changed = self.get_commit_stats(&hash)?;

            commits.push(Commit {
                hash,
                short_hash: parts[1].to_string(),
                message: parts[2].to_string(),
                timestamp: parts[3].to_string(),
                files_changed,
            });
        }

        Ok(commits)
    }

    fn get_commit_stats(&self, hash: &str) -> Result<Vec<FileChange>> {
        // Get numstat for this commit
        let output = self.run_git(&["show", hash, "--numstat", "--format="])?;

        let mut changes = Vec::new();
        for line in output.lines() {
            if line.is_empty() {
                continue;
            }

            let parts: Vec<&str> = line.split('\t').collect();
            if parts.len() < 3 {
                continue;
            }

            // Handle binary files (shown as "-")
            let additions = parts[0].parse().unwrap_or(0);
            let deletions = parts[1].parse().unwrap_or(0);

            changes.push(FileChange {
                additions,
                deletions,
                path: parts[2].to_string(),
            });
        }

        Ok(changes)
    }

    pub fn collect_summary(
        &self,
        branch: &str,
        since: Option<&str>,
        until: Option<&str>,
    ) -> Result<GitSummaryData> {
        let commits = self.get_commits(branch, since, until)?;

        // Calculate area stats (group by top-level directory)
        let mut area_map: std::collections::HashMap<String, (usize, i32, i32)> =
            std::collections::HashMap::new();

        let mut total_additions = 0;
        let mut total_deletions = 0;

        for commit in &commits {
            // Track which areas this commit touched (for commit count)
            let mut commit_areas: std::collections::HashSet<String> =
                std::collections::HashSet::new();

            for file in &commit.files_changed {
                let area = extract_area(&file.path);
                commit_areas.insert(area.clone());

                let entry = area_map.entry(area).or_insert((0, 0, 0));
                entry.1 += file.additions;
                entry.2 += file.deletions;

                total_additions += file.additions;
                total_deletions += file.deletions;
            }

            // Increment commit count for each area touched
            for area in commit_areas {
                let entry = area_map.entry(area).or_insert((0, 0, 0));
                entry.0 += 1;
            }
        }

        let mut area_stats: Vec<AreaStats> = area_map
            .into_iter()
            .map(|(path, (commit_count, additions, deletions))| AreaStats {
                path,
                commit_count,
                additions,
                deletions,
            })
            .collect();

        // Sort by commit count descending
        area_stats.sort_by(|a, b| b.commit_count.cmp(&a.commit_count));

        // Build date range string
        let date_range = match (since, until) {
            (Some(s), Some(u)) => format!("{} to {}", s, u),
            (Some(s), None) => format!("since {}", s),
            (None, Some(u)) => format!("until {}", u),
            (None, None) => "all time".to_string(),
        };

        let actual_branch = if branch == "HEAD" {
            self.get_current_branch().unwrap_or_else(|_| "HEAD".to_string())
        } else {
            branch.to_string()
        };

        Ok(GitSummaryData {
            branch: actual_branch,
            date_range,
            commits,
            area_stats,
            total_additions,
            total_deletions,
        })
    }
}

/// Extract the "area" from a file path (top-level directory or root)
fn extract_area(path: &str) -> String {
    let parts: Vec<&str> = path.split('/').collect();
    if parts.len() > 1 {
        format!("{}/", parts[0])
    } else {
        "(root)".to_string()
    }
}
