use std::collections::HashMap;

use ndarray::Array1;

//function to calculate cosine similarity
pub fn cosine_similarity(str1: &str, str2: &str) -> f64{

    // Convert strings to bytes (Vec<u8>)
    let vec1: Vec<u8> = str1.bytes().collect();
    let vec2: Vec<u8> = str2.bytes().collect();

    // Convert bytes to Array1<f64> for numerical operations
    let arr1 = Array1::from(vec1.iter().map(|&x| x as f64).collect::<Vec<_>>());
    let arr2 = Array1::from(vec2.iter().map(|&x| x as f64).collect::<Vec<_>>());

    // Calculate dot product
    let dot_product = arr1.dot(&arr2);

    // Calculate norms
    let norm_arr1 = arr1.mapv(|x| x.powi(2)).sum().sqrt();
    let norm_arr2 = arr2.mapv(|x| x.powi(2)).sum().sqrt();

    // Return cosine similarity
    dot_product / (norm_arr1 * norm_arr2)
}



//function to pad two vector to same length
pub fn pad_vector(vec1: Vec<u8>, vec2: Vec<u8>)-> (Vec<u8>,Vec<u8>){

    let len1 = vec1.len();
    let len2 = vec2.len();

    if len1 > len2 {
        let mut vec2_cloned = vec2.clone();
        vec2_cloned.resize(len1, 0);
        return (vec1,vec2_cloned);
    }
    else if len2 > len1{
        let mut vec1_cloned = vec1.clone();
        vec1_cloned.resize(len2, 0);
        return (vec1_cloned, vec2)
    }
    else{
        return (vec1,vec2)
    }  
}

pub fn pad_strings(str1: &str, str2: &str) -> (String, String) {
    // Convert the input strings to vectors of u8
    let mut vec1: Vec<u8> = str1.as_bytes().to_vec();
    let mut vec2: Vec<u8> = str2.as_bytes().to_vec();

    let len1 = vec1.len();
    let len2 = vec2.len();

    // Pad the shorter vector with 0 (which corresponds to '\0' in a string)
    if len1 > len2 {
        vec2.resize(len1, b'\0');
    } else if len2 > len1 {
        vec1.resize(len2, b'\0');
    }

    // Convert the padded vectors back to strings
    let padded_str1 = String::from_utf8(vec1).expect("Invalid UTF-8 in vec1");
    let padded_str2 = String::from_utf8(vec2).expect("Invalid UTF-8 in vec2");

    (padded_str1, padded_str2)
}