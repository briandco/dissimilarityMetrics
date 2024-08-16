use core::str;
use regex::Regex;
use std::{collections::HashMap, fs::File, io::Read, io::Result};

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

                println!("key : {}", key.as_str());
                println!("value : {:?}", instruction);
            }
        }
    }  
    map
}


pub fn generate_hashmap_srcRefBlock(file_content: &[u8], _filename: &str) -> HashMap<String,String>{
    let content_str = str::from_utf8(file_content).expect("Invalid UTF-8 sequence");
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

                    println!("key : {}", key.as_str());
                    println!("value : {:?}", instruction);
                }
            }
        }
    }  
    map
}