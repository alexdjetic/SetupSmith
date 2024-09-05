use std::process;
mod yaml;
mod json;
mod sys_ops;
mod install_package;

use yaml::{Yaml, open_yaml};
use json::{Json, open_json};
use sys_ops::{is_running_as_root, rename_computer};
use install_package::{install_package_with_apt,
                      install_package_with_dnf,
                      install_package_with_homebrew,
                      install_package_with_pacman,
                      install_package_with_winget};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Check if the program is running with root or administrative privileges
    if !is_running_as_root() {
        println!("This operation requires root or administrative privileges.");
        process::exit(1); // Exit with code 1 to indicate lack of privileges
    }

    // Load the YAML configuration
    let yaml = open_yaml("example/vm1.yaml")?;
    let hostname = yaml.get("hostname")
        .and_then(|v| v.as_str())
        .ok_or("Missing or invalid 'hostname' in YAML")?;

    let package_manager = yaml.get("package_manager")
        .and_then(|v| v.as_str())
        .ok_or("Missing or invalid 'package_manager' in YAML")?;

    let packages = yaml.get("packages")
        .and_then(|v| v.as_sequence())
        .ok_or("Missing or invalid 'packages' in YAML")?;

    // Rename the computer
    match rename_computer(hostname) {
        Ok(()) => {
            println!("Successfully renamed the computer to '{}'.", hostname);
        }
        Err(e) => {
            eprintln!("Error renaming computer: {}", e);
            process::exit(1); // Exit with code 1 to indicate failure
        }
    }

    // Install packages based on the package manager
    let mut install_results = Vec::new();
    for package in packages {
        let package_name = package.as_str().ok_or("Package name is not a string")?;
        let result = match package_manager {
            "apt" => install_package_with_apt(package_name),
            "dnf" => install_package_with_dnf(package_name),
            "pacman" => install_package_with_pacman(package_name),
            "homebrew" => install_package_with_homebrew(package_name),
            "winget" => install_package_with_winget(package_name),
            _ => {
                eprintln!("Unsupported package manager: {}", package_manager);
                continue;
            }
        };

        // Collect the results of the installation
        match result {
            Ok(()) => install_results.push(format!("{}", package_name)),
            Err(e) => install_results.push(format!("Failed to install '{}': {}", package_name, e)),
        }
    }

    // Print a summary of the operations
    println!("\n--------------------------- Summary ------------------------------------------");
    println!(" > Hostname changed to: '{}'", hostname);
    println!(" > Package Manager used: '{}'", package_manager);
    println!(" > installed package:");
    for result in install_results {
        println!("    - {}", result);
    }
    println!("------------------------------------------------------------------------------");

    Ok(())
}
