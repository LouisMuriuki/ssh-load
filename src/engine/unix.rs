use std::{ fs::OpenOptions, io::{ Error, Write }, process::Command };

pub fn super_user_do() -> Result<(), Error> {
    if cfg!(unix) {
        let output = Command::new("sudo").arg("-s").arg("-H").output();
        match output {
            Ok(_) => {
                println!("Super user mode enabled");
                Ok(())
            }
            Err(error) => {
                println!("Failed to enable super user mode {}", error);
                Err(error)
            }
        }
    } else {
        println!("Super user mode not supported on this platform");
        Ok(())
    }
}

pub fn open_and_modify_config_file(data: &[u8]) -> Result<(), Error> {
    let home_dir = dirs::home_dir().expect("Failed to get home directory");
    let ssh_config_path = home_dir.join(".ssh/config");
    let config_file = OpenOptions::new()
        .append(true) // Open in append mode
        .create(true) // Create the file if it doesn't exist
        .open(ssh_config_path);
    match config_file {
        Ok(mut file) => {
            match file.write_all(data) {
                Ok(file) => {
                    println!("Data written to file");
                    return Ok(file);
                }
                Err(error) => {
                    println!("Failed to write data to file {}", error);
                    return Err(error);
                }
            }
        }
        Err(error) => {
            println!("Failed to open file {}", error);
            return Err(error);
        }
    }
}
