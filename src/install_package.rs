use duct::cmd;

pub fn install_package_with_apt(package: &str) -> Result<(), Box<dyn std::error::Error>> {
    cmd!("sudo", "apt-get", "install", "-y", package).run()?;
    Ok(())
}

pub fn install_package_with_dnf(package: &str) -> Result<(), Box<dyn std::error::Error>> {
    cmd!("sudo", "dnf", "install", "-y", package).run()?;
    Ok(())
}

pub fn install_package_with_pacman(package: &str) -> Result<(), Box<dyn std::error::Error>> {
    cmd!("sudo", "pacman", "-S", "--noconfirm", package).run()?;
    Ok(())
}

pub fn install_package_with_winget(package: &str) -> Result<(), Box<dyn std::error::Error>> {
    cmd!("winget", "install", package).run()?;
    Ok(())
}

pub fn install_package_with_homebrew(package: &str) -> Result<(), Box<dyn std::error::Error>> {
    cmd!("brew", "install", package).run()?;
    Ok(())
}

// Example usage
fn main() -> Result<(), Box<dyn std::error::Error>> {
    install_package_with_apt("curl")?;
    install_package_with_dnf("curl")?;
    install_package_with_pacman("curl")?;
    install_package_with_winget("curl")?;
    install_package_with_homebrew("curl")?;
    Ok(())
}
