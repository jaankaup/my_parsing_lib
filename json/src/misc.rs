use std::io;
use std::fs::File;
use std::io::{BufReader, Read, BufRead};
use std::error::Error;
use std::path::Path;
// use std::{error::Error, fmt::Debug};
use thiserror::Error;
use quick_xml::{
    Reader,
};
use quick_xml::events::Event;
use quick_xml::name::QName;


#[derive(Error, Debug)]
pub enum BlockError {
    #[error("Malformed input. There are more `{0}` characters than `{1}` characters")]
    EndCharError(char, char),

    #[error("The input is malformed. Odd number of `{0}` and `{1}` pairs.")]
    OddNumberError(char, char),

}

pub fn seek_block(input_str: &String, start: char, end: char) -> Result<Vec<(u64, u64)>, BlockError> {

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
            if depth == 0 { return Err(BlockError::EndCharError( start, end)); }
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
        return Err(BlockError::OddNumberError(end, start)); 
    }

    Ok(result)
}

pub fn seek_xml_blocks(input_str: &String, tags: &Vec<(QName, u32)>) {
    let mut reader = Reader::from_str(input_str);
    reader.trim_text(true);

    let mut buffer = Vec::<u8>::new();
    let mut buffer_position = reader.buffer_position();
    loop {
        match reader.read_event_into(&mut buffer).unwrap() {
            Event::Start(e) => {
                for x in tags {
                    if e.name().as_ref() == x.0.local_name().as_ref() {

                        let a = buffer_position;
                        let mut span = reader.read_to_end(x.0).unwrap();
                        buffer_position = reader.buffer_position();
                        println!("{:?}", input_str[a..buffer_position].to_string());
                        break;
                    }
                }
            },
            Event::End(e) => {
                // buffer_position = reader.buffer_position();
            }
            Event::Empty(e) => {
                let a = buffer_position;
                buffer_position = reader.buffer_position();
                println!("{:?}", input_str[a..buffer_position].to_string());
            }
            Event::Eof => { break; },
            _ => {
                   buffer_position = reader.buffer_position();
                   continue;
            }
        }
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
