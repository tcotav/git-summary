use clap::{Parser, ValueEnum};

#[derive(Debug, Clone, ValueEnum)]
pub enum OutputFormat {
    Pretty,
    Markdown,
    Json,
}

#[derive(Parser, Debug)]
#[command(name = "git-summary")]
#[command(about = "Summarize git commits using LLM", long_about = None)]
pub struct Args {
    /// Specific date to summarize (YYYY-MM-DD)
    #[arg(long)]
    pub date: Option<String>,

    /// Start date for range (YYYY-MM-DD or relative like "3 days ago")
    #[arg(long)]
    pub since: Option<String>,

    /// End date for range (YYYY-MM-DD or relative like "yesterday")
    #[arg(long)]
    pub until: Option<String>,

    /// Branch to summarize
    #[arg(long, short, default_value = "HEAD")]
    pub branch: String,

    /// Output format
    #[arg(long, short, value_enum, default_value = "pretty")]
    pub format: OutputFormat,

    /// Include file lists and detailed stats
    #[arg(long, short)]
    pub verbose: bool,

    /// Minimal output - just the summary
    #[arg(long, short)]
    pub quiet: bool,

    /// Path to git repository (defaults to current directory)
    #[arg(long, default_value = ".")]
    pub repo: String,

    /// Skip LLM summary, just show git data
    #[arg(long)]
    pub no_llm: bool,
}

impl Args {
    /// Resolve date arguments into a (since, until) pair for git log
    pub fn resolve_date_range(&self) -> (Option<String>, Option<String>) {
        if let Some(ref date) = self.date {
            // Single date: from start of day to end of day
            let since = format!("{} 00:00:00", date);
            let until = format!("{} 23:59:59", date);
            (Some(since), Some(until))
        } else {
            (self.since.clone(), self.until.clone())
        }
    }
}
