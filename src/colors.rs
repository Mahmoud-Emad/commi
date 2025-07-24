use console::{style, StyledObject};
use std::io::{self, IsTerminal};
use std::sync::OnceLock;

/// Color configuration for the application
pub struct ColorConfig {
    pub enabled: bool,
}

impl ColorConfig {
    pub fn new(force_disable: bool) -> Self {
        let enabled = !force_disable && io::stdout().is_terminal();
        Self { enabled }
    }
}

/// Theme colors for consistent styling across the application
pub struct Theme {
    config: ColorConfig,
}

impl Theme {
    pub fn new(config: ColorConfig) -> Self {
        Self { config }
    }

    /// Primary accent color for headers and important information
    pub fn primary<T: std::fmt::Display>(&self, text: T) -> StyledObject<T> {
        if self.config.enabled {
            style(text).cyan().bold()
        } else {
            style(text)
        }
    }

    /// Success color for positive outcomes
    pub fn success<T: std::fmt::Display>(&self, text: T) -> StyledObject<T> {
        if self.config.enabled {
            style(text).green()
        } else {
            style(text)
        }
    }

    /// Warning color for cautionary messages
    pub fn warning<T: std::fmt::Display>(&self, text: T) -> StyledObject<T> {
        if self.config.enabled {
            style(text).yellow()
        } else {
            style(text)
        }
    }

    /// Error color for error messages
    pub fn error<T: std::fmt::Display>(&self, text: T) -> StyledObject<T> {
        if self.config.enabled {
            style(text).red()
        } else {
            style(text)
        }
    }

    /// Value color for configuration values, etc.
    pub fn value<T: std::fmt::Display>(&self, text: T) -> StyledObject<T> {
        if self.config.enabled {
            style(text).green()
        } else {
            style(text)
        }
    }

    /// Key color for configuration keys, etc.
    pub fn key<T: std::fmt::Display>(&self, text: T) -> StyledObject<T> {
        if self.config.enabled {
            style(text).cyan()
        } else {
            style(text)
        }
    }
}

/// Global theme instance
static GLOBAL_THEME: OnceLock<Theme> = OnceLock::new();

/// Initialize the global theme
pub fn init_theme(no_color: bool) {
    let config = ColorConfig::new(no_color);
    let theme = Theme::new(config);
    let _ = GLOBAL_THEME.set(theme);
}

/// Get the global theme instance
pub fn theme() -> &'static Theme {
    GLOBAL_THEME
        .get()
        .expect("Theme not initialized. Call init_theme() first.")
}

/// Convenience functions for common styling
pub fn success<T: std::fmt::Display>(text: T) -> StyledObject<T> {
    theme().success(text)
}

pub fn error<T: std::fmt::Display>(text: T) -> StyledObject<T> {
    theme().error(text)
}

pub fn warning<T: std::fmt::Display>(text: T) -> StyledObject<T> {
    theme().warning(text)
}

pub fn primary<T: std::fmt::Display>(text: T) -> StyledObject<T> {
    theme().primary(text)
}

pub fn key<T: std::fmt::Display>(text: T) -> StyledObject<T> {
    theme().key(text)
}

pub fn value<T: std::fmt::Display>(text: T) -> StyledObject<T> {
    theme().value(text)
}
