use clap::{Args as ClapArgs, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(
    name = "commi",
    version = "4.0.0",
    about = "AI-Powered Git Commit Message Generator"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,

    /// Enable verbose output
    #[arg(short, long, global = true)]
    pub verbose: bool,

    /// Disable colored output
    #[arg(long, global = true)]
    pub no_color: bool,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Generate commit messages (default command)
    #[command(alias = "gen")]
    Generate(GenerateArgs),

    /// Manage configuration
    #[command(subcommand)]
    Config(ConfigCommands),

    /// List and manage AI models
    #[command(subcommand)]
    Model(ModelCommands),

    /// Generate shell completion scripts
    #[command(subcommand)]
    Completion(CompletionCommands),

    /// Update management
    #[command(subcommand)]
    Update(UpdateCommands),

    /// Show repository and tool status
    Status(StatusArgs),
}

#[derive(ClapArgs, Debug)]
pub struct GenerateArgs {
    /// Path to Git repository (defaults to current directory)
    #[arg(short = 'r', long = "repo")]
    pub repo: Option<String>,

    /// Gemini AI API key (or set COMMI_API_KEY env var)
    #[arg(short = 'k', long = "api-key", env = "COMMI_API_KEY")]
    pub api_key: Option<String>,

    /// Use staged changes only
    #[arg(short = 'c', long = "cached")]
    pub cached: bool,

    /// Copy message to clipboard
    #[arg(short = 't', long = "copy")]
    pub copy: bool,

    /// Auto commit with generated message
    #[arg(short = 'm', long = "commit")]
    pub commit: bool,

    /// Add a co-author to the commit
    #[arg(short = 'a', long = "co-author")]
    pub co_author: Option<String>,
}

#[derive(Subcommand, Debug)]
pub enum ConfigCommands {
    /// Set a configuration value
    Set {
        /// Configuration key (api-key, model, max-tokens, etc.)
        key: String,
        /// Configuration value
        value: String,
    },
    /// Get a configuration value or show all config
    Get {
        /// Configuration key to retrieve (optional - shows all if omitted)
        key: Option<String>,
    },
    /// List all configuration values
    List,
    /// Reset configuration to defaults
    Reset {
        /// Reset specific key only
        #[arg(short, long)]
        key: Option<String>,
        /// Skip confirmation prompt
        #[arg(short = 'y', long)]
        yes: bool,
    },
}

#[derive(Subcommand, Debug)]
pub enum ModelCommands {
    /// List all supported AI models
    List,
}

#[derive(Subcommand, Debug)]
pub enum CompletionCommands {
    /// Generate bash completion script
    Bash,
    /// Generate zsh completion script
    Zsh,
    /// Generate fish completion script
    Fish,
}

#[derive(Subcommand, Debug)]
pub enum UpdateCommands {
    /// Check for available updates
    Check,
    /// Install available updates
    Install {
        /// Force reinstall even if already up to date
        #[arg(short, long)]
        force: bool,
    },
}

#[derive(ClapArgs, Debug)]
pub struct StatusArgs {
    /// Path to Git repository (defaults to current directory)
    #[arg(short = 'r', long = "repo")]
    pub repo: Option<String>,

    /// Show detailed status information
    #[arg(short, long)]
    pub verbose: bool,
}

// Legacy Args struct for backward compatibility
#[derive(Parser, Debug)]
#[command(
    name = "commi",
    version = "4.0.0",
    about = "AI-powered Git commit message generator using Gemini AI",
    hide = true, // Hide from help to encourage new CLI usage
)]
pub struct LegacyArgs {
    /// Path to Git repository (defaults to current directory)
    #[arg(short = 'r', long = "repo")]
    pub repo: Option<String>,

    /// Gemini AI API key (or set COMMI_API_KEY env var)
    #[arg(short = 'k', long = "api-key", env = "COMMI_API_KEY")]
    pub api_key: Option<String>,

    /// Use staged changes only
    #[arg(short = 'c', long = "cached")]
    pub cached: bool,

    /// Copy message to clipboard
    #[arg(short = 't', long = "copy")]
    pub copy: bool,

    /// Auto commit with generated message
    #[arg(short = 'm', long = "commit")]
    pub commit: bool,

    /// Add a co-author to the commit
    #[arg(short = 'a', long = "co-author")]
    pub co_author: Option<String>,

    /// Update Commi to the latest version
    #[arg(short = 'u', long = "update")]
    pub update: bool,
}

// For backward compatibility, keep the old Args type alias
pub type Args = LegacyArgs;

impl Default for LegacyArgs {
    fn default() -> Self {
        Self {
            repo: None,
            api_key: None,
            cached: false,
            copy: false,
            commit: false,
            co_author: None,
            update: false,
        }
    }
}
