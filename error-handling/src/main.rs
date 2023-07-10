use std::error::Error;
use std::fs::{self, File};
use std::io::{self, ErrorKind, Read};

fn main() -> Result<(), Box<dyn Error>> {
    let _greeting_file = File::open("hello.txt")?;

    Ok(())
}

fn _manual_panic() {
    panic!("Crash and burn!");
}

fn _invalid_index_panic() {
    let v = vec![1, 2, 3];

    v[99];
}

fn _recoverable_errors() {
    let greeting_file_result = File::open("hello.txt");

    let _greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.text") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => panic!("Problem opening the file: {:?}", other_error),
        },
    };
}

fn _unwrapping() {
    let _greeting_file_result = File::open("hello.text").unwrap();
}

fn _expecting() {
    let _greeting_file_result =
        File::open("hello.text").expect("hello.txt should be included in this project");
}

fn _propagating_errors() {
    let _result = _read_username_from_file().expect("Could not read username");
    let _result = _read_username_from_file_quick().expect("Could not read username");
    let _result = _read_username_from_file_quickest().expect("Could not read username");
    let _result = _read_username_from_file_ultimate().expect("Could not read username");
}

fn _read_username_from_file() -> Result<String, io::Error> {
    let username_result = File::open("hello.txt");

    let mut username_file = match username_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

fn _read_username_from_file_quick() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

fn _read_username_from_file_quickest() -> Result<String, io::Error> {
    let mut username = String::new();

    File::open("hello.text")?.read_to_string(&mut username)?;

    Ok(username)
}

fn _read_username_from_file_ultimate() -> Result<String, io::Error> {
    fs::read_to_string("hello.text")
}
