use std::{
    fs::File,
    io::{Error, ErrorKind, Read},
};

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

fn try_to_open_file_with_unwrap(path: &str) -> File {
    File::open(path).unwrap()
}

fn try_to_open_file_with_expect(path: &str) -> File {
    File::open(path).expect(&format!("{path} must exist in this project"))
}

fn read_username_from_file_explicit(path: &str) -> Result<String, Error> {
    let mut username_file = match File::open(path) {
        Ok(f) => f,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    if let Err(e) = username_file.read_to_string(&mut username) {
        Err(e)
    } else {
        Ok(username)
    }
}

fn read_username_from_file(path: &str) -> Result<String, Error> {
    let mut username_file = File::open(path)?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

fn read_username_from_file_succinct(path: &str) -> Result<String, Error> {
    let mut username = String::new();

    File::open(path)?.read_to_string(&mut username)?;

    Ok(username)
}

pub fn test() {
    let creatable_file_path = "non_existing_file.txt";
    let unreadable_file_path = "unreadable.txt";
    let contentful_file = "file_has_content.txt";

    match read_username_from_file_explicit(contentful_file) {
        Ok(username) => println!("username in file: {username}"),
        Err(e) => println!("could not read from file: {contentful_file}: {e:?}"),
    }

    match read_username_from_file(contentful_file) {
        Ok(username) => println!("username in file: {username}"),
        Err(e) => println!("could not read from file: {contentful_file}, {e:?}"),
    }

    match read_username_from_file_succinct(contentful_file) {
        Ok(username) => println!("username in file: {username}"),
        Err(e) => println!("could not read from file: {contentful_file}, {e:?}"),
    }

    try_to_open_file_with_expect(creatable_file_path);
    try_to_open_file_with_unwrap(creatable_file_path);
    try_to_open_file(creatable_file_path);
    try_to_open_file(unreadable_file_path);
}
