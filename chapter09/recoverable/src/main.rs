use std::{fs::File, io::ErrorKind};

fn main() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) if error.kind() == ErrorKind::NotFound => {
            match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => {
                    panic!("Problem creating the file: {:?}", e);
                }
            }
        },
        Err(error) => {
            panic!("Problem opening the file: {:?}", error);
        }
    };
}
