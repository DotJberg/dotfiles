use colored::Colorize;
use indicatif::{ProgressBar, ProgressStyle};

pub fn banner() {
    let box_text = [
        "  ┌─────────────────────────────┐",
        "  │                             │",
        "  │    RuDI                     │",
        "  │    Rust Dotfiles Installer  │",
        "  │                             │",
        "  │    Arch Linux + Hyprland    │",
        "  │                             │",
        "  └─────────────────────────────┘",
    ];
    println!();
    for line in &box_text {
        println!("{}", line.bold().cyan());
    }
    println!();
}

pub fn section(title: &str) {
    let rule_len = 45usize.saturating_sub(title.len() + 5);
    let rule = "━".repeat(rule_len);
    println!("\n{}", format!("━━━ {title} {rule}").bold().cyan());
}

pub fn success(msg: &str) {
    println!("  {} {msg}", "✓".bold().green());
}

pub fn skipped(msg: &str) {
    println!("  {} {msg}", "⊘".bold().yellow());
}

pub fn failed(msg: &str) {
    println!("  {} {msg}", "✗".bold().red());
}

pub fn action(msg: &str) {
    println!("  {} {msg}", "→".bold().blue());
}

pub fn info(msg: &str) {
    println!("  {} {msg}", "ℹ".dimmed());
}

pub fn progress_bar(len: u64, msg: &str) -> ProgressBar {
    let pb = ProgressBar::new(len);
    pb.set_style(
        ProgressStyle::default_bar()
            .template(&format!(
                "  {{spinner:.cyan}} {msg}  [{{bar:30.cyan/dim}}]  {{pos}}/{{len}}"
            ))
            .unwrap()
            .progress_chars("██░"),
    );
    pb
}

pub fn spinner(msg: &str) -> ProgressBar {
    let sp = ProgressBar::new_spinner();
    sp.set_style(
        ProgressStyle::default_spinner()
            .template(&format!("  {{spinner:.cyan}} {msg}"))
            .unwrap(),
    );
    sp.enable_steady_tick(std::time::Duration::from_millis(80));
    sp
}

pub fn summary(installed: &[String], skipped_pkgs: &[String], failed_pkgs: &[String]) {
    section("Summary");
    println!();

    let installed_list = if installed.is_empty() {
        "none".to_string()
    } else {
        installed.join(", ")
    };
    let skipped_list = if skipped_pkgs.is_empty() {
        "none".to_string()
    } else {
        skipped_pkgs.join(", ")
    };
    let failed_list = if failed_pkgs.is_empty() {
        "none".to_string()
    } else {
        failed_pkgs.join(", ")
    };

    println!(
        "  {}  Installed ({:>2}):  {installed_list}",
        "✓".bold().green(),
        installed.len()
    );
    println!(
        "  {}  Skipped  ({:>2}):  {skipped_list}",
        "⊘".bold().yellow(),
        skipped_pkgs.len()
    );
    println!(
        "  {}  Failed   ({:>2}):  {failed_list}",
        "✗".bold().red(),
        failed_pkgs.len()
    );

    section("Notes");
    println!();
    info("Log out and back in for fish shell");
    info("Open neovim — plugins auto-install on first launch");
    info("Rofi themes: https://github.com/lr-tech/rofi-themes-collection");
    info("Enable bluetooth: sudo systemctl enable --now bluetooth");
    println!();
    success("All done!");
    println!();
}
