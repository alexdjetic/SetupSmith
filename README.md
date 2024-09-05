# SetupSmith

SetupSmith is a command-line tool for managing system configuration and package installations based on YAML or JSON configuration files.

## Overview

SetupSmith allows you to:
- Change the system hostname.
- Install a list of packages using the specified package manager.
- Supports configuration files in YAML and JSON formats.

## Usage

Run the application with the following command format:

```bash
SetupSmith /path/to/file.yaml # or /path/to/file.json
```

## Configuration File

The configuration file must be in YAML or JSON format. It should contain the following fields:

- `hostname`: The new hostname for the system.
- `package_manager`: The package manager to use (`apt`, `dnf`, `pacman`, `homebrew`, `winget`).
- `packages`: A list of packages to install.

### Example YAML File

```yaml
hostname: "host"
package_manager: "pacman"
packages:
  - firefox
  - discord
  - obs
```

### Example JSON File

```json
{
  "hostname": "host",
  "package_manager": "pacman",
  "packages": [
    "firefox",
    "discord",
    "obs"
  ]
}
```

## Requirements

- Root or administrative privileges are required to change the hostname and install packages.
- At least Rust and cargo for dev 

## Error Handling

- The application will exit with an error message if the hostname, package manager, or packages are missing or invalid.
- Package installation failures will be summarized in the output, but the script will continue with other packages.

## Installation

Ensure you have Rust installed. Clone the repository and run:

```bash
git clone https://github.com/alexdjetic/SetupSmith.git
cargo build --release
```

The binary will be located in the *target/release* directory.

Run:

```bash
git clone https://github.com/alexdjetic/SetupSmith.git
cargo run -- /path/to/file.yaml
```

## License

This project is licensed under the MIT License. See the LICENSE file for details.
