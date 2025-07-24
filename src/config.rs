use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::PathBuf;

use crate::cli::{Args, GenerateArgs};

#[derive(Debug, Serialize, Deserialize)]
pub struct CommiConfig {
    #[serde(rename = "commi-config")]
    pub config: ConfigSection,
    #[serde(default)]
    pub cache: CacheSection,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigSection {
    #[serde(rename = "api-key")]
    pub api_key: Option<String>,
    pub model: Option<String>,
    #[serde(rename = "max-tokens")]
    pub max_tokens: Option<usize>,
    #[serde(rename = "chunk-overlap")]
    pub chunk_overlap: Option<usize>,
    #[serde(rename = "enable-chunking")]
    pub enable_chunking: Option<bool>,
    #[serde(rename = "validate-format")]
    pub validate_format: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct CacheSection {
    #[serde(rename = "last-version-check")]
    pub last_version_check: Option<DateTime<Utc>>,
    #[serde(rename = "latest-version")]
    pub latest_version: Option<String>,
}

impl Default for CommiConfig {
    fn default() -> Self {
        Self {
            config: ConfigSection {
                api_key: None,
                model: None,
                max_tokens: None,
                chunk_overlap: None,
                enable_chunking: None,
                validate_format: None,
            },
            cache: CacheSection::default(),
        }
    }
}

pub struct Config {
    pub api_key: String,
    pub model_name: String,
    pub repo_path: PathBuf,
    pub max_tokens: usize,
    pub chunk_overlap: usize,
    pub enable_chunking: bool,
    pub validate_format: bool,
}

impl Config {
    pub fn load(args: &Args) -> Result<Self> {
        // Load .env file if it exists
        dotenv::dotenv().ok();

        // Load TOML config file if it exists
        let toml_config = Self::load_toml_config()?;

        // Get API key from args, environment, or config file
        let api_key = args
            .api_key
            .clone()
            .or_else(|| env::var("COMMI_API_KEY").ok())
            .or_else(|| env::var("GEMINI_API_KEY").ok())
            .or_else(|| toml_config.config.api_key.clone())
            .ok_or_else(|| {
                anyhow::anyhow!(
                    "API key not found. Please set it using:\n\
                     - Environment variable: export COMMI_API_KEY=\"your_key\"\n\
                     - Command line: commi config set api-key \"your_key\"\n\
                     - Or use --api-key flag\n\n\
                     Get your API key at: https://makersuite.google.com/app/apikey"
                )
            })?;

        // Validate API key format
        Self::validate_api_key(&api_key)?;

        // Get model name from environment, config file, or use default
        let model_name = env::var("MODEL_NAME")
            .ok()
            .or_else(|| toml_config.config.model.clone())
            .unwrap_or_else(|| "gemini-1.5-flash".to_string());

        // Get chunking configuration from environment or config file
        let max_tokens = env::var("COMMI_MAX_TOKENS")
            .ok()
            .and_then(|s| s.parse().ok())
            .or_else(|| toml_config.config.max_tokens)
            .unwrap_or(800_000); // Conservative limit for Gemini 1.5 Flash

        let chunk_overlap = env::var("COMMI_CHUNK_OVERLAP")
            .ok()
            .and_then(|s| s.parse().ok())
            .or_else(|| toml_config.config.chunk_overlap)
            .unwrap_or(200); // Number of lines to overlap between chunks

        let enable_chunking = env::var("COMMI_ENABLE_CHUNKING")
            .ok()
            .map(|s| s.to_lowercase() == "true" || s == "1")
            .or_else(|| toml_config.config.enable_chunking)
            .unwrap_or(true); // Enable chunking by default

        let validate_format = env::var("COMMI_VALIDATE_FORMAT")
            .ok()
            .map(|s| s.to_lowercase() == "true" || s == "1")
            .or_else(|| toml_config.config.validate_format)
            .unwrap_or(true); // Enable validation by default

        // Get repository path
        let repo_path = if let Some(repo) = &args.repo {
            PathBuf::from(repo)
        } else {
            env::current_dir().context("Failed to get current directory")?
        };

        // Validate repository path
        if !repo_path.exists() {
            anyhow::bail!("Repository path does not exist: {}", repo_path.display());
        }

        Ok(Config {
            api_key,
            model_name,
            repo_path,
            max_tokens,
            chunk_overlap,
            enable_chunking,
            validate_format,
        })
    }

    pub fn load_from_generate_args(args: &GenerateArgs) -> Result<Self> {
        // Load .env file if it exists
        dotenv::dotenv().ok();

        // Load TOML config file if it exists
        let toml_config = Self::load_toml_config()?;

        // Get API key from args, environment, or config file
        let api_key = args
            .api_key
            .clone()
            .or_else(|| env::var("COMMI_API_KEY").ok())
            .or_else(|| env::var("GEMINI_API_KEY").ok())
            .or_else(|| toml_config.config.api_key.clone())
            .ok_or_else(|| {
                anyhow::anyhow!(
                    "API key not found. Please set it using:\n\
                     - Environment variable: export COMMI_API_KEY=\"your_key\"\n\
                     - Command line: commi config set api-key \"your_key\"\n\
                     - Or use --api-key flag\n\n\
                     Get your API key at: https://makersuite.google.com/app/apikey"
                )
            })?;

        // Validate API key format
        Self::validate_api_key(&api_key)?;

        // Get model name from environment, config file, or use default
        let model_name = env::var("MODEL_NAME")
            .ok()
            .or_else(|| toml_config.config.model.clone())
            .unwrap_or_else(|| "gemini-1.5-flash".to_string());

        // Get chunking configuration from environment or config file
        let max_tokens = env::var("COMMI_MAX_TOKENS")
            .ok()
            .and_then(|s| s.parse().ok())
            .or_else(|| toml_config.config.max_tokens)
            .unwrap_or(800_000); // Conservative limit for Gemini 1.5 Flash

        let chunk_overlap = env::var("COMMI_CHUNK_OVERLAP")
            .ok()
            .and_then(|s| s.parse().ok())
            .or_else(|| toml_config.config.chunk_overlap)
            .unwrap_or(200); // Number of lines to overlap between chunks

        let enable_chunking = env::var("COMMI_ENABLE_CHUNKING")
            .ok()
            .map(|s| s.to_lowercase() == "true" || s == "1")
            .or_else(|| toml_config.config.enable_chunking)
            .unwrap_or(true); // Enable chunking by default

        let validate_format = env::var("COMMI_VALIDATE_FORMAT")
            .ok()
            .map(|s| s.to_lowercase() == "true" || s == "1")
            .or_else(|| toml_config.config.validate_format)
            .unwrap_or(true); // Enable validation by default

        // Get repository path
        let repo_path = if let Some(repo) = &args.repo {
            PathBuf::from(repo)
        } else {
            env::current_dir().context("Failed to get current directory")?
        };

        // Validate repository path
        if !repo_path.exists() {
            anyhow::bail!("Repository path does not exist: {}", repo_path.display());
        }

        Ok(Config {
            api_key,
            model_name,
            repo_path,
            max_tokens,
            chunk_overlap,
            enable_chunking,
            validate_format,
        })
    }

    /// Validate API key format
    fn validate_api_key(api_key: &str) -> Result<()> {
        if api_key.is_empty() {
            anyhow::bail!("API key cannot be empty. Please set COMMI_API_KEY environment variable or use --api-key");
        }

        if api_key.len() < 20 {
            anyhow::bail!("API key appears to be too short. Please check your Gemini AI API key");
        }

        if !api_key.starts_with("AIza") {
            log::warn!(
                "API key doesn't start with 'AIza' - this may not be a valid Gemini AI API key"
            );
        }

        Ok(())
    }

    /// Validate configuration values
    pub fn validate(&self) -> Result<()> {
        // Validate repository path
        if !self.repo_path.exists() {
            anyhow::bail!(
                "Repository path does not exist: {}",
                self.repo_path.display()
            );
        }

        // Validate token limits
        if self.max_tokens == 0 {
            anyhow::bail!("Max tokens must be greater than 0");
        }

        if self.max_tokens > 2_000_000 {
            log::warn!(
                "Max tokens ({}) is very high, this may cause API issues",
                self.max_tokens
            );
        }

        // Validate chunk overlap
        if self.chunk_overlap > 10_000 {
            log::warn!(
                "Chunk overlap ({}) is very high, this may cause performance issues",
                self.chunk_overlap
            );
        }

        Ok(())
    }

    /// Set a configuration value by key and save to TOML
    pub fn set_config_value(key: &str, value: &str) -> Result<()> {
        // Load existing config
        let mut config = Self::load_toml_config()?;

        match key {
            "api-key" => {
                Self::validate_api_key(value)?;
                config.config.api_key = Some(value.to_string());
                env::set_var("COMMI_API_KEY", value);
            }
            "model" => {
                config.config.model = Some(value.to_string());
                env::set_var("MODEL_NAME", value);
            }
            "max-tokens" => {
                let tokens: usize = value
                    .parse()
                    .context("Invalid value for max-tokens, must be a number")?;
                config.config.max_tokens = Some(tokens);
                env::set_var("COMMI_MAX_TOKENS", tokens.to_string());
            }
            "chunk-overlap" => {
                let overlap: usize = value
                    .parse()
                    .context("Invalid value for chunk-overlap, must be a number")?;
                config.config.chunk_overlap = Some(overlap);
                env::set_var("COMMI_CHUNK_OVERLAP", overlap.to_string());
            }
            "enable-chunking" => {
                let enabled = matches!(value.to_lowercase().as_str(), "true" | "1" | "yes" | "on");
                config.config.enable_chunking = Some(enabled);
                env::set_var("COMMI_ENABLE_CHUNKING", enabled.to_string());
            }
            "validate-format" => {
                let enabled = matches!(value.to_lowercase().as_str(), "true" | "1" | "yes" | "on");
                config.config.validate_format = Some(enabled);
                env::set_var("COMMI_VALIDATE_FORMAT", enabled.to_string());
            }
            _ => anyhow::bail!("Unknown configuration key: {}", key),
        }

        // Save updated config
        Self::save_toml_config(&config)?;
        Ok(())
    }

    /// Reset configuration to defaults
    pub fn reset_config(key: Option<&str>) -> Result<()> {
        match key {
            Some(key) => {
                match key {
                    "api-key" => env::remove_var("COMMI_API_KEY"),
                    "model" => env::remove_var("MODEL_NAME"),
                    "max-tokens" => env::remove_var("COMMI_MAX_TOKENS"),
                    "chunk-overlap" => env::remove_var("COMMI_CHUNK_OVERLAP"),
                    "enable-chunking" => env::remove_var("COMMI_ENABLE_CHUNKING"),
                    "validate-format" => env::remove_var("COMMI_VALIDATE_FORMAT"),
                    _ => return Err(anyhow::anyhow!("Unknown configuration key: {}", key)),
                }
                Ok(())
            }
            None => {
                // Reset all configuration
                env::remove_var("COMMI_API_KEY");
                env::remove_var("MODEL_NAME");
                env::remove_var("COMMI_MAX_TOKENS");
                env::remove_var("COMMI_CHUNK_OVERLAP");
                env::remove_var("COMMI_ENABLE_CHUNKING");
                Ok(())
            }
        }
    }

    /// List all configuration values
    pub fn list_config(&self) -> HashMap<String, String> {
        let mut config = HashMap::new();

        // Only show first/last few characters of API key for security
        let masked_api_key = if self.api_key.len() > 8 {
            format!(
                "{}...{}",
                &self.api_key[..4],
                &self.api_key[self.api_key.len() - 4..]
            )
        } else {
            "***".to_string()
        };

        config.insert("api-key".to_string(), masked_api_key);
        config.insert("model".to_string(), self.model_name.clone());
        config.insert("max-tokens".to_string(), self.max_tokens.to_string());
        config.insert("chunk-overlap".to_string(), self.chunk_overlap.to_string());
        config.insert(
            "enable-chunking".to_string(),
            self.enable_chunking.to_string(),
        );
        config.insert(
            "repo-path".to_string(),
            self.repo_path.display().to_string(),
        );

        config
    }

    /// Load TOML configuration from file
    pub fn load_toml_config() -> Result<CommiConfig> {
        let config_path = Self::get_toml_config_path()?;

        if config_path.exists() {
            let content = fs::read_to_string(&config_path).context("Failed to read config file")?;
            let config: CommiConfig =
                toml::from_str(&content).context("Failed to parse TOML config file")?;
            Ok(config)
        } else {
            Ok(CommiConfig::default())
        }
    }

    /// Get the TOML configuration file path
    fn get_toml_config_path() -> Result<PathBuf> {
        // Use test-specific config file when running tests
        if env::var("COMMI_TEST_MODE").is_ok() {
            let temp_dir = env::temp_dir();
            return Ok(temp_dir.join("commi_test.toml"));
        }

        let config_dir = if let Ok(xdg_config) = env::var("XDG_CONFIG_HOME") {
            PathBuf::from(xdg_config)
        } else if let Ok(home) = env::var("HOME") {
            PathBuf::from(home).join(".config")
        } else {
            anyhow::bail!("Could not determine config directory");
        };

        fs::create_dir_all(&config_dir).context("Failed to create config directory")?;

        Ok(config_dir.join("commi.toml"))
    }

    /// Save TOML configuration to file
    pub fn save_toml_config(config: &CommiConfig) -> Result<()> {
        let config_path = Self::get_toml_config_path()?;
        let content =
            toml::to_string_pretty(config).context("Failed to serialize config to TOML")?;

        fs::write(&config_path, content).context("Failed to write config file")?;

        Ok(())
    }

    /// Get a specific configuration value from TOML file
    pub fn get_toml_config_value(key: &str) -> Result<String> {
        let config = Self::load_toml_config()?;

        match key {
            "api-key" => config
                .config
                .api_key
                .ok_or_else(|| anyhow::anyhow!("API key not set")),
            "model" => Ok(config
                .config
                .model
                .unwrap_or_else(|| "gemini-1.5-flash".to_string())),
            "max-tokens" => Ok(config.config.max_tokens.unwrap_or(800_000).to_string()),
            "chunk-overlap" => Ok(config.config.chunk_overlap.unwrap_or(200).to_string()),
            "enable-chunking" => Ok(config.config.enable_chunking.unwrap_or(true).to_string()),
            "validate-format" => Ok(config.config.validate_format.unwrap_or(true).to_string()),
            "version" => Ok(config
                .cache
                .latest_version
                .unwrap_or_else(|| "unknown".to_string())),
            "last-version-check" => Ok(config
                .cache
                .last_version_check
                .map(|dt| dt.to_rfc3339())
                .unwrap_or_else(|| "never".to_string())),
            _ => anyhow::bail!("Unknown configuration key: {}", key),
        }
    }

    /// Show the entire TOML configuration file
    pub fn show_toml_config() -> Result<()> {
        let config_path = Self::get_toml_config_path()?;

        if config_path.exists() {
            let content = fs::read_to_string(&config_path).context("Failed to read config file")?;
            println!("{}", content);
        } else {
            println!("Configuration file not found at: {}", config_path.display());
            println!("Use 'commi config set <key> <value>' to create configuration.");
        }

        Ok(())
    }
}
