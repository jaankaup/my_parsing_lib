use json::misc::{
    load_from_file,
    SourceIndexer,
};

pub fn main() {
    let source = load_from_file(String::from("joo.txt")).unwrap(); 
    let indexer = SourceIndexer::init();
    let indices = indexer.seek_block(&source, '{', '}');
    println!("indices :: {:?}", indices);
}
