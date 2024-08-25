use std::{ fmt::Result, fs, io };
fn open_ssh_config() -> Result<String, std::fmt::Error> {
    let home_dir = dirs::home_dir().ok_or(io::Error::new(io::ErrorKind::NotFound, "Home directory not found"))?;
    let ssh_config_path = home_dir.join(".ssh/config");
    let file = fs::read_to_string("~/.ssh/config");
    match file {
        Ok(file) => file,
        Err(error) => {
            match error.kind() {
                io::ErrorKind::NotFound => {
                    create_ssh_config(ssh_config_path)
                }
                _=>{
                    println!("Failed to open file {}", error);
                    return Err(std::fmt::Error);
                }
            }
        }
    }
}

fn create_ssh_config(path) -> Result<String, io::Error> {
    let file = fs::File::create(ssh_config_path);
    match file {
        Ok(file) => file,
        Err(error) => {
            println!("Failed to create file {}", error);
            return Err(std::fmt::Error);
        }
    }
}
pub fn mac_modify_config() -> Result<String, std::fmt::Error> {}
