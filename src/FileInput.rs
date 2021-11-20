use core::fmt;
use std::fs;

pub fn GetInput(subfolder: &str) -> String
{
    let pathName = ".\\IO\\".to_string() + &subfolder.to_string();
    return fs::read_to_string(&pathName).expect(&format!("{}", &pathName));
}