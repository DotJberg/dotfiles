mod packages;
mod shell;
mod stow;
mod ui;

use std::io::{self, Write};

use anyhow::Result;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "rudi", about = "RuDI — Rust Dotfiles Installer")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Install everything (default)
    Install,
    Clean {
        #[arg(long)]
        uninstall: bool,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    ui::banner();

    match cli.command.unwrap_or(Commands::Install) {
        Commands::Install => install()?,
        Commands::Clean { uninstall } => clean(uninstall)?,
    }

    Ok(())
}

fn install() -> Result<()> {
    ui::section("Authentication");
    ui::action("Requesting sudo access (needed for package installation)...");
    let _sudo = shell::acquire_sudo()?;
    ui::success("sudo session acquired");

    packages::bootstrap_yay()?;

    let (installed, skipped, failed) = packages::install_packages()?;

    packages::set_default_shell()?;

    stow::stow_all()?;

    ui::summary(&installed, &skipped, &failed);

    Ok(())
}

fn clean(uninstall: bool) -> Result<()> {
    stow::unstow_all()?;

    if uninstall {
        print!("\n  Are you sure you want to uninstall all packages? [y/N] ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        if input.trim().eq_ignore_ascii_case("y") {
            ui::action("Requesting sudo access (needed for package removal)...");
            let _sudo = shell::acquire_sudo()?;
            ui::success("sudo session acquired");
            packages::uninstall_packages()?;
        } else {
            ui::info("Package removal cancelled");
        }
    }

    println!();
    ui::success("Clean complete!");
    println!();

    Ok(())
}
