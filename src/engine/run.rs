
use std::{ f32::consts::E, process::Command };


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
    let eval_output = Command::new("eval").arg("$(ssh-agent -s)").output();
    match eval_output {
        Ok(_) => {
           load_ssh_key(key)
        }
        Err(_) => {
            println!("Failed to start ssh-agent");
        }
    }
}
