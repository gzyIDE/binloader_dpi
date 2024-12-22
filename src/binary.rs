use std::fs::File;
use std::io::{Read, BufReader};
use crate::memory::{MemPages, Memory};

pub fn loadbin(fname: &str, memory: &mut MemPages) -> Result<usize, std::io::Error> {
    let mut size = 0;
    for result in BufReader::new(File::open(fname)?).bytes() {
        //let byte = result.1?;
        //println!("{}", byte);
        memory.write8(size, result?);
        size += 1;
    }

    return Ok(size)
}
