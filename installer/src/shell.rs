use std::process::{Command, Stdio};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::Duration;

use anyhow::{Context, Result, bail};

/// Run a command, inheriting stdin/stdout/stderr so the user sees output (e.g. sudo prompts).
/// Returns Ok(()) on success, Err on non-zero exit.
pub fn run(program: &str, args: &[&str]) -> Result<()> {
    let status = Command::new(program)
        .args(args)
        .status()
        .with_context(|| format!("failed to execute `{program}`"))?;

    if !status.success() {
        bail!("`{program}` exited with {status}");
    }
    Ok(())
}

/// Run a command in a specific directory.
pub fn run_in_dir(dir: &str, program: &str, args: &[&str]) -> Result<()> {
    let status = Command::new(program)
        .args(args)
        .current_dir(dir)
        .status()
        .with_context(|| format!("failed to execute `{program}` in {dir}"))?;

    if !status.success() {
        bail!("`{program}` exited with {status}");
    }
    Ok(())
}

/// Run a command and capture its stdout, trimming trailing whitespace.
pub fn run_output(program: &str, args: &[&str]) -> Result<String> {
    let output = Command::new(program)
        .args(args)
        .output()
        .with_context(|| format!("failed to execute `{program}`"))?;

    if !output.status.success() {
        bail!("`{program}` exited with {}", output.status);
    }
    Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

/// Prompt for sudo once and keep the session alive in a background thread.
/// The credentials are cached by sudo itself — we never touch the password.
/// Drop the returned guard to stop the refresh thread.
pub fn acquire_sudo() -> Result<SudoGuard> {
    let status = Command::new("sudo")
        .args(["-v"])
        .status()
        .context("failed to execute sudo")?;

    if !status.success() {
        bail!("sudo authentication failed");
    }

    let running = Arc::new(AtomicBool::new(true));
    let flag = running.clone();

    let handle = thread::spawn(move || {
        while flag.load(Ordering::Relaxed) {
            thread::sleep(Duration::from_secs(1));
            if flag.load(Ordering::Relaxed) {
                let _ = Command::new("sudo")
                    .args(["-v"])
                    .stdout(Stdio::null())
                    .stderr(Stdio::null())
                    .status();
            }
        }
    });

    Ok(SudoGuard {
        running,
        handle: Some(handle),
    })
}

pub struct SudoGuard {
    running: Arc<AtomicBool>,
    handle: Option<thread::JoinHandle<()>>,
}

impl Drop for SudoGuard {
    fn drop(&mut self) {
        self.running.store(false, Ordering::Relaxed);
        if let Some(handle) = self.handle.take() {
            let _ = handle.join();
        }
    }
}

/// Check if a binary is on PATH.
pub fn binary_exists(name: &str) -> bool {
    which::which(name).is_ok()
}

/// Check if a package is installed via yay (queries the local package database).
pub fn package_installed(pkg: &str) -> bool {
    Command::new("yay")
        .args(["-Qi", pkg])
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status()
        .is_ok_and(|s| s.success())
}
