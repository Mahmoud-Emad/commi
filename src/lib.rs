pub mod ai;
pub mod cli;
pub mod colors;
pub mod commands;
pub mod config;
pub mod git;
pub mod safety;
pub mod update;
pub mod utils;

pub use ai::GeminiClient;
pub use cli::Args;
pub use config::Config;
pub use git::GitRepo;
