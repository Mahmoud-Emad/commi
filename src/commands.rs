use anyhow::Result;
use clap::CommandFactory;
use clap_complete::{generate, Shell};
use log::info;
use std::io;

use crate::colors::{
    init_theme, key as color_key, success as color_success, value as color_value,
    warning as color_warning,
};
use crate::safety::{confirm_dangerous_operation, warn_about_important_files, DangerousOperation};

use crate::ai::GeminiClient;
use crate::cli::{
    Cli, Commands, CompletionCommands, ConfigCommands, GenerateArgs, ModelCommands, StatusArgs,
    UpdateCommands,
};
use crate::config::Config;
use crate::git::GitRepo;
use crate::update;
use crate::utils::copy_to_clipboard;

pub async fn handle_command(cli: Cli) -> Result<()> {
    // Set up logging based on verbosity
    let log_level = if cli.verbose {
        log::LevelFilter::Debug
    } else {
        log::LevelFilter::Info
    };

    env_logger::Builder::from_default_env()
        .filter_level(log_level)
        .format_timestamp_secs()
        .format_module_path(false)
        .format_target(false)
        .init();

    // Initialize theme based on CLI options
    init_theme(cli.no_color);

    match cli.command {
        Some(Commands::Generate(args)) => handle_generate(args).await,
        Some(Commands::Config(cmd)) => handle_config(cmd).await,
        Some(Commands::Model(cmd)) => handle_model(cmd).await,
        Some(Commands::Completion(cmd)) => handle_completion(cmd),
        Some(Commands::Update(cmd)) => handle_update_command(cmd).await,
        Some(Commands::Status(args)) => handle_status(args).await,
        None => {
            // Show help when no command is provided
            show_modern_help();
            Ok(())
        }
    }
}

pub async fn handle_generate(args: GenerateArgs) -> Result<()> {
    let start_time = std::time::Instant::now();

    // Load configuration
    log::debug!("Loading configuration...");
    let config = Config::load_from_generate_args(&args)?;
    config.validate()?;
    log::debug!("Configuration loaded and validated successfully");

    // Check for updates (non-blocking)
    if let Ok(update_available) = update::check_for_updates().await {
        if update_available {
            info!("{}", color_warning("A new version of Commi is available!"));
            info!(
                "{}",
                color_warning("Run 'commi update install' to update to the latest version.")
            );
        }
    }

    // Initialize Git repository
    log::debug!(
        "Initializing Git repository at: {}",
        config.repo_path.display()
    );
    let repo = GitRepo::new(&config.repo_path)?;
    log::debug!("Git repository initialized successfully");

    // Check if there are changes
    log::debug!("Checking for changes (cached: {})", args.cached);
    if !repo.has_changes(args.cached)? {
        if args.cached {
            anyhow::bail!("No staged changes found. Use 'git add' to stage changes.");
        } else {
            anyhow::bail!("No changes found in the repository.");
        }
    }

    // Get git diff
    log::debug!("Generating git diff...");
    let diff = repo.get_diff(args.cached)?;
    if diff.is_empty() {
        anyhow::bail!("No changes found in the git diff.");
    }
    log::debug!(
        "Git diff generated successfully ({} characters)",
        diff.len()
    );

    // Initialize AI client
    let ai_client = if config.enable_chunking {
        GeminiClient::new_with_full_config(
            &config.api_key,
            &config.model_name,
            config.max_tokens,
            config.chunk_overlap,
            config.validate_format,
        )?
    } else {
        // Use a very high token limit to effectively disable chunking
        GeminiClient::new_with_full_config(
            &config.api_key,
            &config.model_name,
            usize::MAX,
            config.chunk_overlap,
            config.validate_format,
        )?
    };

    // Generate commit message
    info!("Generating commit message...");
    let mut commit_message = ai_client.generate_commit_message(&diff).await?;

    // Add co-author if specified
    if let Some(co_author) = &args.co_author {
        if !co_author.contains('@') {
            anyhow::bail!("Invalid co-author email. Please provide a valid email address.");
        }
        let author_name = co_author.split('@').next().unwrap();
        commit_message.push_str(&format!("\n\nCo-authored-by: {author_name} <{co_author}>"));
    }

    // Display the generated commit message
    println!("\n{}", color_success("Generated Commit Message:"));
    println!("\n{commit_message}\n");

    // Handle commit operation
    if args.commit {
        if !repo.has_staged_changes()? {
            anyhow::bail!("No staged changes to commit. Use 'git add' to stage changes.");
        }

        // Show warning about important files
        warn_about_important_files(&config.repo_path)?;

        // Enhanced safety check for commit operation
        let has_unstaged = repo.has_changes(false)? && !repo.has_changes(true)?;
        let files_count = repo.count_changed_files(true)?; // Count staged files
        let operation = DangerousOperation::GitCommit {
            files_count,
            has_staged: true,
            has_unstaged,
        };

        if !confirm_dangerous_operation(operation)? {
            return Ok(());
        }

        info!("Committing changes...");
        repo.commit(&commit_message)?;
        info!("{}", color_success("Changes committed successfully!"));
    }

    // Handle copy operation
    if args.copy {
        copy_to_clipboard(&commit_message)?;
        info!("{}", color_success("Commit message copied to clipboard."));
    } else if !args.commit {
        info!(
            "{}",
            color_warning("Use --copy to copy the message or --commit to commit changes.")
        );
    }

    let elapsed = start_time.elapsed();
    log::debug!("Total execution time: {:.2}ms", elapsed.as_millis());

    Ok(())
}

