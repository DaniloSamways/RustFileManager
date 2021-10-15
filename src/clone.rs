extern crate fs_extra;
use fs_extra::dir::*;

pub fn clone_dir(){
    //fs::copy("src/created_files/dsadds.txt", "src/created_files/arquivo.txt");
    let options = CopyOptions::new();
    copy("src/created_files", "src/pasta", &options);
}