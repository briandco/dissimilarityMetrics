use core::str;
use regex::Regex;
use std::{collections::{hash_map, HashMap}, fs::File, io::Read};

fn read_binary_file(file_path: &str) -> std::io::Result<Vec<u8>> {
    let mut file = File::open(file_path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    Ok(buffer)
}

fn extract_file_name(file_content: &[u8]) -> Option<String> {
    let content_str = str::from_utf8(file_content).ok()?;
    let re = Regex::new(r#"\.file\s+"([^"]+)""#).unwrap();
    re.captures(content_str)
        .and_then(|cap| cap.get(1).map(|m| m.as_str().to_string()))
}

fn generate_hashmap(file_content: &[u8], filename: &str) -> HashMap<String,String>{
    let content_str = str::from_utf8(file_content).expect("Invalid UTF-8 sequence");
    let regex_pattern = format!(r#"(?m)^#\s*{}:(\d+)"#, regex::escape(filename));
    let re = Regex::new(&regex_pattern).unwrap();
    let mut map = HashMap::new();

    for cap in re.captures_iter(content_str){
        if let Some(line) = cap.get(1){
            let key = format!("{}:{}",filename,line.as_str());
            map.insert(key, filename.to_string());
        }
    }
    map
}

fn main() -> std::io::Result<()> {
    let file1 = "/home/cbq2kor/Desktop/DevSpace/Test/RDS/C/Cpp/Multiplication/math_O2_fv.asm";
    let file2 = "/home/cbq2kor/Desktop/DevSpace/Test/RDS/C/Cpp/Multiplication/math_O1_fv.asm";

    let data1 = read_binary_file(file1)?;
    let data2 = read_binary_file(file2)?;

    let file_name1 = extract_file_name(&data1);
    let file_name2 = extract_file_name(&data2);

    match (file_name1.as_deref(), file_name2.as_deref()) {
        (Some(name1), Some(name2)) => {
            if name1 == name2 {
                println!("File name = {}", name1);
            } else {
                println!("Files do not match: {} != {}", name1, name2);
            }
        }
        (None, None) => println!("File names not found in either files"),
        (None, Some(_)) => println!("File1 name not found"),
        (Some(_), None) => println!("File2 name not found"),
    }

    if let Some(file_name1) = &file_name1 {
        let map1 = generate_hashmap(&data1, file_name1);
        println!("Map from file1: {:?}", map1);
    }

    if let Some(file_name2) = &file_name2 {
        let map2 = generate_hashmap(&data2, file_name2);
        println!("Map from file2: {:?}", map2);
    }

    Ok(())
}
