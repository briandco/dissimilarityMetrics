use std::{collections::HashMap, hash};

use regex::bytes;

use crate::SuperBlock;

pub trait Levenstein {
    fn calculate_levinstine(sb1:SuperBlock, sb2:SuperBlock);
}

pub fn optimized_levenshtein(a: &[u8], b: &[u8]) -> usize {
    let len_a = a.len();
    let len_b = b.len();

    if len_a == 0 {
        return len_b;
    }
    if len_b == 0 {
        return len_a ;
    }

    let mut prev_row = (0..=len_b).collect::<Vec<usize>>();
    let mut curr_row = vec![0; (len_b + 1).try_into().unwrap()];

    for i in 1..=len_a {
        curr_row[0] = i;
        for j in 1..=len_b {
            let cost = if a[i - 1] == b[j - 1] { 0 } else { 1 };
            curr_row[j] = *[
                prev_row[j] + 1,          // deletion
                curr_row[j - 1] + 1,      // insertion
                prev_row[j - 1] + cost,   // substitution
            ].iter().min().unwrap();
        }
        std::mem::swap(&mut prev_row, &mut curr_row);
    }
    prev_row[len_b]
}

pub fn calculate_levinstine(str_v1: Vec<String>, str_v2:Vec<String>) -> u32{
    let mut distance = 0;
    for (s1, s2) in str_v1.iter().zip(str_v2.iter()){
        distance += optimized_levenshtein(s1.as_bytes(), s2.as_bytes());
        let max_distance = s1.len().max(s2.len());
    }
    distance as u32
}

// impl Levenstein for SuperBlock{
//     fn calculate_levinstine(sb1:SuperBlock, sb2:SuperBlock) {
//         let mut hashmap:HashMap<String, u32> = HashMap::new();
//         let mut l_distance = 0;
//         for (key, src_ref) in sb1.src_references{
//             if let Some(src_ref2) = sb2.src_references.get(&key){
//                 l_distance = calculate_levinstine(src_ref.content, src_ref2.content.clone());
//                 hashmap.insert(key, l_distance);
//                 // println!("{:?}", hashmap);
//             }
//             else{
//                 println!("src_reference {:?} does not exist", &key);
//             }
//         }
//         hashmap
//     }
// }

pub fn calculate_levinstine_super_block(sb1: SuperBlock, sb2: SuperBlock) -> HashMap<String, u32>{
    let mut hashmap:HashMap<String, u32> = HashMap::new();
    let mut l_distance = 0;
    for (key, src_ref) in sb1.src_references{
        if let Some(src_ref2) = sb2.src_references.get(&key){
            l_distance = calculate_levinstine(src_ref.content, src_ref2.content.clone());
            hashmap.insert(key, l_distance);
            // println!("{:?}", hashmap);
        }
        else{
            println!("src_reference {:?} does not exist", &key);
        }
    }
    hashmap
}

pub fn generate_levinstine_map(sb1: HashMap<String, SuperBlock>, sb2: HashMap<String, SuperBlock>) -> HashMap<String, i32>{
    for(key, value1) in sb1{
        if let Some( value2 ) = sb2.get(&key){
            calculate_levinstine_super_block(value1, value2.clone());
        }
    }
    let mut hash_map:HashMap<String, i32> = HashMap::new();
    hash_map
}   