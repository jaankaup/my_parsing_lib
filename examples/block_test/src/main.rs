use quick_xml::name::QName;
use json::misc::{
    load_from_file,
    seek_block,
    seek_xml_block
};

pub fn main() {
    let source = load_from_file(String::from("joo.txt")).unwrap(); 
    // let indexer = SourceIndexer::init();
    let indices = seek_block(&source, '{', '}');
    println!("indices :: {:?}", indices);

    let source_xml = load_from_file(String::from("punamusta_tuho.dek")).unwrap(); 
    // let source_xml = load_from_file(String::from("puna_kakka.dek")).unwrap(); 
    let tags = vec![(QName(b"deck"), 0)];
    seek_xml_block(&source_xml, &tags);
}
