use std::fs;
use std::io;

pub fn delete(){
    // ASK/MAKE THE DIRECTORY TO CREATE THE FILE -> START
    let reader = io::stdin();

    let mut directory: String = String::new(); // file directory
    let mut name: String = String::new(); // file name and extension

    println!("Please, enter the file name with the extension:");
    reader.read_line(&mut name).ok().expect("ERRMSG"); // users input
    name = name[..name.len()-2].parse().unwrap(); // remove \n

    println!("Please, put the directory where the file is located:");
    reader.read_line(&mut directory).ok().expect("ERRMSG"); // users input
    directory = directory[..directory.len()-2].parse().unwrap(); // remove \n


    let full_path = directory + &name;
    // END

    // DELETE THE FILE
    println!("\nSuccessful Deleted File In -> {}\n", full_path);
    fs::remove_file(full_path); // REMOVE THE FILE FROM THE DIRECTORY

}