#![allow(nonstandard_style)]
use std::file;
use std::path::Path;
use bitreader::BitReader;
use std::fs;

fn main() {
  let currentDir = Path::new(file!()).parent().unwrap().to_str().unwrap();
  let filePath = "/resources/example.txt";

  let fileDir = "./".to_string() + currentDir + filePath;

  let reader = fs::read(fileDir).expect("This is stupid");

  let mut bitReader = BitReader::new(reader.as_slice());

  for _ in 0..8 {
    let bit = bitReader.read_u8(1).unwrap();
    println!("Bit: {}", bit);
  };
}
