use anyhow::{Context, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct GenerateRequest {
    contents: Vec<Content>,
}

#[derive(Serialize)]
struct Content {
    parts: Vec<Part>,
}

#[derive(Serialize)]
struct Part {
    text: String,
}

#[derive(Deserialize)]
struct GenerateResponse {
    candidates: Vec<Candidate>,
}

#[derive(Deserialize)]
struct Candidate {
    content: ResponseContent,
}

#[derive(Deserialize)]
struct ResponseContent {
    parts: Vec<ResponsePart>,
}

#[derive(Deserialize)]
struct ResponsePart {
    text: String,
}

pub struct GeminiClient {
    client: Client,
    api_key: String,
    model_name: String,
    max_retries: usize,
    max_tokens: usize,
    #[allow(dead_code)] // Reserved for future chunk overlap functionality
    chunk_overlap: usize,
    validate_format: bool,
    last_request_time: std::sync::Mutex<Option<std::time::Instant>>,
}

impl GeminiClient {
    #[allow(dead_code)] // Public API for backward compatibility
    pub fn new(api_key: &str, model_name: &str) -> Result<Self> {
        Self::new_with_config(api_key, model_name, 800_000, 200)
    }

    pub fn new_with_config(
        api_key: &str,
        model_name: &str,
        max_tokens: usize,
        chunk_overlap: usize,
    ) -> Result<Self> {
        Self::new_with_full_config(api_key, model_name, max_tokens, chunk_overlap, true)
    }

    pub fn new_with_full_config(
        api_key: &str,
        model_name: &str,
        max_tokens: usize,
        chunk_overlap: usize,
        validate_format: bool,
    ) -> Result<Self> {
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .connect_timeout(std::time::Duration::from_secs(10))
            .pool_idle_timeout(std::time::Duration::from_secs(30))
            .pool_max_idle_per_host(2)
            .user_agent(concat!(
                env!("CARGO_PKG_NAME"),
                "/",
                env!("CARGO_PKG_VERSION")
            ))
            .build()
            .context("Failed to create HTTP client")?;

        Ok(GeminiClient {
            client,
            api_key: api_key.to_string(),
            model_name: model_name.to_string(),
            max_retries: 3,
            max_tokens,
            chunk_overlap,
            validate_format,
            last_request_time: std::sync::Mutex::new(None),
        })
    }

    /// Estimate the number of tokens in a text string
    /// Uses a conservative estimate of 4 characters per token
    fn estimate_tokens(&self, text: &str) -> usize {
        // Conservative estimate: 1 token ≈ 4 characters for most languages
        // This is based on OpenAI's tokenization, Gemini might be slightly different
        // but this gives us a safe upper bound
        (text.len() + 3) / 4
    }

    /// Check if text exceeds the token limit
    fn exceeds_token_limit(&self, text: &str) -> bool {
        self.estimate_tokens(text) > self.max_tokens
    }

    /// Check for potentially suspicious content in diff
    fn contains_suspicious_content(&self, text: &str) -> bool {
        let suspicious_patterns = [
            "password",
            "secret",
            "token",
            "api_key",
            "private_key",
            "-----BEGIN",
            "ssh-rsa",
            "ssh-ed25519",
        ];

        let text_lower = text.to_lowercase();
        suspicious_patterns
            .iter()
            .any(|pattern| text_lower.contains(pattern))
    }

    /// Implement basic rate limiting to avoid hitting API limits
    async fn rate_limit(&self) {
        const MIN_REQUEST_INTERVAL: std::time::Duration = std::time::Duration::from_millis(100);

        let sleep_duration = {
            if let Ok(mut last_time) = self.last_request_time.lock() {
                let sleep_duration = if let Some(last) = *last_time {
                    let elapsed = last.elapsed();
                    if elapsed < MIN_REQUEST_INTERVAL {
                        Some(MIN_REQUEST_INTERVAL - elapsed)
                    } else {
                        None
                    }
                } else {
                    None
                };
                *last_time = Some(std::time::Instant::now());
                sleep_duration
            } else {
                None
            }
        };

        if let Some(duration) = sleep_duration {
            log::debug!("Rate limiting: sleeping for {duration:?}");
            tokio::time::sleep(duration).await;
        }
    }

    /// Split a diff into chunks while preserving file boundaries
    fn chunk_diff(&self, diff_text: &str) -> Vec<String> {
        // If the diff is small enough, return it as a single chunk
        if !self.exceeds_token_limit(diff_text) {
            return vec![diff_text.to_string()];
        }

        let mut chunks = Vec::with_capacity(4); // Pre-allocate for common case
        let mut current_chunk = String::with_capacity(self.max_tokens * 2); // Pre-allocate
        let mut current_file_content = String::with_capacity(1024); // Pre-allocate
        let mut in_file = false;

        for line in diff_text.lines() {
            // Check if this is a file header (starts with "diff --git" or "+++/---")
            if line.starts_with("diff --git")
                || line.starts_with("index ")
                || line.starts_with("+++")
                || line.starts_with("---")
            {
                // If we were building a file and the current chunk would exceed limits,
                // save the current chunk and start a new one
                if in_file && !current_file_content.is_empty() {
                    let test_chunk = format!("{current_chunk}\n{current_file_content}");
                    if self.exceeds_token_limit(&test_chunk) && !current_chunk.is_empty() {
                        chunks.push(current_chunk.clone());
                        current_chunk = current_file_content.clone();
                    } else {
                        current_chunk = test_chunk;
                    }
                    current_file_content.clear();
                }

                // Start a new file
                in_file = true;
                current_file_content.push_str(line);
                current_file_content.push('\n');
            } else {
                // Add line to current file content
                current_file_content.push_str(line);
                current_file_content.push('\n');
            }
        }

        // Handle the last file
        if !current_file_content.is_empty() {
            let test_chunk = format!("{current_chunk}\n{current_file_content}");
            if self.exceeds_token_limit(&test_chunk) && !current_chunk.is_empty() {
                chunks.push(current_chunk);
                chunks.push(current_file_content);
            } else {
                chunks.push(test_chunk);
            }
        } else if !current_chunk.is_empty() {
            chunks.push(current_chunk);
        }

        // If we still have chunks that are too large, split them more aggressively
        let mut final_chunks = Vec::new();
        for chunk in chunks {
            if self.exceeds_token_limit(&chunk) {
                final_chunks.extend(self.split_large_chunk(&chunk));
            } else {
                final_chunks.push(chunk);
            }
        }

        final_chunks
    }

    /// Split a large chunk more aggressively by lines
    fn split_large_chunk(&self, chunk: &str) -> Vec<String> {
        let lines: Vec<&str> = chunk.lines().collect();
        let mut chunks = Vec::new();
        let mut current_chunk = String::new();

        for line in lines {
            let test_chunk = if current_chunk.is_empty() {
                line.to_string()
            } else {
                format!("{current_chunk}\n{line}")
            };

            if self.exceeds_token_limit(&test_chunk) {
                if !current_chunk.is_empty() {
                    chunks.push(current_chunk);
                    current_chunk = line.to_string();
                } else {
                    // Even a single line is too large, truncate it
                    chunks.push(line[..line.len().min(self.max_tokens * 4)].to_string());
                }
            } else {
                current_chunk = test_chunk;
            }
        }

        if !current_chunk.is_empty() {
            chunks.push(current_chunk);
        }

        chunks
    }

    pub async fn generate_commit_message(&self, diff_text: &str) -> Result<String> {
        // Input validation
        if diff_text.trim().is_empty() {
            anyhow::bail!("Diff text cannot be empty");
        }

        // Security: Check for potentially malicious content
        if self.contains_suspicious_content(diff_text) {
            log::warn!("Diff contains potentially suspicious content, proceeding with caution");
        }

        // Check if we need to chunk the diff
        if self.exceeds_token_limit(diff_text) {
            log::info!("Diff is large, splitting into chunks...");
            return self.generate_commit_message_chunked(diff_text).await;
        }

        // Process normally for small diffs
        for attempt in 0..self.max_retries {
            match self.try_generate_commit_message(diff_text, attempt).await {
                Ok(message) => {
                    if self.is_valid_commit_message(&message) {
                        return Ok(message);
                    }
                    log::warn!("Improving commit message format...");
                    // Try to validate and format the message
                    if let Ok(validated) = self.validate_and_format_commit_message(&message).await {
                        return Ok(validated);
                    }
                }
                Err(e) if attempt == self.max_retries - 1 => return Err(e),
                Err(e) => {
                    // Don't retry for authentication/API key errors
                    if e.to_string().contains("Invalid API key")
                        || e.to_string().contains("Authentication failed")
                        || e.to_string().contains("Access forbidden")
                    {
                        return Err(e);
                    }
                    log::warn!("Attempt {} failed, retrying...", attempt + 1);
                }
            }
        }

        anyhow::bail!(
            "Failed to generate valid commit message after {} attempts",
            self.max_retries
        )
    }

    /// Generate commit message for large diffs by processing chunks
    async fn generate_commit_message_chunked(&self, diff_text: &str) -> Result<String> {
        let chunks = self.chunk_diff(diff_text);
        log::info!("Processing {} chunks", chunks.len());

        let mut chunk_messages = Vec::new();

        for (i, chunk) in chunks.iter().enumerate() {
            log::info!("Processing chunk {} of {}", i + 1, chunks.len());

            // Generate commit message for this chunk
            match self
                .try_generate_commit_message_for_chunk(chunk, i + 1, chunks.len())
                .await
            {
                Ok(message) => chunk_messages.push(message),
                Err(e) => {
                    log::warn!("Failed to process chunk {}: {}", i + 1, e);
                    // Continue with other chunks, but note the failure
                    chunk_messages.push(format!("Changes in chunk {}", i + 1));
                }
            }
        }

        // Combine the chunk messages into a single commit message
        let combined_message = self.combine_chunk_messages(chunk_messages).await?;
        self.validate_and_format_commit_message(&combined_message)
            .await
    }

    /// Generate commit message for a single chunk
    async fn try_generate_commit_message_for_chunk(
        &self,
        chunk: &str,
        chunk_num: usize,
        total_chunks: usize,
    ) -> Result<String> {
        let prompt = self.build_chunk_commit_message_prompt(chunk, chunk_num, total_chunks);

        let request = GenerateRequest {
            contents: vec![Content {
                parts: vec![Part { text: prompt }],
            }],
        };

        let url = format!(
            "https://generativelanguage.googleapis.com/v1beta/models/{}:generateContent?key={}",
            self.model_name, self.api_key
        );

        // Apply rate limiting
        self.rate_limit().await;

        let response = self
            .client
            .post(&url)
            .json(&request)
            .send()
            .await
            .context("Failed to send request to Gemini API")?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();

            let user_message = match status.as_u16() {
                400 => {
                    if error_text.contains("API_KEY_INVALID")
                        || error_text.contains("API key not valid")
                    {
                        "Invalid API key. Please check your COMMI_API_KEY or use --api-key with a valid Gemini AI API key.\n\nGet your API key at: https://makersuite.google.com/app/apikey"
                    } else if error_text.contains("quota") || error_text.contains("QUOTA_EXCEEDED")
                    {
                        "API quota exceeded. Please check your Gemini AI usage limits or try again later."
                    } else {
                        "Bad request. Please check your input and try again."
                    }
                }
                401 => "Authentication failed. Please check your API key.",
                403 => "Access forbidden. Your API key may not have the required permissions.",
                429 => {
                    eprintln!("Rate limit exceeded. Please wait a moment and try again.");
                    eprintln!();
                    eprintln!("Tip: Consider changing the model to reduce rate limits:");
                    eprintln!("   commi model list                    # See supported models");
                    eprintln!(
                        "   commi config set model gemini-2.5-pro  # Change to a different model"
                    );
                    eprintln!();
                    eprintln!("Supported models:");
                    eprintln!("   • gemini-1.5-flash      Fast and efficient (default)");
                    eprintln!("   • gemini-2.5-flash      Latest fast model");
                    eprintln!("   • gemini-2.5-flash-lite Lightweight version");
                    eprintln!("   • gemini-2.5-pro        Most capable model");
                    "Rate limit exceeded. Please wait a moment and try again."
                }
                500..=599 => {
                    "Gemini AI service is temporarily unavailable. Please try again later."
                }
                _ => "API request failed. Please check your internet connection and try again.",
            };

            anyhow::bail!("{}", user_message);
        }

        let response_data: GenerateResponse = response
            .json()
            .await
            .context("Failed to parse API response")?;

        let commit_message = response_data
            .candidates
            .first()
            .and_then(|c| c.content.parts.first())
            .map(|p| p.text.trim().to_string())
            .context("No commit message in API response")?;

        Ok(commit_message)
    }

    /// Build a prompt for generating commit message for a chunk
    fn build_chunk_commit_message_prompt(
        &self,
        chunk: &str,
        chunk_num: usize,
        total_chunks: usize,
    ) -> String {
        self.load_chunk_analysis_prompt(chunk, chunk_num, total_chunks)
            .unwrap_or_else(|e| {
                log::error!("Failed to load chunk analysis prompt: {e}");
                "Error: Could not load chunk analysis prompt. Please check the embedded prompt content.".to_string()
            })
    }

    /// Combine multiple chunk messages into a single coherent commit message
    async fn combine_chunk_messages(&self, chunk_messages: Vec<String>) -> Result<String> {
        if chunk_messages.is_empty() {
            anyhow::bail!("No chunk messages to combine");
        }

        if chunk_messages.len() == 1 {
            return Ok(chunk_messages[0].clone());
        }

        // Create a prompt to combine the chunk descriptions
        let combined_descriptions = chunk_messages.join("\n- ");
        let combine_prompt = self.load_chunk_combination_prompt(&combined_descriptions)
            .unwrap_or_else(|e| {
                log::error!("Failed to load chunk combination prompt: {e}");
                "Error: Could not load chunk combination prompt. Please check the embedded prompt content.".to_string()
            });

        // Send the combine prompt to the AI
        let request = GenerateRequest {
            contents: vec![Content {
                parts: vec![Part {
                    text: combine_prompt,
                }],
            }],
        };

        let url = format!(
            "https://generativelanguage.googleapis.com/v1beta/models/{}:generateContent?key={}",
            self.model_name, self.api_key
        );

        // Apply rate limiting
        self.rate_limit().await;

        let response = self
            .client
            .post(&url)
            .json(&request)
            .send()
            .await
            .context("Failed to send combine request to Gemini API")?;

        if !response.status().is_success() {
            // If combining fails, fall back to a simple combination
            log::warn!("Failed to combine chunk messages via AI, using fallback");
            return Ok(self.fallback_combine_messages(chunk_messages));
        }

        let response_data: GenerateResponse = response
            .json()
            .await
            .context("Failed to parse combine response")?;

        let combined_message = response_data
            .candidates
            .first()
            .and_then(|c| c.content.parts.first())
            .map(|p| p.text.trim().to_string())
            .context("No combined message in API response")?;

        // Validate the combined message
        if self.is_valid_commit_message(&combined_message) {
            Ok(combined_message)
        } else {
            log::warn!("Combined message is not valid, using fallback");
            Ok(self.fallback_combine_messages(chunk_messages))
        }
    }

    /// Fallback method to combine messages when AI combination fails
    fn fallback_combine_messages(&self, chunk_messages: Vec<String>) -> String {
        // Try to determine the most common type from chunk messages
        let mut type_counts = std::collections::HashMap::new();
        let commit_types = [
            "feat", "fix", "docs", "style", "refactor", "perf", "test", "build", "ci", "chore",
        ];

        for message in &chunk_messages {
            for commit_type in &commit_types {
                if message.to_lowercase().contains(commit_type) {
                    *type_counts.entry(commit_type).or_insert(0) += 1;
                }
            }
        }

        let most_common_type = type_counts
            .iter()
            .max_by_key(|(_, count)| *count)
            .map(|(t, _)| **t)
            .unwrap_or("chore");

        // Create a simple combined message
        let summary = if chunk_messages.len() > 3 {
            format!("{most_common_type}: update multiple components")
        } else {
            format!("{most_common_type}: update components")
        };

        let mut combined = summary;
        combined.push_str("\n\n");

        for (i, message) in chunk_messages.iter().enumerate() {
            combined.push_str(&format!("- {}", message.trim()));
            if i < chunk_messages.len() - 1 {
                combined.push('\n');
            }
        }

        combined
    }

    async fn try_generate_commit_message(
        &self,
        diff_text: &str,
        retry_count: usize,
    ) -> Result<String> {
        let prompt = self.build_commit_message_prompt(diff_text, retry_count);

        let request = GenerateRequest {
            contents: vec![Content {
                parts: vec![Part { text: prompt }],
            }],
        };

        let url = format!(
            "https://generativelanguage.googleapis.com/v1beta/models/{}:generateContent?key={}",
            self.model_name, self.api_key
        );

        // Apply rate limiting
        self.rate_limit().await;

        let response = self
            .client
            .post(&url)
            .json(&request)
            .send()
            .await
            .context("Failed to send request to Gemini API")?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();

            // Parse and provide user-friendly error messages
            let user_message = match status.as_u16() {
                400 => {
                    if error_text.contains("API_KEY_INVALID")
                        || error_text.contains("API key not valid")
                    {
                        "Invalid API key. Please check your COMMI_API_KEY or use --api-key with a valid Gemini AI API key.\n\nGet your API key at: https://makersuite.google.com/app/apikey"
                    } else if error_text.contains("quota") || error_text.contains("QUOTA_EXCEEDED")
                    {
                        "API quota exceeded. Please check your Gemini AI usage limits or try again later."
                    } else {
                        "Bad request. Please check your input and try again."
                    }
                }
                401 => "Authentication failed. Please check your API key.",
                403 => "Access forbidden. Your API key may not have the required permissions.",
                429 => {
                    eprintln!("Rate limit exceeded. Please wait a moment and try again.");
                    eprintln!();
                    eprintln!("Tip: Consider changing the model to reduce rate limits:");
                    eprintln!("   commi model list                    # See supported models");
                    eprintln!(
                        "   commi config set model gemini-2.5-pro  # Change to a different model"
                    );
                    eprintln!();
                    eprintln!("Supported models:");
                    eprintln!("   • gemini-1.5-flash      Fast and efficient (default)");
                    eprintln!("   • gemini-2.5-flash      Latest fast model");
                    eprintln!("   • gemini-2.5-flash-lite Lightweight version");
                    eprintln!("   • gemini-2.5-pro        Most capable model");
                    "Rate limit exceeded. Please wait a moment and try again."
                }
                500..=599 => {
                    "Gemini AI service is temporarily unavailable. Please try again later."
                }
                _ => "API request failed. Please check your internet connection and try again.",
            };

            anyhow::bail!("{}", user_message);
        }

        let response_data: GenerateResponse = response
            .json()
            .await
            .context("Failed to parse API response")?;

        let commit_message = response_data
            .candidates
            .first()
            .and_then(|c| c.content.parts.first())
            .map(|p| p.text.trim().to_string())
            .context("No commit message in API response")?;

        Ok(commit_message)
    }

    fn build_commit_message_prompt(&self, diff_text: &str, retry_count: usize) -> String {
        let commit_types = [
            ("feat", "New feature"),
            ("fix", "Bug fix"),
            ("docs", "Documentation changes"),
            ("style", "Code style changes (formatting, etc)"),
            ("refactor", "Code refactoring"),
            ("perf", "Performance improvements"),
            ("test", "Adding or updating tests"),
            ("build", "Build system changes"),
            ("ci", "CI/CD changes"),
            ("chore", "General maintenance"),
            ("revert", "Reverting changes"),
            ("merge", "Merge commits"),
        ];

        let retry_guidance = if retry_count > 0 {
            "\nPlease strictly follow the commit message format guidelines."
        } else {
            ""
        };

        self.load_commit_message_generation_prompt(diff_text, retry_guidance, &commit_types)
            .unwrap_or_else(|e| {
                log::error!("Failed to load commit message generation prompt: {e}");
                "Error: Could not load commit message generation prompt. Please check the embedded prompt content.".to_string()
            })
    }

    fn is_valid_commit_message(&self, message: &str) -> bool {
        let lines: Vec<&str> = message.lines().collect();

        if lines.is_empty() {
            return false;
        }

        // Check summary line length
        let summary = lines[0].trim();
        if summary.len() > 72 {
            return false;
        }

        // Check for conventional commit format
        let commit_types = [
            "feat", "fix", "docs", "style", "refactor", "perf", "test", "build", "ci", "chore",
            "revert", "merge",
        ];

        if let Some(colon_pos) = summary.find(':') {
            let prefix = &summary[..colon_pos].to_lowercase();
            return commit_types.contains(&prefix.as_str());
        }

        false
    }

    /// Validate and format commit message using AI to ensure best practices
    async fn validate_and_format_commit_message(&self, message: &str) -> Result<String> {
        // If validation is disabled, return as-is
        if !self.validate_format {
            return Ok(message.to_string());
        }

        // If basic validation passes, return as-is
        if self.is_valid_commit_message(message) {
            return Ok(message.to_string());
        }

        log::info!("Validating and formatting commit message...");

        let validation_prompt = self.load_validation_prompt(message)?;

        let request = GenerateRequest {
            contents: vec![Content {
                parts: vec![Part {
                    text: validation_prompt,
                }],
            }],
        };

        let url = format!(
            "https://generativelanguage.googleapis.com/v1beta/models/{}:generateContent?key={}",
            self.model_name, self.api_key
        );

        // Apply rate limiting
        self.rate_limit().await;

        let response = self
            .client
            .post(&url)
            .json(&request)
            .send()
            .await
            .context("Failed to send validation request to Gemini API")?;

        if !response.status().is_success() {
            log::warn!("Validation request failed, returning original message");
            return Ok(message.to_string());
        }

        let response_data: GenerateResponse = response
            .json()
            .await
            .context("Failed to parse validation response")?;

        let validated_message = response_data
            .candidates
            .first()
            .and_then(|c| c.content.parts.first())
            .map(|p| p.text.trim().to_string())
            .unwrap_or_else(|| message.to_string());

        Ok(validated_message)
    }

    /// Load validation prompt from embedded content
    fn load_validation_prompt(&self, message: &str) -> Result<String> {
        // Load the prompt from embedded content
        let content = include_str!("../ai_prompts/commit_message_validation.md");

        // Replace the placeholder with the actual message
        let prompt = content.replace("{commit_message}", message);
        Ok(prompt)
    }

    /// Load chunk analysis prompt from embedded content
    fn load_chunk_analysis_prompt(
        &self,
        chunk: &str,
        chunk_num: usize,
        total_chunks: usize,
    ) -> Result<String> {
        // Load the prompt from embedded content
        let content = include_str!("../ai_prompts/chunk_analysis.md");

        // Replace placeholders with actual values
        let prompt = content
            .replace("{chunk_num}", &chunk_num.to_string())
            .replace("{total_chunks}", &total_chunks.to_string())
            .replace("{chunk}", chunk);

        Ok(prompt)
    }

    /// Load chunk combination prompt from embedded content
    fn load_chunk_combination_prompt(&self, chunk_descriptions: &str) -> Result<String> {
        // Load the prompt from embedded content
        let content = include_str!("../ai_prompts/chunk_combination.md");

        // Replace placeholder with actual descriptions
        let prompt = content.replace("{chunk_descriptions}", chunk_descriptions);
        Ok(prompt)
    }

    /// Load commit message generation prompt from embedded content
    fn load_commit_message_generation_prompt(
        &self,
        diff_text: &str,
        retry_guidance: &str,
        commit_types: &[(&str, &str)],
    ) -> Result<String> {
        // Load the prompt from embedded content
        let content = include_str!("../ai_prompts/commit_message_generation.md");

        let formatted_types = commit_types
            .iter()
            .map(|(t, d)| format!("   {t}: {d}"))
            .collect::<Vec<_>>()
            .join("\n");

        // Replace placeholders with actual values
        let prompt = content
            .replace("{commit_types}", &formatted_types)
            .replace("{retry_guidance}", retry_guidance)
            .replace("{diff_text}", diff_text);

        Ok(prompt)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_estimate_tokens() {
        let client = GeminiClient::new_with_config("test", "test", 1000, 100).unwrap();

        // Test basic token estimation
        assert_eq!(client.estimate_tokens("hello"), 2); // 5 chars / 4 = 1.25 -> 2
        assert_eq!(client.estimate_tokens("hello world"), 3); // 11 chars / 4 = 2.75 -> 3
        assert_eq!(client.estimate_tokens(""), 0); // 0 chars / 4 = 0
    }

    #[test]
    fn test_exceeds_token_limit() {
        let client = GeminiClient::new_with_config("test", "test", 10, 100).unwrap();

        // 10 tokens = 40 characters max
        assert!(!client.exceeds_token_limit("short")); // 5 chars = 2 tokens
        assert!(!client.exceeds_token_limit("exactly forty characters in this string")); // 39 chars = 10 tokens
        assert!(client.exceeds_token_limit(
            "this string is definitely longer than forty characters and should exceed the limit"
        )); // 83 chars = 21 tokens
    }

    #[test]
    fn test_chunk_diff_small() {
        let client = GeminiClient::new_with_config("test", "test", 1000, 100).unwrap();
        let small_diff = "diff --git a/file.txt b/file.txt\n+added line\n-removed line";

        let chunks = client.chunk_diff(small_diff);
        assert_eq!(chunks.len(), 1);
        assert_eq!(chunks[0], small_diff);
    }

    #[test]
    fn test_chunk_diff_large() {
        let client = GeminiClient::new_with_config("test", "test", 50, 100).unwrap(); // Very small limit

        // Create a large diff that will exceed the token limit
        let mut large_diff = String::new();
        large_diff.push_str("diff --git a/file1.txt b/file1.txt\n");
        large_diff.push_str("index 1234567..abcdefg 100644\n");
        large_diff.push_str("--- a/file1.txt\n");
        large_diff.push_str("+++ b/file1.txt\n");
        for i in 0..20 {
            large_diff.push_str(&format!(
                "+this is a very long line number {i} that adds content\n"
            ));
        }

        large_diff.push_str("diff --git a/file2.txt b/file2.txt\n");
        large_diff.push_str("index 7654321..gfedcba 100644\n");
        large_diff.push_str("--- a/file2.txt\n");
        large_diff.push_str("+++ b/file2.txt\n");
        for i in 0..20 {
            large_diff.push_str(&format!(
                "+another very long line number {i} in second file\n"
            ));
        }

        let chunks = client.chunk_diff(&large_diff);
        assert!(
            chunks.len() > 1,
            "Large diff should be split into multiple chunks"
        );

        // Each chunk should be within the token limit
        for chunk in &chunks {
            assert!(
                !client.exceeds_token_limit(chunk),
                "Chunk should not exceed token limit: {}",
                chunk.len()
            );
        }
    }

    #[test]
    fn test_fallback_combine_messages() {
        let client = GeminiClient::new_with_config("test", "test", 1000, 100).unwrap();

        let messages = vec![
            "Add new feature for user authentication".to_string(),
            "Fix bug in login validation".to_string(),
            "Update documentation for API".to_string(),
        ];

        let combined = client.fallback_combine_messages(messages);

        // Should contain a commit type
        assert!(
            combined.contains("feat:") || combined.contains("fix:") || combined.contains("chore:")
        );

        // Should contain bullet points
        assert!(combined.contains("- Add new feature"));
        assert!(combined.contains("- Fix bug"));
        assert!(combined.contains("- Update documentation"));
    }

    #[test]
    fn test_split_large_chunk() {
        let client = GeminiClient::new_with_config("test", "test", 20, 100).unwrap(); // Very small limit

        let mut large_chunk = String::new();
        for i in 0..50 {
            large_chunk.push_str(&format!("This is line number {i} with some content\n"));
        }

        let chunks = client.split_large_chunk(&large_chunk);
        assert!(chunks.len() > 1, "Large chunk should be split");

        // Each chunk should be within the token limit
        for chunk in &chunks {
            assert!(
                !client.exceeds_token_limit(chunk),
                "Split chunk should not exceed token limit"
            );
        }
    }

    #[test]
    fn test_prompt_loading() {
        let client = GeminiClient::new_with_config("test", "test", 1000, 100).unwrap();

        // Test chunk analysis prompt loading
        let chunk_prompt = client.load_chunk_analysis_prompt("test chunk", 1, 2);
        assert!(chunk_prompt.is_ok(), "Should load chunk analysis prompt");
        let prompt = chunk_prompt.unwrap();
        assert!(
            prompt.contains("test chunk"),
            "Prompt should contain the chunk content"
        );
        assert!(prompt.contains("1"), "Prompt should contain chunk number");
        assert!(prompt.contains("2"), "Prompt should contain total chunks");

        // Test chunk combination prompt loading
        let combination_prompt = client.load_chunk_combination_prompt("test descriptions");
        assert!(
            combination_prompt.is_ok(),
            "Should load chunk combination prompt"
        );
        let prompt = combination_prompt.unwrap();
        assert!(
            prompt.contains("test descriptions"),
            "Prompt should contain descriptions"
        );

        // Test commit message generation prompt loading
        let commit_types = [("feat", "New feature"), ("fix", "Bug fix")];
        let generation_prompt =
            client.load_commit_message_generation_prompt("test diff", "", &commit_types);
        assert!(
            generation_prompt.is_ok(),
            "Should load commit message generation prompt"
        );
        let prompt = generation_prompt.unwrap();
        assert!(
            prompt.contains("test diff"),
            "Prompt should contain diff text"
        );
        assert!(
            prompt.contains("feat: New feature"),
            "Prompt should contain commit types"
        );

        // Test validation prompt loading
        let validation_prompt = client.load_validation_prompt("test message");
        assert!(validation_prompt.is_ok(), "Should load validation prompt");
        let prompt = validation_prompt.unwrap();
        assert!(
            prompt.contains("test message"),
            "Prompt should contain message"
        );
    }
}
