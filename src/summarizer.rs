use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

use crate::git::GitSummaryData;

#[derive(Debug, Serialize)]
struct AnthropicRequest {
    model: String,
    max_tokens: u32,
    messages: Vec<Message>,
}

#[derive(Debug, Serialize)]
struct Message {
    role: String,
    content: String,
}

#[derive(Debug, Deserialize)]
struct AnthropicResponse {
    content: Vec<ContentBlock>,
}

#[derive(Debug, Deserialize)]
struct ContentBlock {
    text: String,
}

pub struct Summarizer {
    api_key: String,
    model: String,
}

impl Summarizer {
    pub fn new() -> Result<Self> {
        let api_key =
            std::env::var("ANTHROPIC_API_KEY").context("ANTHROPIC_API_KEY not set")?;

        Ok(Self {
            api_key,
            model: "claude-sonnet-4-20250514".to_string(),
        })
    }

    pub async fn summarize(&self, data: &GitSummaryData) -> Result<String> {
        let prompt = self.build_prompt(data);

        let request = AnthropicRequest {
            model: self.model.clone(),
            max_tokens: 1024,
            messages: vec![Message {
                role: "user".to_string(),
                content: prompt,
            }],
        };

        let client = reqwest::Client::new();
        let response = client
            .post("https://api.anthropic.com/v1/messages")
            .header("x-api-key", &self.api_key)
            .header("anthropic-version", "2023-06-01")
            .header("content-type", "application/json")
            .json(&request)
            .send()
            .await
            .context("Failed to send request to Anthropic API")?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            anyhow::bail!("Anthropic API error ({}): {}", status, body);
        }

        let result: AnthropicResponse = response
            .json()
            .await
            .context("Failed to parse Anthropic response")?;

        result
            .content
            .first()
            .map(|c| c.text.clone())
            .context("No content in response")
    }

    fn build_prompt(&self, data: &GitSummaryData) -> String {
        let mut prompt = String::new();

        prompt.push_str("Summarize the following git commits as a bulleted list. ");
        prompt.push_str("Each bullet should describe a theme or area of work. ");
        prompt.push_str("Group related changes together conceptually. ");
        prompt.push_str("Use simple '- ' for bullets. Keep each bullet to 1-2 sentences. ");
        prompt.push_str("Do not list individual commits or mention specific authors.\n\n");

        prompt.push_str(&format!(
            "Branch: {}\nDate range: {}\nTotal commits: {}\n\n",
            data.branch,
            data.date_range,
            data.commits.len()
        ));

        prompt.push_str("Areas changed:\n");
        for area in &data.area_stats {
            prompt.push_str(&format!(
                "  {} - {} commits, +{}/-{} lines\n",
                area.path, area.commit_count, area.additions, area.deletions
            ));
        }

        prompt.push_str("\nCommit messages:\n");
        for commit in &data.commits {
            prompt.push_str(&format!("  - {}\n", commit.message));
        }

        prompt
    }
}
