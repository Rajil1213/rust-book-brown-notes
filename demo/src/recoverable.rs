use std::{fs::File, io::ErrorKind};

fn try_to_open_file(path: &str) -> File {
    let file_result = File::open(path);

    let my_file = match file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create(path) {
                Ok(f) => f,
                Err(e) => panic!("could not create file: {}, {:?}", path, e),
            },
            other_error => panic!(
                "an error occurred while opening file: {}, {:?}",
                path, other_error
            ),
        },
    };

    return my_file;
}

pub fn test() {
    let creatable_file_path = "non_existing_file.txt";
    let unreadable_file_path = "unreadable.txt";

    try_to_open_file(creatable_file_path);
    try_to_open_file(unreadable_file_path);
}
