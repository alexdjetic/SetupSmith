use std::process::Command;
use std::io::{self, ErrorKind};

#[cfg(unix)]
fn is_running_as_root() -> bool {
    // On Unix-like systems, check if the effective user ID is 0 (root)
    nix::unistd::geteuid().is_root()
}

#[cfg(windows)]
fn is_running_as_root() -> bool {
    // On Windows, use the `net session` command to check if the user is an administrator
    let output = Command::new("net")
        .arg("session")
        .output()
        .expect("Failed to check if running as administrator");

    output.status.success()
}

pub fn rename_computer(new_name: &str) -> io::Result<()> {
    // Check if running with root or admin privileges
    if !is_running_as_root() {
        return Err(io::Error::new(
            ErrorKind::PermissionDenied,
            "This operation requires root or administrative privileges.",
        ));
    }

    #[cfg(target_os = "linux")]
    {
        rename_computer_linux(new_name)
    }

    #[cfg(target_os = "macos")]
    {
        rename_computer_macos(new_name)
    }

    #[cfg(target_os = "windows")]
    {
        rename_computer_windows(new_name)
    }
}

// Linux: Renaming the computer
#[cfg(target_os = "linux")]
fn rename_computer_linux(new_name: &str) -> io::Result<()> {
    let status = Command::new("hostnamectl")
        .arg("set-hostname")
        .arg(new_name)
        .status()?;

    if status.success() {
        println!("Successfully renamed the computer to '{}'", new_name);
        Ok(())
    } else {
        Err(io::Error::new(ErrorKind::Other, "Failed to rename computer on Linux"))
    }
}

// macOS: Renaming the computer
#[cfg(target_os = "macos")]
fn rename_computer_macos(new_name: &str) -> io::Result<()> {
    let status = Command::new("scutil")
        .arg("--set")
        .arg("ComputerName")
        .arg(new_name)
        .status()?;

    if status.success() {
        println!("Successfully renamed the computer to '{}'", new_name);
        Ok(())
    } else {
        Err(io::Error::new(ErrorKind::Other, "Failed to rename computer on macOS"))
    }
}

// Windows: Renaming the computer
#[cfg(target_os = "windows")]
fn rename_computer_windows(new_name: &str) -> io::Result<()> {
    let status = Command::new("powershell")
        .arg("-Command")
        .arg(format!("Rename-Computer -NewName '{}' -Force", new_name))
        .status()?;

    if status.success() {
        println!("Successfully renamed the computer to '{}'", new_name);
        Ok(())
    } else {
        Err(io::Error::new(ErrorKind::Other, "Failed to rename computer on Windows"))
    }
}
