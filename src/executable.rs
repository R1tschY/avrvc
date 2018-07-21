use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::ffi::OsStr;

trait ExecutableReader {
    fn parse(bytes: &Vec<u8>) -> Vec<u8>;
}

pub fn read_executable_file(filepath: &Path) -> Vec<u8> {
    let mut f = File::open(filepath).expect("file not found");
    let mut contents: Vec<u8> = Vec::new();
    f.read_to_end(&mut contents)
        .expect("something went wrong reading the file");

    if filepath.extension() == Some(OsStr::new("bin")) {
        return BinaryReader::parse(&contents);
    }
    panic!("unknown file type")
}


struct BinaryReader { }

impl ExecutableReader for BinaryReader {
    fn parse(bytes: &Vec<u8>) -> Vec<u8> {
        bytes.clone()
    }
}

