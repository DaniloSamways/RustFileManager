use std::io;
use std::fs::OpenOptions;
use std::fs;

pub fn create(){
    // ASK/MAKE THE DIRECTORY TO CREATE THE FILE -> START
    let reader = io::stdin();

    let mut name: String = String::new(); // file name
    let mut directory: String = String::new(); // file directory
    let mut extension: String = String::new(); // file extension

    println!("Please, enter the file name:");
    reader.read_line(&mut name).ok().expect("ERRMSG"); // users input
    name = name[..name.len()-2].parse().unwrap(); // remove \n

    println!("Please, enter the full file directory:");
    reader.read_line(&mut directory).ok().expect("ERRMSG"); // users input
    directory = directory[..directory.len()-2].parse().unwrap(); // remove \n

    println!("Please, enter the file extension:");
    reader.read_line(&mut extension).ok().expect("ERRMSG"); // users input
    extension = extension[..extension.len()-2].parse().unwrap(); // remove \n

    
    extension = name + "." + &extension; // file extension
    let full_path = directory.to_owned() + &extension; // full directory

    println!("Successful Created File In -> {}", full_path);
    // END

    // CREATE THE FILE -> START
    let _file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(full_path);
    // END
}