use anyhow::Result;
use clap::Parser;

mod ai;
mod cli;
mod colors;
mod commands;
mod config;
mod error;
mod git;
mod safety;
mod update;
mod utils;

use cli::{Cli, LegacyArgs};
use colors::{error as color_error, init_theme};
use commands::handle_command;

#[tokio::main]
async fn main() -> Result<()> {
    // Try to parse as new CLI first
    let args: Vec<String> = std::env::args().collect();

    // Only use legacy mode if we have specific legacy flags without subcommands
    let has_legacy_flags = args.len() > 1
        && args
            .iter()
            .any(|arg| matches!(arg.as_str(), "--update" | "-u"))
        && !args.iter().any(|arg| {
            matches!(
                arg.as_str(),
                "generate" | "config" | "completion" | "update" | "status" | "gen"
            )
        });

    if has_legacy_flags {
        // Use legacy CLI for backward compatibility
        handle_legacy_cli().await
    } else {
        // Use new CLI structure
        let cli = Cli::parse();

        // Initialize theme based on CLI options
        init_theme(cli.no_color);

        if let Err(e) = handle_command(cli).await {
            use log::error;

            // Enhanced error handling with suggestions
            let error_msg = e.to_string();
            error!("{}", color_error(&error_msg));

            // Try to provide "did you mean?" suggestions
            if error_msg.contains("unrecognized subcommand") {
                suggest_command(&error_msg);
            }

            std::process::exit(1);
        }
        Ok(())
    }
}

async fn handle_legacy_cli() -> Result<()> {
    use colors::warning as color_warning;
    use log::error;
    use utils::print_header;

    // Initialize theme for legacy mode (no color override)
    init_theme(false);

    // Initialize logging for legacy mode
    env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Info)
        .format_timestamp_secs()
        .format_module_path(false)
        .format_target(false)
        .init();

    // Print deprecation warning
    eprintln!(
        "{}",
        color_warning("Warning: You are using the legacy CLI interface.")
    );
    eprintln!(
        "{}",
        color_warning("Consider migrating to the new subcommand structure:")
    );
    eprintln!("  Old: commi --cached --copy");
    eprintln!("  New: commi generate --cached --copy");
    eprintln!();

    // Print ASCII header
    print_header();

    // Parse legacy arguments
    let args = LegacyArgs::parse();

    // Handle legacy logic (same as before)
    if let Err(e) = run_legacy(args).await {
        error!("{}", color_error(&e.to_string()));
        std::process::exit(1);
    }

    Ok(())
}

fn suggest_command(error_msg: &str) {
    use colors::{primary as color_primary, success as color_success};
    use strsim::jaro_winkler;

    const COMMANDS: &[&str] = &["generate", "config", "completion", "update", "status"];

    // Extract the unrecognized command from error message
    if let Some(start) = error_msg.find('\'') {
        if let Some(end) = error_msg[start + 1..].find('\'') {
            let bad_command = &error_msg[start + 1..start + 1 + end];

            // Find the most similar command
            let mut best_match = "";
            let mut best_score = 0.0;

            for &cmd in COMMANDS {
                let score = jaro_winkler(bad_command, cmd);
                if score > best_score && score > 0.6 {
                    best_score = score;
                    best_match = cmd;
                }
            }

            if !best_match.is_empty() {
                eprintln!();
                eprintln!("{}", color_primary(format!("Did you mean '{best_match}'?")));
                eprintln!();
                eprintln!("Available commands:");
                for &cmd in COMMANDS {
                    eprintln!("  {}", color_success(cmd));
                }
            }
        }
    }
}

async fn run_legacy(args: LegacyArgs) -> Result<()> {
    use ai::GeminiClient;
    use colors::{success as color_success, warning as color_warning};
    use config::Config;
    use git::GitRepo;
    use log::info;
    use utils::copy_to_clipboard;

    let start_time = std::time::Instant::now();

    // Handle update command
    if args.update {
        return handle_update().await;
    }

    // Load configuration
    log::debug!("Loading configuration...");
    let config = Config::load(&args)?;
    config.validate()?;
    log::debug!("Configuration loaded and validated successfully");

    // Check for updates (non-blocking)
    if let Ok(update_available) = update::check_for_updates().await {
        if update_available {
            info!("{}", color_warning("A new version of Commi is available!"));
            info!(
                "{}",
                color_warning("Run 'commi --update' to update to the latest version.")
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
    println!("{commit_message}");

    // Handle commit operation
    if args.commit {
        if !repo.has_staged_changes()? {
            anyhow::bail!("No staged changes to commit. Use 'git add' to stage changes.");
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

async fn handle_update() -> Result<()> {
    update::update_binary().await
}
