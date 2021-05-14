use std::fs;
use std::fs::File;
use std::io::{self, ErrorKind, Read};

fn main() {
    println!("Example: error handling with Result");
    let f = File::open("aaa.txt");
    match f {
        Ok(file) => {
            println!("Ok: got file handle: {:?}", file);
        }
        Err(error) => {
            println!("Err: issue opening the file: {:?}", error);
        }
    };
    println!("---------------------------------\n");

    println!("Example: error handling with Err Result specific error type handling (the long way)");
    let f = File::open("bbb.txt");
    let f = match f {
        Ok(file) => file,
        Err(error) => {
            println!("file not found, creating...");
            match error.kind() {
                ErrorKind::NotFound => match File::create("bbb.txt") {
                    Ok(fc) => fc,
                    Err(e) => panic!("Problem creating the file: {:?}", e),
                },
                other_error => {
                    panic!("Problem opening the file: {:?}", other_error)
                }
            }
        }
    };
    println!("f: {:?}", f);
    println!("---------------------------------\n");

    println!("Example: error handling with Err Result specific error type handling (the shorter way)");
    let f = File::open("ccc.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("ccc.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
    println!("f: {:?}", f);
    println!("---------------------------------\n");

    println!("Example: using unwrap which will call panic! on error...");
    let f = File::open("bbb.txt").unwrap();
    println!("f: {:?}", f);
    println!("---------------------------------\n");

    println!("Example: using expect which will call panic! on error with a custom message...");
    let f = File::open("ccc.txt").expect("ISSUE: couldn't open the file");
    println!("f: {:?}", f);
    println!("---------------------------------\n");

    println!("Example: various ways of reading text from a file with error handling...");
    let user1 = read_username_from_file_1();
    let user2 = read_username_from_file_2();
    let user3 = read_username_from_file_3();
    let user4 = read_username_from_file_4();
    let users = vec![user1, user2, user3, user4];
    println!("Users should be the same: {:?}", users);

}

fn read_username_from_file_1() -> Result<String, io::Error> {
    let f = File::open("user.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn read_username_from_file_2() -> Result<String, io::Error> {
    let mut f = File::open("user.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn read_username_from_file_3() -> Result<String, io::Error> {
    let mut s = String::new();

    File::open("user.txt")?.read_to_string(&mut s)?;

    Ok(s)
}

fn read_username_from_file_4() -> Result<String, io::Error> {
    fs::read_to_string("user.txt")
}
