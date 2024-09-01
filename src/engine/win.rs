use std::process::{ Command, Output, };
use std::io;

/// Starts the SSH agent service on Windows.
///
/// Sets the SSH agent service to start automatically and then starts it.
///
/// # Returns
/// `Result<Output, io::Error>` containing the command output or an error.
pub fn win_start_ssh_agent() -> Result<Output, io::Error> {
    let output_set_service = Command::new("powershell")
        .arg("--Command")
        .arg("Set-Service -Name ssh-agent -StartupType Automatic")
        .output();

    match output_set_service {
        Ok(output) => {
            let output_start_service = Command::new("powershell")
                .arg("--Command")
                .arg("Start-Service ssh-agent")
                .output();
            return output_start_service;
        }
        Err(error) => {
            println!("Failed to set ssh-agent service to automatic {}", error);
            return Err(error);
        }
    }
}
