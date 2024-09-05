// src/main.rs
mod yaml;
mod json;
mod sys_ops;
mod install_package;

use std::env;
use std::process;
use std::path::Path;
use crate::yaml::{Yaml, open_yaml};
use crate::json::{Json, open_json};
use crate::sys_ops::{is_running_as_root, rename_computer};
use crate::install_package::{install_package_with_apt, install_package_with_dnf, install_package_with_homebrew, install_package_with_pacman, install_package_with_winget};

pub enum FileFormat {
    Yaml,
    Json,
    Unknown,
}

pub fn determine_file_format(file_path: &str) -> FileFormat {
    let path = Path::new(file_path);
    match path.extension().and_then(|ext| ext.to_str()) {
        Some("yaml") | Some("yml") => FileFormat::Yaml,
        Some("json") => FileFormat::Json,
        _ => FileFormat::Unknown,
    }
}

fn get_str_value_yaml(yaml: &Yaml, key: &str) -> Option<String> {
    yaml.get(key).and_then(|v| v.as_str().map(|s| s.to_string()))
}

fn get_str_value_json(json: &Json, key: &str) -> Option<String> {
    json.get(key).and_then(|v| v.as_str().map(|s| s.to_string()))
}

fn extract_packages_yaml(yaml: &Yaml) -> Result<Vec<String>, &'static str> {
    yaml.get("packages")
        .and_then(|v| v.as_sequence())
        .map(|seq| seq.iter().filter_map(|item| item.as_str().map(|s| s.to_string())).collect())
        .ok_or("Missing or invalid 'packages'")
}

fn extract_packages_json(json: &Json) -> Result<Vec<String>, &'static str> {
    json.get("packages")
        .and_then(|v| v.as_array())
        .map(|arr| arr.iter().filter_map(|item| item.as_str().map(|s| s.to_string())).collect())
        .ok_or("Missing or invalid 'packages'")
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    if !is_running_as_root() {
        println!("This operation requires root or administrative privileges.");
        process::exit(1);
    }

    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: SetupSmith /path/to/file.yaml");
        process::exit(1);
    }

    let file_path = &args[1];
    let file_format = determine_file_format(file_path);
    let (hostname, package_manager, packages) = match file_format {
        FileFormat::Yaml => {
            let yaml = open_yaml(file_path)?;
            (
                get_str_value_yaml(&yaml, "hostname").ok_or("Missing or invalid 'hostname'")?,
                get_str_value_yaml(&yaml, "package_manager").ok_or("Missing or invalid 'package_manager'")?,
                extract_packages_yaml(&yaml)?,
            )
        },
        FileFormat::Json => {
            let json = open_json(file_path)?;
            (
                get_str_value_json(&json, "hostname").ok_or("Missing or invalid 'hostname'")?,
                get_str_value_json(&json, "package_manager").ok_or("Missing or invalid 'package_manager'")?,
                extract_packages_json(&json)?,
            )
        },
        _ => {
            eprintln!("Unsupported file format.");
            process::exit(1);
        }
    };

    match rename_computer(&hostname) {
        Ok(()) => println!("Successfully renamed the computer to '{}'.", hostname),
        Err(e) => {
            eprintln!("Error renaming computer: {}", e);
            process::exit(1);
        }
    }

    let mut install_results = Vec::new();
    for package_name in packages {
        let result = match package_manager.as_str() {
            "apt" => install_package_with_apt(&package_name),
            "dnf" => install_package_with_dnf(&package_name),
            "pacman" => install_package_with_pacman(&package_name),
            "homebrew" => install_package_with_homebrew(&package_name),
            "winget" => install_package_with_winget(&package_name),
            _ => {
                eprintln!("Unsupported package manager: {}", package_manager);
                continue;
            }
        };

        match result {
            Ok(()) => install_results.push(format!("Successfully installed '{}'", package_name)),
            Err(e) => install_results.push(format!("Failed to install '{}': {}", package_name, e)),
        }
    }

    println!("\n--------------------------- Summary ------------------------------------------");
    println!(" > Hostname changed to: '{}'", hostname);
    println!(" > Package Manager used: '{}'", package_manager);
    println!(" > Installed packages:");
    for result in install_results {
        println!("    - {}", result);
    }
    println!("------------------------------------------------------------------------------");

    Ok(())
}
