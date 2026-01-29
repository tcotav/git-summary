mod cli;
mod formatters;
mod git;
mod summarizer;

use anyhow::Result;
use clap::Parser;

use cli::{Args, OutputFormat};
use formatters::{Formatter, JsonFormatter, MarkdownFormatter, PrettyFormatter};
use git::GitRepo;
use summarizer::Summarizer;

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    // Initialize git repo
    let repo = GitRepo::new(&args.repo);

    // Resolve date range
    let (since, until, used_default) = args.resolve_date_range();

    if used_default {
        eprintln!("No date range specified, defaulting to last 1 day.");
    }

    // Collect git data
    let data = repo.collect_summary(
        &args.branch,
        since.as_deref(),
        until.as_deref(),
    )?;

    if data.commits.is_empty() {
        eprintln!("No commits found for the specified date range.");
        return Ok(());
    }

    // Get LLM summary (only if --llm flag is passed)
    let summary = if args.llm {
        let summarizer = Summarizer::new()?;
        summarizer.summarize(&data).await?
    } else {
        "(LLM summary skipped - use --llm to enable)".to_string()
    };

    // Format output
    let formatter: Box<dyn Formatter> = match args.format {
        OutputFormat::Pretty => Box::new(PrettyFormatter),
        OutputFormat::Markdown => Box::new(MarkdownFormatter),
        OutputFormat::Json => Box::new(JsonFormatter),
    };

    let output = formatter.format(&data, &summary, args.verbose, args.quiet);
    println!("{}", output);

    Ok(())
}
