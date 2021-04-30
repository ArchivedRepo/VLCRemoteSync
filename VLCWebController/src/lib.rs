use roxmltree;
use std::collections::HashMap;


pub enum VLCState {
    STOPPED = 1,
    PAUSED = 2,
    PLAYING = 3,
}


pub struct VLCStatus {
    time: u32,
    is_full_screen: bool,
    file: String,
}


impl VLCState {
    
    // fn new(xml_string: &str) {
    //     let xml = match roxmltree::Document::parse(xml_string) {
    //         Ok(doc) => doc,
    //         Err(e) => {
    //             println!("Error: {}.", e);
    //             return;
    //         },
    //     };
    //     let mut table: HashMap<String, String> = HashMap::new();
    //     let time: u32;
    //     let is_full_scrren: bool;
    //     let file: String;
        
    //     for node in xml.descendants() {
    //         if node.has_tag_name("is_full_screen") {

    //         }
    //     }
        


    // }
}
