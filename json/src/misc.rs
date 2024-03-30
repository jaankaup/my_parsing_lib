use std::io;
use std::fs::File;
use std::io::{BufReader, Read, BufRead};
use std::error::Error;
use std::path::Path;

pub struct SourceIndexer {

    start_index: u32,
    end_index: u32,
    // temp_buffer: Vec[u8],
}

impl SourceIndexer {

    pub fn init() -> Self {
        Self {
            start_index: 0,
            end_index: 0,
        }
    }

    pub fn seek_block(&self, input_str: &String, start: char, end: char) -> Vec<(u64, u64)> {

        let mut start_index = 0; 
        let mut end_index = 0; 
        let mut depth: i32 = 0;
        let mut result = Vec::<(u64,u64)>::new();
        
        for c in input_str.char_indices() {
            // The start of a block.
            if c.1 == start && depth == 0 { // && start_index == end_index {
                start_index = c.0; 
                depth += 1;
            }
            else if c.1 == start {
                depth += 1;
            }
            else if c.1 == end {
                if depth == 0 { panic!("The input is malformed. There are more {:?} characters than {:?} characters.", end, start); }
                depth -= 1;
                // An enclosing char found.
                if depth == 0 {
                    end_index = c.0;
                    result.push(
                        (start_index.try_into().unwrap(),
                         end_index.try_into().unwrap())
                    );
                }
            }
        }

        if depth != 0 {
            panic!("The input is malformed. Odd number of {:?} and {:?} pairs.", start, end);
        }

        result
    }
}

pub fn load_from_file(filename: String) -> Result<String, Box<dyn Error>> {
    let mut f = File::open(filename)?;
    let mut buffer = String::new();
    f.read_to_string(&mut buffer)?;
    Ok(buffer)
}

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
