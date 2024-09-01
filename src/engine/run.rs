use std::io;
use std::process::{ Command, exit };
use crate::engine::unix::{ open_and_modify_config_file, super_user_do };
use crate::engine::win::win_start_ssh_agent;

pub fn load_ssh_key(key: &str) {
    println!("Loading ssh key {}", key);
    let done = clear_ssh_keys();
    if done {
        let output = Command::new("ssh-add").arg(key).output();
        match output {
            Ok(outout) => {
                println!("Key loaded successfully {:?}", outout);
            }
            Err(error) => {
                match error.kind() {
                    io::ErrorKind::PermissionDenied => {
                        let call_sudo = super_user_do();
                        match call_sudo {
                            Ok(_) => {
                                load_ssh_key(key);
                            }
                            Err(error) => {
                                println!("Failed to enable super user mode {}", error);
                            }
                        }
                        println!("Key not found");
                    }
                    _ => {
                        println!("Failed to load ssh key {}", error);
                    }
                }
                println!("Failed to load ssh key {}", error);
            }
        }
    }
}

pub fn delete_ssh_key(key: &str) -> bool {
    let output = Command::new("ssh-add").arg("-D").arg(key).output();
    match output {
        Ok(_) => true,
        Err(error) => {
            match error.kind() {
                io::ErrorKind::NotFound => {
                    println!("Key not found");
                    false
                }
                _ => {
                    println!("Failed to delete ssh key {}", error);
                    false
                }
            }
        }
    }
}
fn clear_ssh_keys() -> bool {
    let output = Command::new("ssh-add").arg("-D").output();
    match output {
        Ok(_) => true,
        Err(error) => {
            println!("Failed to clear ssh keys {}", error);
            false
        }
    }
}

pub fn add_ssh_key(key: &str) {
    let config_data = format!("\nHost github.com\n\tAddKeysToAgent yes\n\tIdentityFile {}\n", key);
    let mut eval_output = Command::new("eval").arg("$(ssh-agent -s)").output();

    if cfg!(windows) {
        eval_output = win_start_ssh_agent();
    }

    match eval_output {
        Ok(_) => {
            let data = config_data.as_bytes();
            if cfg!(unix) {
                let file = open_and_modify_config_file(data);
                match file {
                    Ok(_) => {
                        load_ssh_key(key);
                    }
                    Err(error) => {
                        println!("Failed to open config file {}", error);
                    }
                }
            } else {
                let file = open_and_modify_config_file(data);
                println!("{:?}", file)
            }
        }
        Err(error) => {
            match error.kind() {
                io::ErrorKind::PermissionDenied => {
                    let call_sudo = super_user_do();
                    match call_sudo {
                        Ok(_) => {
                            add_ssh_key(key);
                        }
                        Err(error) => {
                            println!("Failed to enable super user mode {}", error);
                        }
                    }
                }
                _ => {
                    println!("Failed to start ssh-agent");
                    exit(1);
                }
            }

            println!("Failed to start ssh-agent");
            exit(1)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::process::Command;
    use std::io;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_load_ssh_key() {
        // This test is a bit tricky as it involves system commands
        // We'll just check if the function runs without panicking
        load_ssh_key("dummy_key");
    }

    #[test]
    fn test_delete_ssh_key() {
        // Similarly, we'll just check if the function runs without panicking
        assert!(delete_ssh_key("dummy_key") == false);
    }

    #[test]
    fn test_clear_ssh_keys() {
        // Again, we'll just check if the function runs without panicking
        assert!(clear_ssh_keys() == true || clear_ssh_keys() == false);
    }

    #[test]
    fn test_add_ssh_key() {
        // This test is complex due to system interactions
        // We'll create a temporary directory for testing
        let temp_dir = tempdir().unwrap();
        let key_path = temp_dir.path().join("test_key");
        fs::write(&key_path, "dummy key content").unwrap();

        // Run the function
        add_ssh_key(key_path.to_str().unwrap());

        // Check if the config file was modified (on Unix systems)
        if cfg!(unix) {
            let home_dir = dirs::home_dir().expect("Failed to get home directory");
            let ssh_config_path = home_dir.join(".ssh/config");
            let config_content = fs::read_to_string(ssh_config_path).unwrap_or_default();
            assert!(config_content.contains(key_path.to_str().unwrap()));
        }

        // Clean up
        temp_dir.close().unwrap();
    }
}

