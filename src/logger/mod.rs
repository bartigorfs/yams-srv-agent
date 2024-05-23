use std::fs::File;
use std::io::{ErrorKind, Write};

pub fn write_to_log(message: String) {
    let log_file_result = File::open("log.txt");

    match log_file_result {
        Ok(mut file) => {
            let write_result = file.write_all(message.as_bytes());
            handle_file_result(write_result);
        }
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("log.txt") {
                Ok(mut fc) => {
                    let write_result = fc.write_all(message.as_bytes());
                    handle_file_result(write_result);
                }
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        },
    };
}

fn handle_file_result<E: std::fmt::Debug>(result: Result<(), E>) {
    match result {
        Ok(..) => return,
        Err(e) => panic!("Problem writing to the file: {:?}", e),
    }
}