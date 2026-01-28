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
    let (since, until) = args.resolve_date_range();

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

    // Get LLM summary (or placeholder if --no-llm)
    let summary = if args.no_llm {
        "(LLM summary skipped)".to_string()
    } else {
        let summarizer = Summarizer::new()?;
        summarizer.summarize(&data).await?
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
