
use std::fs::{self, File};
use std::io::{self, ErrorKind, Read};

fn main() {
    let f = File::open("hello.txt").unwrap();
    let f = File::open("hello.txt").expect("Failed to open hello.txt");
    
}