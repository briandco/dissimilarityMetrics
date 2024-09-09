use core::str;
use regex::Regex;
use std::{cell::OnceCell, collections::HashMap, error, fs::File, io::{Read, Result}};

#[derive(Debug, Clone)]
#[no_mangle]
pub struct SrcReference{
    pub foreign: bool,
    pub src_ref: String,
    pub partial: bool,
    pub content: Vec<String>,
}

#[derive(Debug, Clone)]
#[no_mangle]
pub struct SuperBlock{
    pub block_name: String,
    // partial: bool,
    pub src_references: HashMap<String, SrcReference>,
}

#[derive(Debug, Clone)]
#[no_mangle]
pub struct FileBlock<'a>{
    file_path: &'a str, 
    blocks: Vec<SuperBlock>,
}

// pub static mut RESULT_HASHMAP:OnceCell<HashMap<String, SuperBlock>> = OnceCell::new();



impl<'a> FileBlock <'a>{
    pub fn new(file_path: &'a str) -> Self{
        FileBlock{
            file_path,
            blocks: Vec::new(),
        }
    }

    /// Reads file & returns the content as buffer of Vec<u8>
    ///
    /// Reads the file from the provided absolute path & returns the buffer of Vec<u8> in the Result<>
    pub fn read_binary_file(&self) -> Result<String> {
        let mut file = File::open(self.file_path)?;
        let mut buffer: String = String::new();
        file.read_to_string(&mut buffer)?;
        Ok(buffer)
    }