pub async fn handle_model(cmd: ModelCommands) -> Result<()> {
    match cmd {
        ModelCommands::List => {
            println!("Supported AI Models:");
            println!();
            println!("  gemini-1.5-flash      Fast and efficient (default)");
            println!("  gemini-2.5-flash      Latest fast model");
            println!("  gemini-2.5-flash-lite Lightweight version");
            println!("  gemini-2.5-pro        Most capable model");
            println!();
            println!("To change model:");
            println!("  commi config set model gemini-2.5-pro");
            println!();
            println!("Current model:");
            let config = Config::load_toml_config()?;
            let current_model = config
                .config
                .model
                .unwrap_or_else(|| "gemini-1.5-flash".to_string());
            println!("  {current_model}");
        }
    }
    Ok(())
}

pub async fn handle_config(cmd: ConfigCommands) -> Result<()> {
    match cmd {
        ConfigCommands::Set { key, value } => {
            info!(
                "Setting configuration: {} = {}",
                color_key(&key),
                color_value(&value)
            );
            Config::set_config_value(&key, &value)?;
            info!("Configuration updated successfully");
            Ok(())
        }
        ConfigCommands::Get { key } => {
            match key {
                Some(key) => {
                    // Get specific configuration value
                    info!("Getting configuration for: {}", color_key(&key));
                    let value = Config::get_toml_config_value(&key)?;
                    println!("{}: {}", color_key(&key), color_value(&value));
                }
                None => {
                    // Show entire config file
                    info!("Configuration file contents:");
                    Config::show_toml_config()?;
                }
            }
            Ok(())
        }
        ConfigCommands::List => {
            info!("Configuration values:");

            // Load actual configuration
            let args = crate::cli::Args::default();
            let config = Config::load(&args)?;
            let config_data = config.list_config();

            // Display configuration in simple format
            println!("Key\t\tValue");
            println!("---\t\t-----");
            for (key, value) in config_data {
                println!("{key}\t\t{value}");
            }

            Ok(())
        }
        ConfigCommands::Reset { key, yes } => {
            let operation = DangerousOperation::ConfigReset {
                all_config: key.is_none(),
                key: key.clone(),
            };

            let confirm = if yes {
                true
            } else {
                confirm_dangerous_operation(operation)?
            };

            if confirm {
                if let Some(key) = key {
                    info!("Resetting configuration key: {}", color_key(&key));
                    Config::reset_config(Some(&key))?;
                    info!("Configuration key '{key}' reset successfully");
                } else {
                    info!("Resetting all configuration to defaults");
                    Config::reset_config(None)?;
                    info!("All configuration reset to defaults");
                }
            }
            Ok(())
        }
    }
}

pub fn handle_completion(cmd: CompletionCommands) -> Result<()> {
    let mut app = Cli::command();
    let app_name = app.get_name().to_string();

    match cmd {
        CompletionCommands::Bash => {
            generate(Shell::Bash, &mut app, app_name, &mut io::stdout());
        }
        CompletionCommands::Zsh => {
            generate(Shell::Zsh, &mut app, app_name, &mut io::stdout());
        }
        CompletionCommands::Fish => {
            generate(Shell::Fish, &mut app, app_name, &mut io::stdout());
        }
    }

    Ok(())
}

