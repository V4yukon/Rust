use std::io::ErrorKind;
use std::fs::File;

fn main() {
    let open_file = File::open("hello.txt");

    let file_pass = match open_file{
        Ok(file) => file,
        Err(error) => match error.kind(){
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("{:?}", e),
            },
            other_error => {
                panic!("Proplem opening the file {:?}", other_error)
            }
        },
    };
}

fn closure() {
    let open_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem opening the file {:?}", error);
            })
        }else{
            panic!("Problem opening the file {:?}", error);
        }
    });
}

