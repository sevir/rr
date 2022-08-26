use std::path::{Path, PathBuf};
use std::env;
extern crate dirs;
use dirs::home_dir;

pub fn get_code_from_file(codefile : &str) -> Option<PathBuf>
{
    let current_dir_file = env::current_dir().unwrap().join(Path::new(codefile));

    let profile_file = home_dir().unwrap().join(Path::new(codefile));

    let return_value: Option<PathBuf> = if current_dir_file.exists(){
        Some(current_dir_file)
    }else if profile_file.exists(){
        Some(profile_file)
    }else{
        None
    };

    return_value
}
