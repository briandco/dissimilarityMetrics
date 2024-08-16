#![feature(lazy_cell)]

mod parser;
mod levenstein;
use std::collections::HashMap;

use parser::*;
use levenstein::*;
use regex::bytes;

fn main() -> std::io::Result<()> {
    // let file1 = "/home/cbq2kor/Desktop/DevSpace/Test/RDS/C/Cpp/Multiplication/operation_fverb_o1.asm";
    // let file2 = "/home/cbq2kor/Desktop/DevSpace/Test/RDS/C/Cpp/Multiplication/operation_fverb_o2.asm";

    let file1 = "/home/cbq2kor/Desktop/DevSpace/Test/RDS/C/Cpp/Multiplication/math_O1_fv.asm";
    let file2 = "/home/cbq2kor/Desktop/DevSpace/Test/RDS/C/Cpp/Multiplication/math_O2_fv.asm";

    // let file1 = "/home/cbq2kor/Desktop/DevSpace/Test/RDS/Files/file1.txt";
    // let file2 = "/home/cbq2kor/Desktop/DevSpace/Test/RDS/Files/file2.txt";
    let file_block = FileBlock::new(file1);
    let data_file1 = file_block.read_binary_file()?;
    let file_name = file_block.clone().get_file_name(&data_file1).ok_or("Test").unwrap();
    // println!("filename {}", file_name);
    file_block.generate_hashmap(&data_file1, &file_name);

    let data1 = read_binary_file(file1)?;
    let data2 = read_binary_file(file2)?;

    let file_name1 = extract_file_name(&data1);
    let file_name2 = extract_file_name(&data2);

    let mut map1 = HashMap::new();
    let mut map2 = HashMap::new();

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

    // if let Some(file_name1) = &file_name1 {
    //     map1 = generate_hashmap(&data1, file_name1);
    //     println!("Map from file1: {:?}", map1);
    // }
    
    // if let Some(file_name2) = &file_name2 {
    //     map2 = generate_hashmap(&data2, file_name2);
    //     println!("Map from file2: {:?}", map2);
    // }

    if let Some(file_name1) = &file_name1 {
        map1 = generate_hashmap_srcRefBlock(&data1, file_name1);
        println!("Map from file1: {:?}", map1);
    }
    
    if let Some(file_name2) = &file_name2 {
        map2 = generate_hashmap_srcRefBlock(&data2, file_name2);
        println!("Map from file2: {:?}", map2);
    }

    println!("\n");

    let mut total_normalized_distance = 0.0;
    let mut count = 0;

    for(key, value1) in &map1{
        if let Some( value2 ) = map2.get(key){
            let distance = optimized_levenshtein(value1.as_bytes(), value2.as_bytes());
            let max_distance = value1.len().max(value2.len());
            let mut normalized_distance = 0.0;
            if max_distance > 0 {
                normalized_distance = distance as f64 / max_distance as f64;
                total_normalized_distance += normalized_distance;
                count +=1;
            }

            println!("Key: {}, Levenshtein distance: {}, normalized distance: {}", key, distance, normalized_distance);
            let max_file_len = value1.len().max(value2.len()) as f64;
            let change_percent = (distance as f64 / max_file_len) * 100.0;
            println!("Change % = {}", change_percent);
        }
        else {
            println!("Key: {} does not exist in map2", key);
            total_normalized_distance += value1.len() as f64;
            count +=1;
        }
   }

    for (key, value) in &map2 {
        if !map1.contains_key(key){
            println!("Key: {} does not exist in map1", key);
            total_normalized_distance += value.len() as f64;
            count +=1;
        }
    }

    if count > 0 {
        let average_normalized_distance = total_normalized_distance / count as f64;
        println!("Average normalized Levenshtein distance: {}", average_normalized_distance);
    } else {
        println!("No matching keys found to compute Levenshtein distance.");
    }

    Ok(())
}
