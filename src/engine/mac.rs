

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


