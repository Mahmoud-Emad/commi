use std::fmt;

/// Custom error types for Commi
#[derive(Debug)]
#[allow(dead_code)] // Will be used as we integrate error handling
pub enum CommiError {
    /// Git repository related errors
    Git(GitError),
    /// AI/API related errors
    Ai(AiError),
    /// Configuration related errors
    Config(ConfigError),
    /// IO related errors
    Io(std::io::Error),
    /// Network related errors
    Network(NetworkError),
    /// Validation errors
    Validation(ValidationError),
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum GitError {
    /// Repository not found or invalid
    RepositoryNotFound(String),
    /// No changes found in repository
    NoChanges,
    /// No staged changes found
    NoStagedChanges,
    /// Git operation failed
    OperationFailed(String),
    /// Invalid repository state
    InvalidState(String),
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum AiError {
    /// API key is invalid or missing
    InvalidApiKey,
    /// API quota exceeded
    QuotaExceeded,
    /// Rate limit exceeded
    RateLimitExceeded,
    /// API request failed
    RequestFailed(String),
    /// Invalid response from API
    InvalidResponse(String),
    /// Service temporarily unavailable
    ServiceUnavailable,
    /// Chunking failed
    ChunkingFailed(String),
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum ConfigError {
    /// Missing required configuration
    MissingRequired(String),
    /// Invalid configuration value
    InvalidValue(String, String),
    /// Configuration file not found
    FileNotFound(String),
    /// Failed to parse configuration
    ParseError(String),
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum NetworkError {
    /// Connection timeout
    Timeout,
    /// DNS resolution failed
    DnsResolution,
    /// Connection refused
    ConnectionRefused,
    /// SSL/TLS error
    TlsError(String),
    /// General network error
    General(String),
}

#[derive(Debug)]
#[allow(dead_code)]
#[allow(clippy::enum_variant_names)] // All variants are validation errors
pub enum ValidationError {
    /// Invalid email format
    InvalidEmail(String),
    /// Invalid repository path
    InvalidPath(String),
    /// Invalid API key format
    InvalidApiKeyFormat,
    /// Invalid commit message format
    InvalidCommitMessage(String),
}

impl fmt::Display for CommiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CommiError::Git(e) => write!(f, "Git error: {e}"),
            CommiError::Ai(e) => write!(f, "AI error: {e}"),
            CommiError::Config(e) => write!(f, "Configuration error: {e}"),
            CommiError::Io(e) => write!(f, "IO error: {e}"),
            CommiError::Network(e) => write!(f, "Network error: {e}"),
            CommiError::Validation(e) => write!(f, "Validation error: {e}"),
        }
    }
}

impl fmt::Display for GitError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GitError::RepositoryNotFound(path) => {
                write!(f, "Git repository not found at '{path}'. Make sure you're in a Git repository or specify a valid path with --repo")
            }
            GitError::NoChanges => {
                write!(
                    f,
                    "No changes found in the repository. Make some changes and try again"
                )
            }
            GitError::NoStagedChanges => {
                write!(
                    f,
                    "No staged changes found. Use 'git add' to stage changes before using --cached"
                )
            }
            GitError::OperationFailed(op) => {
                write!(f, "Git operation failed: {op}")
            }
            GitError::InvalidState(msg) => {
                write!(f, "Invalid repository state: {msg}")
            }
        }
    }
}

impl fmt::Display for AiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AiError::InvalidApiKey => {
                write!(f, "Invalid API key. Please check your COMMI_API_KEY or use --api-key with a valid Gemini AI API key.\n\nGet your API key at: https://makersuite.google.com/app/apikey")
            }
            AiError::QuotaExceeded => {
                write!(f, "API quota exceeded. Please check your Gemini AI usage limits or try again later")
            }
            AiError::RateLimitExceeded => {
                write!(f, "Rate limit exceeded. Please wait a moment and try again")
            }
            AiError::RequestFailed(msg) => {
                write!(f, "API request failed: {msg}")
            }
            AiError::InvalidResponse(msg) => {
                write!(f, "Invalid response from AI service: {msg}")
            }
            AiError::ServiceUnavailable => {
                write!(
                    f,
                    "Gemini AI service is temporarily unavailable. Please try again later"
                )
            }
            AiError::ChunkingFailed(msg) => {
                write!(f, "Failed to process large diff: {msg}")
            }
        }
    }
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConfigError::MissingRequired(field) => {
                write!(f, "Missing required configuration: {field}")
            }
            ConfigError::InvalidValue(field, value) => {
                write!(
                    f,
                    "Invalid value '{value}' for configuration field '{field}'"
                )
            }
            ConfigError::FileNotFound(path) => {
                write!(f, "Configuration file not found: {path}")
            }
            ConfigError::ParseError(msg) => {
                write!(f, "Failed to parse configuration: {msg}")
            }
        }
    }
}

impl fmt::Display for NetworkError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NetworkError::Timeout => {
                write!(
                    f,
                    "Request timed out. Please check your internet connection and try again"
                )
            }
            NetworkError::DnsResolution => {
                write!(
                    f,
                    "DNS resolution failed. Please check your internet connection"
                )
            }
            NetworkError::ConnectionRefused => {
                write!(
                    f,
                    "Connection refused. The service may be temporarily unavailable"
                )
            }
            NetworkError::TlsError(msg) => {
                write!(f, "TLS/SSL error: {msg}")
            }
            NetworkError::General(msg) => {
                write!(f, "Network error: {msg}")
            }
        }
    }
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ValidationError::InvalidEmail(email) => {
                write!(
                    f,
                    "Invalid email format: '{email}'. Please provide a valid email address"
                )
            }
            ValidationError::InvalidPath(path) => {
                write!(
                    f,
                    "Invalid path: '{path}'. Please provide a valid file or directory path"
                )
            }
            ValidationError::InvalidApiKeyFormat => {
                write!(
                    f,
                    "Invalid API key format. Please provide a valid Gemini AI API key"
                )
            }
            ValidationError::InvalidCommitMessage(msg) => {
                write!(f, "Invalid commit message format: {msg}")
            }
        }
    }
}

impl std::error::Error for CommiError {}
impl std::error::Error for GitError {}
impl std::error::Error for AiError {}
impl std::error::Error for ConfigError {}
impl std::error::Error for NetworkError {}
impl std::error::Error for ValidationError {}

// Conversion implementations for easier error handling
impl From<std::io::Error> for CommiError {
    fn from(err: std::io::Error) -> Self {
        CommiError::Io(err)
    }
}

impl From<git2::Error> for CommiError {
    fn from(err: git2::Error) -> Self {
        match err.code() {
            git2::ErrorCode::NotFound => CommiError::Git(GitError::RepositoryNotFound(
                "Git repository not found".to_string(),
            )),
            _ => CommiError::Git(GitError::OperationFailed(err.message().to_string())),
        }
    }
}

impl From<reqwest::Error> for CommiError {
    fn from(err: reqwest::Error) -> Self {
        if err.is_timeout() {
            CommiError::Network(NetworkError::Timeout)
        } else if err.is_connect() {
            CommiError::Network(NetworkError::ConnectionRefused)
        } else {
            CommiError::Network(NetworkError::General(err.to_string()))
        }
    }
}

/// Result type alias for Commi operations
#[allow(dead_code)]
pub type Result<T> = std::result::Result<T, CommiError>;