pub async fn handle_update_command(cmd: UpdateCommands) -> Result<()> {
    match cmd {
        UpdateCommands::Check => {
            info!("Checking for updates...");

            let current_version = update::get_current_version();
            let latest_version = update::get_latest_version().await?;

            match update::compare_versions(&current_version, &latest_version)? {
                std::cmp::Ordering::Less => {
                    info!("{}", color_success("A new version is available!"));
                    info!("Current version: {current_version}");
                    info!("Latest version: {latest_version}");
                    info!("Run 'commi update install' to install the latest version.");
                }
                std::cmp::Ordering::Equal => {
                    info!(
                        "{}",
                        color_success("You are already using the latest version.")
                    );
                }
                std::cmp::Ordering::Greater => {
                    info!("You are using a newer version than the latest release.");
                    info!("Current version: {current_version}");
                    info!("Latest release: {latest_version}");
                }
            }
            Ok(())
        }
        UpdateCommands::Install { force } => {
            // Get actual version information
            let current_version = update::get_current_version();
            let new_version = match update::get_latest_version().await {
                Ok(version) => version,
                Err(e) => {
                    anyhow::bail!("Failed to check for updates: {}", e);
                }
            };

            // Check if update is actually needed
            match update::compare_versions(&current_version, &new_version)? {
                std::cmp::Ordering::Equal => {
                    if force {
                        info!("Force reinstalling current version ({current_version})");
                    } else {
                        info!("Already up to date (version {current_version})");
                        return Ok(());
                    }
                }
                std::cmp::Ordering::Greater => {
                    info!(
                        "The latest version is already installed (current: {current_version}, latest: {new_version})"
                    );
                    return Ok(());
                }
                std::cmp::Ordering::Less => {
                    // Update is available, continue
                    info!("Update available: {current_version} → {new_version}");
                }
            }

            // Enhanced safety check for binary update
            let operation = DangerousOperation::BinaryUpdate {
                current_version: current_version.clone(),
                new_version: new_version.clone(),
                force,
            };

            if !confirm_dangerous_operation(operation)? {
                return Ok(());
            }

            if force {
                info!("Force installing update...");
            } else {
                info!("Installing update...");
            }
            update::update_binary().await
        }
    }
}

pub async fn handle_status(args: StatusArgs) -> Result<()> {
    info!("Checking repository status...");

    // Get repository path
    let repo_path = if let Some(repo) = &args.repo {
        std::path::PathBuf::from(repo)
    } else {
        std::env::current_dir()?
    };

    // Initialize git repository
    let git_repo = GitRepo::new(&repo_path)?;

    if args.verbose {
        info!("Repository path: {}", repo_path.display());
    }

    // Check if there are any changes
    let has_staged = git_repo.has_changes(true)?;
    let has_unstaged = git_repo.has_changes(false)?;

    if !has_staged && !has_unstaged {
        println!("No changes detected in repository.");
        return Ok(());
    }

    // Get actual git status
    let status_entries = git_repo.get_status_entries()?;

    if status_entries.is_empty() {
        println!("No changes detected in repository.");
        return Ok(());
    }

    // Display status
    println!("Repository Status:");
    println!();
    for entry in status_entries {
        let status_char = match entry.status {
            git2::Status::INDEX_NEW => "A ",
            git2::Status::INDEX_MODIFIED => "M ",
            git2::Status::INDEX_DELETED => "D ",
            git2::Status::INDEX_RENAMED => "R ",
            git2::Status::WT_NEW => "??",
            git2::Status::WT_MODIFIED => " M",
            git2::Status::WT_DELETED => " D",
            _ => "  ",
        };
        println!("  {} {}", status_char, entry.path);
    }

    println!();
    if has_staged {
        println!("Staged changes ready for commit.");
    }
    if has_unstaged {
        println!("Unstaged changes detected.");
    }

    Ok(())
}

/// Show simple help interface
fn show_modern_help() {
    println!();
    println!("Commi v4.0.0 - AI-Powered Git Commit Message Generator");
    println!();
    println!("USAGE:");
    println!("  commi <COMMAND>");
    println!();
    println!("COMMANDS:");
    println!("  generate      Generate commit messages");
    println!("  config        Manage configuration");
    println!("  model         List and manage AI models");
    println!("  status        Show repository status");
    println!("  update        Check for updates");
    println!("  completion    Generate shell completions");
    println!();
    println!("SETUP:");
    println!("  commi config set api-key \"your_key\"");
    println!();
    println!("For more help: commi <command> --help");
    println!();
}
