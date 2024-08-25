use std::process::{Command, exit};
use crate::engine::mac::mac_open_modify_config;
use crate::engine::win::win_start_ssh_agent;

pub fn load_ssh_key(key: &str) {
    println!("Loading ssh key {}", key);
    let done = clear_ssh_keys();
    if done {
        let output = Command::new("ssh-add").arg(key).output().expect("failed to execute process");
        if output.status.success() {
            println!("Key loaded successfully");
        } else {
            println!("Key failed to load");
        }
    }
}
pub fn delete_ssh_key(key: &str) -> bool {
    let output = Command::new("ssh-add").arg("-D").arg(key).output();
    match output {
        Ok(_) => true,
        Err(_) => false,
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
    let mut eval_output= Command::new("eval").arg("$(ssh-agent -s)").output();

    if cfg!(windows) {
        eval_output = win_start_ssh_agent();
    }

    match eval_output {
        Ok(_) => {
            let data = config_data.as_bytes();
            if cfg!(macos) {
                let file = mac_open_modify_config(data);
            }
            load_ssh_key(key)
        }
        Err(_) => {
            println!("Failed to start ssh-agent");
            exit(1)
        }
    }
}
