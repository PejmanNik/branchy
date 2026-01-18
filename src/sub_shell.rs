use anyhow::{Context, Error};
use std::{path::Path, process::Command};

pub fn open_sub_shell(path: &Path) -> Result<(), Error> {
    Command::new(get_shell()?)
        .current_dir(path)
        .status()
        .context("Failed to spawn shell")?;

    Ok(())
}

fn get_shell() -> Result<String, Error> {
    if let Ok(shell) = std::env::var("SHELL") {
        return Ok(shell);
    }

    if cfg!(windows) {
        let shell = find_windows_shell().ok_or_else(|| {
            anyhow::anyhow!("No PowerShell found (tried pwsh.exe, powershell.exe)")
        })?;
        Ok(shell)
    } else {
        let fallback = if cfg!(target_os = "macos") {
            "/bin/zsh"
        } else {
            "/bin/sh"
        };

        if !Path::new(fallback).exists() {
            return Err(anyhow::anyhow!(
                "No shell found: SHELL not set and {} doesn't exist",
                fallback
            ));
        }

        Ok(fallback.to_string())
    }
}

#[cfg(windows)]
fn find_windows_shell() -> Option<String> {
    use std::process::Command;

    let candidates = ["pwsh.exe", "powershell.exe"];

    let shell = candidates
        .iter()
        .find(|&&shell| {
            Command::new("where")
                .arg(shell)
                .output()
                .map(|out| out.status.success())
                .unwrap_or(false)
        })
        .map(|s| s.to_string());

    if let Some(ref shell_path) = shell {
        unsafe {
            std::env::set_var("SHELL", shell_path);
        }
    }

    return shell;
}

#[cfg(not(windows))]
fn find_windows_shell() -> Option<String> {
    None
}
