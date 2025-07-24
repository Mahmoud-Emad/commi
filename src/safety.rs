use anyhow::Result;
use dialoguer::{theme::ColorfulTheme, Confirm, Input};
use log::{info, warn};
use std::path::Path;

use crate::colors::{
    error as color_error, key as color_key, success as color_success, warning as color_warning,
};

/// Types of dangerous operations that require confirmation
#[derive(Debug, Clone, PartialEq)]
pub enum DangerousOperation {
    /// Committing changes to git
    GitCommit {
        files_count: usize,
        has_staged: bool,
        has_unstaged: bool,
    },
    /// Resetting configuration
    ConfigReset {
        all_config: bool,
        key: Option<String>,
    },
    /// Updating the binary
    BinaryUpdate {
        current_version: String,
        new_version: String,
        force: bool,
    },
}

/// Safety configuration
#[derive(Debug, Clone)]
pub struct SafetyConfig {
    /// Skip all confirmations (dangerous!)
    pub skip_all: bool,
    /// Show detailed warnings
    pub verbose_warnings: bool,
}

impl Default for SafetyConfig {
    fn default() -> Self {
        Self {
            skip_all: false,
            verbose_warnings: true,
        }
    }
}

/// Safety checker for dangerous operations
pub struct SafetyChecker {
    config: SafetyConfig,
}

impl SafetyChecker {
    pub fn new(config: SafetyConfig) -> Self {
        Self { config }
    }

    pub fn with_default() -> Self {
        Self::new(SafetyConfig::default())
    }

    /// Check if an operation is safe to proceed
    pub fn check_operation(&self, operation: &DangerousOperation) -> Result<bool> {
        if self.config.skip_all {
            warn!("Safety checks disabled - proceeding without confirmation");
            return Ok(true);
        }

        match operation {
            DangerousOperation::GitCommit {
                files_count,
                has_staged,
                has_unstaged,
            } => self.check_git_commit(*files_count, *has_staged, *has_unstaged),
            DangerousOperation::ConfigReset { all_config, key } => {
                self.check_config_reset(*all_config, key.as_deref())
            }
            DangerousOperation::BinaryUpdate {
                current_version,
                new_version,
                force,
            } => self.check_binary_update(current_version, new_version, *force),
        }
    }

    fn check_git_commit(
        &self,
        files_count: usize,
        has_staged: bool,
        has_unstaged: bool,
    ) -> Result<bool> {
        if self.config.verbose_warnings {
            info!("Commit Summary:");
            info!("Files affected: {files_count}");
            info!("Staged changes: {}", if has_staged { "Yes" } else { "No" });
            info!(
                "Unstaged changes: {}",
                if has_unstaged { "Yes" } else { "No" }
            );

            if has_unstaged {
                warn!(
                    "{}",
                    color_warning("You have unstaged changes that will NOT be committed")
                );
            }
        }

        let prompt = if has_unstaged {
            "Commit staged changes only? (unstaged changes will remain)"
        } else {
            "Commit these changes?"
        };

        let confirmed = Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt(prompt)
            .default(false)
            .interact()?;

        if confirmed {
            info!("{}", color_success("Commit confirmed"));
        } else {
            info!("Commit cancelled");
        }

        Ok(confirmed)
    }

    fn check_config_reset(&self, all_config: bool, key: Option<&str>) -> Result<bool> {
        let (prompt, warning) = if all_config {
            (
                "Reset ALL configuration to defaults?".to_string(),
                "This will permanently delete all your custom settings!".to_string(),
            )
        } else if let Some(key) = key {
            (
                format!("Reset configuration key '{}'?", color_key(key)),
                "This will permanently delete this setting!".to_string(),
            )
        } else {
            return Ok(false);
        };

        if self.config.verbose_warnings {
            warn!("{}", color_warning(&format!(" {warning}")));
            info!("TIP: You can backup your config before resetting");
        }

        // Double confirmation for resetting all config
        if all_config {
            let first_confirm = Confirm::with_theme(&ColorfulTheme::default())
                .with_prompt("Are you sure you want to reset ALL configuration?")
                .default(false)
                .interact()?;

            if !first_confirm {
                info!("Reset cancelled");
                return Ok(false);
            }

            // Type confirmation for extra safety
            let confirmation_text = "RESET ALL";
            let typed_confirmation: String = Input::with_theme(&ColorfulTheme::default())
                .with_prompt(format!("Type '{confirmation_text}' to confirm"))
                .interact_text()?;

            if typed_confirmation != confirmation_text {
                warn!(
                    "{}",
                    color_error("ERROR: Confirmation text doesn't match. Reset cancelled.")
                );
                return Ok(false);
            }
        }

        let confirmed = Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt(&prompt)
            .default(false)
            .interact()?;

        if confirmed {
            info!("{}", color_success("Reset confirmed"));
        } else {
            info!("Reset cancelled");
        }

        Ok(confirmed)
    }

    fn check_binary_update(
        &self,
        current_version: &str,
        new_version: &str,
        force: bool,
    ) -> Result<bool> {
        if self.config.verbose_warnings {
            info!("Update Summary:");
            info!("   Current version: {current_version}");
            info!("   New version: {new_version}");
            info!("   Force update: {}", if force { "Yes" } else { "No" });
        }

        let prompt = if force {
            "WARNING: Force update commi binary? (may overwrite local changes)"
        } else {
            "Update commi to the latest version?"
        };

        if force && self.config.verbose_warnings {
            warn!(
                "{}",
                color_warning("WARNING: Force update may overwrite any local modifications")
            );
        }

        let confirmed = Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt(prompt)
            .default(!force) // Default to false for force updates
            .interact()?;

        if confirmed {
            info!("{}", color_success("Update confirmed"));
        } else {
            info!("Update cancelled");
        }

        Ok(confirmed)
    }
}

/// Convenience function to check a dangerous operation
pub fn confirm_dangerous_operation(operation: DangerousOperation) -> Result<bool> {
    let checker = SafetyChecker::with_default();
    checker.check_operation(&operation)
}

/// Check if a path contains important files that shouldn't be modified
pub fn check_important_files(path: &Path) -> Vec<String> {
    let important_patterns = vec![
        ".git/",
        "Cargo.toml",
        "package.json",
        "requirements.txt",
        "go.mod",
        "pom.xml",
        "build.gradle",
        "Makefile",
        "Dockerfile",
        ".env",
        "config.yml",
        "config.yaml",
        "config.json",
    ];

    let mut found_files = Vec::new();

    if let Ok(entries) = std::fs::read_dir(path) {
        for entry in entries.flatten() {
            let file_name = entry.file_name().to_string_lossy().to_string();
            for pattern in &important_patterns {
                if file_name.contains(pattern) {
                    found_files.push(file_name.clone());
                    break;
                }
            }
        }
    }

    found_files
}

/// Show a warning about important files in the directory
pub fn warn_about_important_files(path: &Path) -> Result<()> {
    let important_files = check_important_files(path);

    if !important_files.is_empty() {
        warn!(
            "{}",
            color_warning("WARNING: Important files detected in this directory:")
        );
        for file in important_files {
            warn!("   FILE: {file}");
        }
        warn!(
            "{}",
            color_warning("TIP: Make sure you understand the impact of your changes")
        );
        println!();
    }

    Ok(())
}
