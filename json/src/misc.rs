use std::fmt::Debug;
use std::io;
use std::fs::File;
use std::io::{BufReader, Read, BufRead};
use std::error::Error;
use std::path::Path;
use thiserror::Error;
use quick_xml::{
    Reader,
};
use quick_xml::de::Deserializer;
use quick_xml::events::Event;
use quick_xml::name::QName;
use serde::Deserialize;


#[derive(Error, Debug)]
pub enum BlockError {
    #[error("Malformed input. There are more `{0}` characters than `{1}` characters")]
    EndCharError(char, char),

    #[error("The input is malformed. Odd number of `{0}` and `{1}` pairs.")]
    OddNumberError(char, char),

}

/// Seek blocks using start and end chars.
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

/// Find and deserialize xml elements.
pub fn seek_xml_blocks<'de, T: Deserialize<'de> + Debug>(input_str: &'de String, tag: &QName, size_hint: Option<u32>) -> Vec<T> {

    let mut result: Vec<T> = Vec::with_capacity(size_hint.unwrap_or_else(|| 10).try_into().unwrap());
    let mut reader = Reader::from_str(input_str);

    reader.trim_text(true);

    let mut buffer_position = reader.buffer_position();
    loop {
        match reader.read_event().unwrap() {
            Event::Start(e) => {
                if e.name().as_ref() == tag.local_name().as_ref() {

                    // The tag name lengt + '<' + '>' + attributes len.
                    let start_offset = e.name().0.len() + e.attributes_raw().len() + 2;

                    // The start position of the tag event. 
                    let start_pos = reader.buffer_position() - start_offset;

                    // Read to the end. Consume the input. 
                    let _ = reader.read_to_end(*tag).unwrap();

                    // Take the end position. Now we have the slice indices that contains the whole
                    // xml element. Update the buffer position.
                    buffer_position = reader.buffer_position();

                    // Deserialize.
                    let mut deserializer = Deserializer::<'de>::from_str(&input_str[start_pos..buffer_position]);
                    let t: T = T::deserialize(&mut deserializer).unwrap();

                    // Save deserialized data.
                    result.push(t);
                }
            },
            Event::End(e) => {
                // Not used for now.
            }
            Event::Empty(e) => {

                // Handle a end element "<SomeEmptyTag />"
                let a = buffer_position;
                buffer_position = reader.buffer_position();
            }
            Event::Eof => { break; },
            _ => {
                   buffer_position = reader.buffer_position();
                   continue;
            }
        }
    }
    result
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
