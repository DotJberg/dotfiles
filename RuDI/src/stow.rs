use anyhow::{Context, Result};

use crate::shell;
use crate::ui;

const STOW_PACKAGES: &[&str] = &["fish", "ghostty", "hypr", "nvim", "rofi", "waybar"];

/// Discover the dotfiles repo root via git.
fn repo_root() -> Result<String> {
    shell::run_output("git", &["rev-parse", "--show-toplevel"])
        .context("failed to find git repo root — are you inside the dotfiles repo?")
}

/// Stow all dotfile packages.
pub fn stow_all() -> Result<()> {
    ui::section("Stow Dotfiles");

    let root = repo_root()?;

    let sp = ui::spinner("Stowing dotfiles...");

    let mut stowed = Vec::new();
    let mut failed = Vec::new();

    for &pkg in STOW_PACKAGES {
        match shell::run_in_dir(&root, "stow", &[pkg]) {
            Ok(()) => stowed.push(pkg),
            Err(_) => failed.push(pkg),
        }
    }

    sp.finish_and_clear();

    if !stowed.is_empty() {
        ui::success(&format!("Stowed: {}", stowed.join(", ")));
    }
    if !failed.is_empty() {
        ui::failed(&format!("Failed: {}", failed.join(", ")));
        ui::info("Hint: check for existing files that conflict with symlinks");
    }

    Ok(())
}

/// Unstow all dotfile packages.
pub fn unstow_all() -> Result<()> {
    ui::section("Unstow Dotfiles");

    let root = repo_root()?;

    let sp = ui::spinner("Unstowing dotfiles...");

    let mut unstowed = Vec::new();
    let mut failed = Vec::new();

    for &pkg in STOW_PACKAGES {
        match shell::run_in_dir(&root, "stow", &["-D", pkg]) {
            Ok(()) => unstowed.push(pkg),
            Err(_) => failed.push(pkg),
        }
    }

    sp.finish_and_clear();

    if !unstowed.is_empty() {
        ui::success(&format!("Unstowed: {}", unstowed.join(", ")));
    }
    if !failed.is_empty() {
        ui::failed(&format!("Failed: {}", failed.join(", ")));
    }

    Ok(())
}
