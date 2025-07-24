use anyhow::{Context, Result};
use arboard::Clipboard;

use crate::colors::{primary as color_primary, success as color_success};

pub fn print_header() {
    let header = r#"
   ______                          _ 
  / ____/___  ____ ___  ____ ___  (_)
 / /   / __ \/ __ `__ \/ __ `__ \/ / 
/ /___/ /_/ / / / / / / / / / / / /  
\____/\____/_/ /_/ /_/_/ /_/ /_/_/   
                                    
"#;

    println!("{}", color_primary(header));
    println!(
        "{}",
        color_success("Welcome to Commi, an AI-powered Git commit message generator tool!")
    );
    println!("{}", color_success("This tool uses Google's Gemini AI to suggest meaningful commit messages based on your git diffs."));
    println!(
        "{}",
        color_success("For more details, visit: https://github.com/Mahmoud-Emad/commi")
    );
    println!();
}

pub fn copy_to_clipboard(text: &str) -> Result<()> {
    let mut clipboard = Clipboard::new().context("Failed to access clipboard")?;

    clipboard
        .set_text(text)
        .context("Failed to copy text to clipboard")?;

    Ok(())
}
