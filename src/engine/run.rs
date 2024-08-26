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
                println!("Key loaded successfully");
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
            }
            load_ssh_key(key)
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
