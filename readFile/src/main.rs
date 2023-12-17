#![allow(nonstandard_style)]

use std::fs;
use std::path::Path;
use std::file;

fn main() {
  let filePath = "/resources/example.txt";

  // ---------------------

  let __baseDirName = Path::new(file!()).parent().unwrap().to_string();
  let __dirname = format!("./{}", __baseDirName);
  let fileDirName: &str = &format!("{}{}", __dirname, filePath).to_string();

  // ---------------------

  println!("Reading contents of: {__dirname}/resources/example.txt");

  let text = fs::read_to_string(fileDirName).expect("This is stupid");

  println!("File contents: {text}")
}
