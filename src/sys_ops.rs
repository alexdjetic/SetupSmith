use std::process::Command;
use std::io::{self, ErrorKind};

#[cfg(unix)]
use nix::unistd::geteuid;


pub fn is_running_as_root() -> bool {
    #[cfg(unix)]
    {
        is_running_as_root_unix()
    }

    #[cfg(windows)]
    {
        is_running_as_root_win()
    }
}

#[cfg(unix)]
fn is_running_as_root_unix() -> bool {
    // On Unix-like systems, check if the effective user ID is 0 (root)
    geteuid().is_root()
}

#[cfg(windows)]
fn is_running_as_root_win() -> bool {
    use whoami::PrivilegeLevel;

    // Check if the user has administrative privileges
    match whoami::privilege_level() {
        PrivilegeLevel::Administrator => true,
        _ => false,
    }
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
        Ok(())
    } else {
        Err(io::Error::new(ErrorKind::Other, "Failed to rename computer on Windows"))
    }
}
