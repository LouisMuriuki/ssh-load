use std::{ fs::OpenOptions, io::{ Write, Error } };

// fn open_ssh_config() -> Result<String, io::Error> {
//     let home_dir = dirs::home_dir().expect("Failed to get home directory");
//     let ssh_config_path = home_dir.join(".ssh/config");
//     let file = File::open(ssh_config_path);
//     match file {
//         Ok(file) => {
//             return file;
//         }
//         Err(error) => {
//             match error.kind() {
//                 io::ErrorKind::NotFound => { create_ssh_config_file(ssh_config_path) }
//                 _ => {
//                     println!("Failed to open file {}", error);
//                     return Err(std::fmt::Error);
//                 }
//             }
//         }
//     }
// }

// fn create_ssh_config_file(path: PathBuf) -> Result<String, io::Error> {
//     let file = File::create(path);
//     match file {
//         Ok(file) => file,
//         Err(error) => {
//             println!("Failed to create file {}", error);
//             return Err(std::fmt::Error);
//         }
//     }
// }

pub fn mac_open_modify_config(data: &[u8]) -> Result<(), Error> {
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
