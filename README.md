# git-summary

CLI tool to summarize git commits using an LLM.

## Installation

```bash
cargo build --release
# Binary will be at target/release/git-summary
```

## Usage

```bash
# Summarize today's commits
git-summary

# Specific date
git-summary --date 2025-01-27

# Date range (supports relative dates)
git-summary --since "3 days ago"
git-summary --since 2025-01-20 --until 2025-01-27

# Different output formats
git-summary --format pretty      # default, terminal colors
git-summary --format markdown    # for Slack/docs
git-summary --format json        # for piping to other tools

# Verbose mode (show file changes per commit)
git-summary --since yesterday --verbose

# Quiet mode (just the LLM summary)
git-summary --since yesterday --quiet

# Skip LLM call (useful for testing)
git-summary --since yesterday --no-llm

# Specify repo path
git-summary --repo /path/to/repo --since yesterday
```

## API Key Setup

This tool requires an Anthropic API key. Set it as an environment variable:

```bash
export ANTHROPIC_API_KEY=sk-ant-...
```

### Claude Code / Claude Pro Subscribers

If you have a Claude Pro subscription ($20/month) or use Claude Code, your subscription includes **$5/month of API credits**. To use them:

1. Go to [console.anthropic.com](https://console.anthropic.com)
2. Sign in with the same account as your Pro/Claude Code subscription
3. Navigate to **API Keys** and create a new key
4. Add it to your shell config:
   ```bash
   # ~/.zshrc or ~/.bashrc
   export ANTHROPIC_API_KEY=sk-ant-your-key-here
   ```

The $5 monthly credit goes far for this use case—each summary costs roughly $0.01-0.05 depending on commit volume.

## Output Example

```
════════════════════════════════════════════════════════════
  Git Summary: since 1 week ago (main)
  15 commits | +450 -120 lines
════════════════════════════════════════════════════════════

## Summary
- Authentication system received major updates with OAuth2 provider integration and token refresh flow
- Several bug fixes in the payment module including decimal precision and currency conversion
- New API endpoints added for user preferences management

## By Area
  src/auth/       8 commits,  +300/-80   lines
  src/payments/   4 commits,   +85/-40   lines
  src/api/        3 commits,  +200/-50   lines

## Commits
  2025-01-27 10:30 feat: Add OAuth2 provider integration
  2025-01-27 09:15 feat: Token refresh flow
  2025-01-26 16:45 fix: Session timeout handling
  ...
```
