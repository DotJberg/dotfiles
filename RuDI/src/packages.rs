use std::fs;

use anyhow::{Result, bail};

use crate::shell;
use crate::ui;

const PACKAGES: &[&str] = &[
    "zen-browser-bin",
    "rofi",
    "waybar",
    "stow",
    "otf-font-awesome",
    "ttf-jetbrains-mono-nerd",
    "ghostty",
    "neovim",
    "fish",
    "starship",
    "eza",
    "ripgrep",
    "glint",
    "hyprshot",
    "hyprlock",
    "playerctl",
    "pavucontrol",
    "brightnessctl",
    "bluez",
    "bluez-utils",
    "blueman",
    "unzip",
    "fzf",
];

const YAY_CLONE_DIR: &str = "/tmp/yay-install";

pub fn bootstrap_yay() -> Result<()> {
    ui::section("AUR Helper");

    if shell::binary_exists("yay") {
        ui::skipped("yay already installed");
        return Ok(());
    }

    ui::action("Installing yay from AUR...");

    // Clean up any previous failed attempt
    let _ = fs::remove_dir_all(YAY_CLONE_DIR);

    shell::run(
        "git",
        &["clone", "https://aur.archlinux.org/yay.git", YAY_CLONE_DIR],
    )?;

    let sp = ui::spinner("Building yay (makepkg -si)...");
    let result = shell::run_in_dir(YAY_CLONE_DIR, "makepkg", &["-si", "--noconfirm"]);
    sp.finish_and_clear();

    let _ = fs::remove_dir_all(YAY_CLONE_DIR);

    if let Err(e) = result {
        bail!(
            "Failed to build yay: {e}\nCannot continue without yay — all packages are installed through it."
        );
    }

    if !shell::binary_exists("yay") {
        bail!(
            "yay was not found in PATH after install.\nCannot continue without yay — all packages are installed through it."
        );
    }

    ui::success("yay installed");
    Ok(())
}

pub fn install_packages() -> Result<(Vec<String>, Vec<String>, Vec<String>)> {
    ui::section("Packages");

    let mut needed: Vec<&str> = Vec::new();
    let mut already: Vec<String> = Vec::new();

    let pb = ui::progress_bar(PACKAGES.len() as u64, "Checking packages");

    for &pkg in PACKAGES {
        if shell::package_installed(pkg) {
            already.push(pkg.to_string());
        } else {
            needed.push(pkg);
        }
        pb.inc(1);
    }
    pb.finish_and_clear();

    for pkg in &already {
        ui::skipped(&format!("{pkg} already installed"));
    }

    if needed.is_empty() {
        ui::info("All packages already installed");
        return Ok((vec![], already, vec![]));
    }

    ui::action(&format!("Installing {} packages via yay...", needed.len()));

    let mut args = vec!["-S", "--needed", "--noconfirm"];
    args.extend_from_slice(&needed);

    let sp = ui::spinner(&format!("Installing {} packages via yay...", needed.len()));
    let result = shell::run("yay", &args);
    sp.finish_and_clear();

    let mut installed = Vec::new();
    let mut failed_pkgs = Vec::new();

    match result {
        Ok(()) => {
            for &pkg in &needed {
                if shell::package_installed(pkg) {
                    ui::success(&format!("{pkg} installed"));
                    installed.push(pkg.to_string());
                } else {
                    ui::failed(&format!("{pkg} failed to install"));
                    failed_pkgs.push(pkg.to_string());
                }
            }
        }
        Err(e) => {
            ui::failed(&format!("yay exited with error: {e}"));
            for &pkg in &needed {
                if shell::package_installed(pkg) {
                    installed.push(pkg.to_string());
                } else {
                    failed_pkgs.push(pkg.to_string());
                }
            }
        }
    }

    Ok((installed, already, failed_pkgs))
}

pub fn uninstall_packages() -> Result<()> {
    ui::section("Uninstall Packages");

    let installed: Vec<&str> = PACKAGES
        .iter()
        .copied()
        .filter(|pkg| shell::package_installed(pkg))
        .collect();

    if installed.is_empty() {
        ui::info("No managed packages are installed");
        return Ok(());
    }

    ui::action(&format!("Removing {} packages...", installed.len()));

    let mut args = vec!["-Rns", "--noconfirm"];
    args.extend_from_slice(&installed);

    let sp = ui::spinner(&format!("Removing {} packages...", installed.len()));
    let result = shell::run("yay", &args);
    sp.finish_and_clear();

    match result {
        Ok(()) => ui::success("Packages removed"),
        Err(e) => ui::failed(&format!("Package removal failed: {e}")),
    }

    Ok(())
}

pub fn set_default_shell() -> Result<()> {
    ui::section("Default Shell");

    let current_shell = std::env::var("SHELL").unwrap_or_default();
    if current_shell == "/usr/bin/fish" {
        ui::skipped("fish is already the default shell");
        return Ok(());
    }

    ui::action("Setting default shell to fish...");
    // Use the sudo session acquired at install time so this works non-interactively
    // (bare `chsh` would prompt for the user's password on its own tty).
    let user = std::env::var("USER").unwrap_or_default();
    match shell::run("sudo", &["chsh", "-s", "/usr/bin/fish", user.as_str()]) {
        Ok(()) => ui::success("Default shell set to fish"),
        Err(e) => ui::failed(&format!("Failed to set shell: {e}")),
    }

    Ok(())
}