    /// Extracts the the file name from the given content that is read from file
    ///
    /// Input to the function is the contents of the file & we get the file name as String wrapped in Option<> type.
    pub fn get_file_name(&self, file_content: &str) -> Option<String> {
        let content_str = &file_content;
        let re = Regex::new(r#"\.file\s+"([^"]+)""#).unwrap();
        re.captures(content_str)
            .and_then(|cap| cap.get(1).map(|m| m.as_str().to_string()))
    }

    pub fn create_src_reference_block(mut content: Vec<String>, src_ref:String, is_foreign_ref:bool) -> SrcReference{
        let mut is_partial: bool = false;
        for each_str in content.iter_mut(){ // For loop to remove unwanted /t- from the content
            *each_str = each_str.replace("\t", "  ");
            if each_str.contains("j"){
                is_partial = true;
            }
        }
        
        SrcReference{
            foreign: is_foreign_ref,
            partial: is_partial,
            src_ref: src_ref.to_string(),
            content: content
        }
    }

    pub fn create_super_block(content: HashMap<String, SrcReference>, block_name: Option<String>) -> Option<SuperBlock>{
            match block_name {
                Some(block_name) =>{
                    Some(SuperBlock{
                        block_name: block_name,
                        src_references: content
                    })
                }
                None => {
                    println!("super block name not found");
                    Some(SuperBlock{
                        block_name: "UnnamedSuperBlock".to_string(),
                        src_references: content
                    })
                }
            }
        }

    /// Generate the hash map of key: value pair from the provided contents & file name
    /// 
    /// Generate the hash map of key: value pair where keys are file_name:line numbers & respective value is the 
    /// data following to that particular line. 
    /// WIP..
    pub fn generate_hashmap(&self, file_content: &str, filename: &str) -> HashMap<String, SuperBlock>{
        // let content_str = str::from_utf8(file_content).expect("Invalid U'TF-8 sequence");
        let regex_pattern = r#"(?m)^#\s*(?:\./|/)([\w/.\+-]+):(\d+):"#;
        let re = Regex::new(&regex_pattern).unwrap();
        let mut iter = file_content.lines().peekable();  
        let mut super_block_key: Option<String> = None;
        let mut src_ref_map:HashMap<String, SrcReference> = HashMap::new();
        let mut hash_map: HashMap<String, SuperBlock> = HashMap::new();
        println!("new vec created");

        while let Some(line) = iter.next(){
            // println!("some(line) = {:?}", line);
            if let Some(cap) = re.captures(line){
                // println!("matched line --> {:?}", cap.get(0));
                if let Some(file_path) = cap.get(1){
                    if let Some(line_number) = cap.get(2){
                        let key = format!("{}:{}", file_path.as_str(), line_number.as_str());
                        let super_block = file_path.as_str().contains(filename);
                        if super_block{
                            super_block_key = format!("{}:{}", file_path.as_str(), line_number.as_str()).into();
                            // println!("super block key: {}", super_block_key.unwrap());
                        }
                        let mut instruction = Vec::new();
                        
                        // let mut super_block: SuperBlock;
                        while let Some(next_line) = iter.peek(){
                            if re.is_match(next_line){
                                let foreign_ref = !key.contains(filename);
                                let src_ref_block = Self::create_src_reference_block(instruction, key.clone(), foreign_ref);
                                // println!("src reference block is {:?}", src_ref_block);
                                src_ref_map.insert(key, src_ref_block);
                                // println!("till now src ref vec");
                                // for  each_src in src_ref_vec.iter(){
                                //     println!("{:?}", *each_src);
                                // }
                                if next_line.contains(filename){
                                    let super_block = Self::create_super_block(src_ref_map.clone(), super_block_key.clone()).unwrap();
                                    src_ref_map = HashMap::new();
                                    hash_map.insert(super_block_key.clone().unwrap_or("UnnamedSuperBlock".to_string()), super_block);
                                }
                                break;
                            }
                            instruction.push(iter.next().unwrap().to_string());
                        }
                    }
                }
            }
        }  
        hash_map
    }


}

/// Reads file & returns the content as buffer of Vec<u8>
///
/// Reads the file from the provided absolute path & returns the buffer of Vec<u8> in the Result<>
pub fn read_binary_file(file_path: &str) -> Result<Vec<u8>> {
    let mut file = File::open(file_path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    Ok(buffer)
}

/// Extracts the the file name from the given content that is read from file
///
/// Input to the function is the contents of the file & we get the file name as String wrapped in Option<> type.
pub fn extract_file_name(file_content: &[u8]) -> Option<String> {
    let content_str = str::from_utf8(file_content).ok()?;
    let re = Regex::new(r#"\.file\s+"([^"]+)""#).unwrap();
    re.captures(content_str)
        .and_then(|cap| cap.get(1).map(|m| m.as_str().to_string()))
}


/// Generate the hash map of key: value pair from the provided contents & file name
/// 
/// Generate the hash map of key: value pair where keys are file_name:line numbers & respective value is the 
/// data following to that particular line. 
/// WIP..
pub fn generate_hashmap(file_content: &[u8], filename: &str) -> HashMap<String,String>{
    let content_str = str::from_utf8(file_content).expect("Invalid UTF-8 sequence");
    let regex_pattern = format!(r#"(?m)^#\s*(?:\./)?{}:(\d+)"#, regex::escape(filename));
    let re = Regex::new(&regex_pattern).unwrap();
    let mut map = HashMap::new();

    let mut iter = content_str.lines().peekable();  
    while let Some(line) = iter.next(){
        if let Some(cap) = re.captures(line){
            if let Some(line_number) = cap.get(1){
                let key = format!("{}:{}", filename, line_number.as_str());
                let mut instruction = Vec::new();
                while let Some(next_line) = iter.peek(){
                    if re.is_match(next_line){
                        break;
                    }
                    instruction.push(iter.next().unwrap().to_string());
                }
                // Append to existing value if the key already exists
                map.entry(key.clone()).and_modify(|e: &mut String| {
                    if !e.is_empty() {
                        e.push('\n');
                    }
                    e.push_str(&instruction.join("\n"));
                }).or_insert_with(|| instruction.join("\n"));

                // println!("key : {}", key.as_str());
                // println!("value : {:?}", instruction);
            }
        }
    }  
    map
}


// pub fn generate_hashmap_srcRefBlock(file_content: &[u8], _filename: &str) -> HashMap<String,String>{
//     let content_str = str::from_utf8(file_content).expect("Invalid UTF-8 sequence");
//     let regex_pattern = r#"(?m)^#\s*([\w/.\+-]+):(\d+):"#;
//     let re = Regex::new(&regex_pattern).unwrap();
//     let mut map = HashMap::new();

//     let mut iter = content_str.lines().peekable();  
//     while let Some(line) = iter.next() {
//         if let Some(cap) = re.captures(line) {
//             if let Some(file_path) = cap.get(1) {
//                 if let Some(line_number) = cap.get(2) {
//                     let key = format!("{}:{}", file_path.as_str(), line_number.as_str());
//                     let mut instruction = Vec::new();
//                     while let Some(next_line) = iter.peek() {
//                         if re.is_match(next_line) {
//                             break;
//                         }
//                         instruction.push(iter.next().unwrap().to_string());
//                        // println!("{:?}\n", next_line);
//                     }
//                     // Append to existing value if the key already exists
//                     map.entry(key.clone()).and_modify(|e: &mut String| {
//                         if !e.is_empty() {
//                             e.push('\n');
//                         }
//                         e.push_str(&instruction.join("\n"));
//                     }).or_insert_with(|| instruction.join("\n"));

//                     // println!("key : {}", key.as_str());
//                     // println!("value : {:?}", instruction);
//                 }
//             }
//         }
//     }  
//     map
// }

pub fn generate_hashmap_srcRefBlock(file_content: &[u8], _filename: &str) -> HashMap<String, String> {
    let content_str = std::str::from_utf8(file_content).expect("Invalid UTF-8 sequence");
    let regex_pattern = r#"(?m)^#\s*([\w/.\+-]+):(\d+):"#;
    let re = Regex::new(&regex_pattern).unwrap();
    let mut map = HashMap::new();

    let mut iter = content_str.lines().peekable();
    while let Some(line) = iter.next() {
        if let Some(cap) = re.captures(line) {
            if let Some(file_path) = cap.get(1) {
                if let Some(line_number) = cap.get(2) {
                    let key = format!("{}:{}", file_path.as_str(), line_number.as_str());
                    let mut instruction = Vec::new();
                    
                    while let Some(next_line) = iter.peek() {
                        if re.is_match(next_line) {
                            break; // Stop if a new key pattern is found
                        }
                        
                        let inst = iter.next().unwrap().split_whitespace().next().unwrap_or("").to_string();
                        
                        // Stop collecting instructions if "main:" is found
                        if inst == "main:" {
                            break;
                        }

                        // Ignore lines that start with a dot, number, or underscore
                        if !(inst.starts_with('.') || inst.starts_with(char::is_numeric) || inst.starts_with('_')) {
                            instruction.push(inst);
                        }
                    }

                    // Append to existing value if the key already exists
                    map.entry(key.clone()).and_modify(|e: &mut String| {
                        if !e.is_empty() {
                            e.push('\n');
                        }
                        e.push_str(&instruction.join("\n"));
                    }).or_insert_with(|| instruction.join("\n"));
                }
            }
        }
    }
    map
}