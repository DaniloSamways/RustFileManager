#[allow(unused_imports)]
use std::fs;
use std::io;
use std::clone;

pub fn delete_all(){
    // ASK/MAKE THE DIRECTORY TO DELETE ALL FILES INSIDE -> START
    let reader = io::stdin();
    let mut directory: String = String::new(); // directory
    let mut ask: String = String::new(); // Y or N

    println!("Please, enter all the folder directory to delete all files inside:");
    reader.read_line(&mut directory).ok().expect("ERRMSG"); // users input
    directory = directory[..directory.len()-2].parse().unwrap(); // remove \n

    println!("Delete ALL files inside -> {}? (Y/N)", directory);
    reader.read_line(&mut ask).ok().expect("ERRMSG"); // users input
    ask = ask[..ask.len()-2].parse().unwrap(); // remove \n


    let directory2 = directory.clone(); // clone directory
    // END

    // DELETE ALL FILES
    if ask == "Y"{
        println!("Successful Deleted All Files In -> {}", directory2);
        fs::remove_dir_all(directory).ok().expect("ERRMSG");
        fs::create_dir(directory2).ok().expect("ERRMSG");

    } else if ask == "N" {
        println!("Operation Cancelled");
    }
}