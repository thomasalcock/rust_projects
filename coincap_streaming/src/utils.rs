
use std::fs::{File, OpenOptions};
use std::path::Path;

pub fn create_output_stream(file_path: &str) -> std::fs::File {
    let file = match OpenOptions::new().create(true).append(true).open(file_path) {
        Ok(file) => file,
        Err(err) => panic!("Error opening the file: {}", err),
    };

    file
}

pub fn create_or_append_output_file(file_name: &str) -> File {
    let path = Path::new(&file_name);
    let file_exists: bool = path.exists() && path.is_file();
    let file: File = create_output_stream(file_name);
    if file_exists {
        println!(
            "The file {} already exists. Data will be appended to this file.",
            file_name
        );
    } else {
        println!(
            "The file {} does not yet exist and will be created!",
            file_name
        );
    }
    file
}
