
use std::fs::{self, File};
use std::io::{self, ErrorKind, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("username.txt");
    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    println!("My file object contains, fle descriptor {:?}", f);

    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(bytes_read) => {
            println!("{}", bytes_read);
            Ok(s)
        },
        Err(e) => Err(e),
    }

   
}

fn read_username_from_file_2ver() -> Result<String, io::Error> {
    let mut f = File::open("username.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn read_username_from_file_3ver() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("username.txt")?.read_to_string(&mut s)?;
    Ok(s)
}

fn read_username_from_file_4ver() -> Result<String, io::Error> {
    fs::read_to_string("username.txt")
}

fn main() {
   let name = read_username_from_file().unwrap();
   let name = read_username_from_file_4ver().unwrap();
   println!("No error detected, no crash!, {}" , name);
    
}