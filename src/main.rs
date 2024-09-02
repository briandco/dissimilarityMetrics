#![feature(lazy_cell)]

mod parser;
mod levenstein;
mod cosine_similarties;
use std::{collections::HashMap, path::PathBuf};

use parser::*;
use levenstein::*;
use cosine_similarties::*;

use eframe::egui;
use rfd::FileDialog;

// struct MyApp{
//     file1_path : Option<PathBuf>,
//     file2_path : Option<PathBuf>,
//     file1_name : Option<String>,
//     file2_name : Option<String>,
//     levenshtein_distance : Option<usize>,
//     error_message : Option<String>,
//     output_text : String,
// }

// impl Default for MyApp{
//     fn default() -> Self {
//         Self{
//             file1_path : None,
//             file2_path : None,
//             file1_name : None,
//             file2_name : None,
//             levenshtein_distance : None,
//             error_message : None,
//             output_text :String::new(),
            
//         }
//     }   
// }

// impl eframe::App for MyApp{
//     fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
//         let mut map1 = HashMap::new();
//         let mut map2 = HashMap::new();
//         egui::CentralPanel::default().show(ctx, |ui| {
//             ui.heading("Dissimilarity Metrics Calculator");

//             // File 1 selection
//             if ui.button("Select File 1").clicked() {
//                 if let Some(path) = FileDialog::new().pick_file() {
//                     self.file1_path = Some(path.clone());
//                     ui.label(format!("File 1: {}", path.display()));
//                 }
//             }

//             if let Some(path) = &self.file1_path {
//                 ui.label(format!("File 1: {}", path.display()));
//             }

//             ui.separator();

//             // File 2 selection
//             if ui.button("Select File 2").clicked() {
//                 if let Some(path) = FileDialog::new().pick_file() {
//                     self.file2_path = Some(path.clone());
//                     ui.label(format!("File 2: {}", path.display()));
//                 }
//             }

//             if let Some(path) = &self.file2_path {
//                 ui.label(format!("File 2: {}", path.display()));
//             }

//             ui.separator();

//             // Calculate Levenshtein Distance
//             if ui.button("Calculate Levenshtein Distance").clicked() {
                
//                 if let (Some(file1_path), Some(file2_path)) = (&self.file1_path, &self.file2_path) {
//                     match (read_binary_file(file1_path.to_str().unwrap()), read_binary_file(file2_path.to_str().unwrap())) {
//                         (Ok(data1), Ok(data2)) => {
//                             let file_name1 = extract_file_name(&data1);
//                             let file_name2 = extract_file_name(&data2);
//                             match (file_name1.as_deref(), file_name2.as_deref()) {
//                                 (Some(name1), Some(name2)) => {
//                                     // if name1 == name2 {
//                                     //     self.file1_name = file_name1.clone();
                                        
//                                     //     //println!("File name = {}", name1);
//                                     // } else {
//                                     //     self.file1_name = file_name1.clone();
//                                     //     self.file2_name = file_name2.clone();
//                                     //    // println!("Files do not match: {} != {}", name1, name2);
//                                     // }
//                                     self.file1_name = file_name1.clone();
//                                     self.file2_name = file_name2.clone();
//                                 }
//                                 (None, None) => {self.file1_name = None;
//                                     self.file2_name = None;},
//                                 (None, Some(_)) => {self.file1_name = None;
//                                     self.file2_name = file_name2.clone();},//println!("File1 name not found"),
//                                 (Some(_), None) => {self.file1_name = file_name1.clone();
//                                     self.file2_name = None;},//println!("File2 name not found"),
//                             }
//                             // self.levenshtein_distance = Some(optimized_levenshtein(&data1, &data2));
//                             // self.error_message = None;
//                         }
//                         _ => {
//                             self.levenshtein_distance = None;
//                             self.error_message = Some("Failed to read one or both files.".to_string());
//                         }
//                     }
//                 } else {
//                     self.error_message = Some("Please select both files.".to_string());
//                 }
//             }

            

//             // Display Results
//             match (&self.file1_name, &self.file2_name) {
//                 (None, None) => {
//                     ui.label("No file names found".to_string());
//                 }
//                 (None, Some(file2_name)) => {
//                     ui.label(format!("File 1 name not found, but File 2 name is: {}", file2_name));
//                 }
//                 (Some(file1_name), None) => {
//                     ui.label(format!("File 2 name not found, but File 1 name is: {}", file1_name));
//                 }
//                 (Some(file1_name), Some(file2_name)) => {
//                     if file1_name == file2_name
//                     {
//                         ui.label(format!("File name: {}", file1_name));
//                     }
//                     else {
//                         ui.label(format!("Files do not match: {} != {}", file1_name, file2_name));
//                     } 
//                 }
//             }

//             if let Some(file1_name) = &self.file1_name {
//                 ui.label(format!("Levenshtein Distance: {}", file1_name));
//             }
            
//             if let Some(distance) = self.levenshtein_distance {
//                 ui.label(format!("Levenshtein Distance: {}", distance));
//             }

//             if let Some(error_message) = &self.error_message {
//                 ui.label(format!("Error: {}", error_message));
//             }
//         });
//     }
// }
struct MyApp {
    file1_path: Option<PathBuf>,
    file2_path: Option<PathBuf>,
    file1_name: Option<String>,
    file2_name: Option<String>,
    levenshtein_distance: Option<usize>,
    error_message: Option<String>,
    output_text: String,
    should_display_output: bool, // Added this flag to persist display state
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            file1_path: None,
            file2_path: None,
            file1_name: None,
            file2_name: None,
            levenshtein_distance: None,
            error_message: None,
            output_text: String::new(),
            should_display_output: false, // Initialize to false
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let mut map1 = HashMap::new();
        let mut map2 = HashMap::new();
        let mut output_text = String::new();

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Dissimilarity Metrics Calculator");

            // File 1 selection
            if ui.button("Select File 1").clicked() {
                if let Some(path) = FileDialog::new().pick_file() {
                    self.file1_path = Some(path.clone());
                    ui.label(format!("File 1: {}", path.display()));
                }
            }

            if let Some(path) = &self.file1_path {
                ui.label(format!("File 1: {}", path.display()));
            }

            ui.separator();

            // File 2 selection
            if ui.button("Select File 2").clicked() {
                if let Some(path) = FileDialog::new().pick_file() {
                    self.file2_path = Some(path.clone());
                    ui.label(format!("File 2: {}", path.display()));
                }
            }

            if let Some(path) = &self.file2_path {
                ui.label(format!("File 2: {}", path.display()));
            }

            ui.separator();

            // Calculate Levenshtein Distance
            if ui.button("Calculate Levenshtein Distance").clicked() {
                println!("Calculate button clicked!");

                if let (Some(file1_path), Some(file2_path)) = (&self.file1_path, &self.file2_path) {
                    println!("Both files selected.");

                    match (read_binary_file(file1_path.to_str().unwrap()), read_binary_file(file2_path.to_str().unwrap())) {
                        (Ok(data1), Ok(data2)) => {
                            println!("Files read successfully.");

                            let file_name1 = extract_file_name(&data1);
                            let file_name2 = extract_file_name(&data2);

                            match (file_name1.as_deref(), file_name2.as_deref()) {
                                (Some(name1), Some(name2)) => {
                                    self.file1_name = file_name1.clone();
                                    self.file2_name = file_name2.clone();
                                    println!("File names extracted: {} and {}", name1, name2);
                                }
                                _ => println!("One or both file names not found."),
                            }

                            // Generate hashmaps for both files
                            if let Some(file_name1) = &file_name1 {
                                map1 = generate_hashmap_srcRefBlock(&data1, file_name1);
                            }

                            if let Some(file_name2) = &file_name2 {
                                map2 = generate_hashmap_srcRefBlock(&data2, file_name2);
                            }

                            // Perform calculations and collect the results in output_text
                            output_text.push_str("\n");

                            for (key, value1) in &map1 {
                                if let Some(value2) = map2.get(key) {
                                    let distance = optimized_levenshtein(value1.as_bytes(), value2.as_bytes());

                                    let (padded_str1, padded_str2) = pad_strings(value1, value2);
                                    let similarity = cosine_similarity(&padded_str1, &padded_str2);

                                    output_text.push_str(&format!(
                                        "Key: {}, Levenshtein distance: {}, cosine similarity: {}\n",
                                        key, distance, similarity
                                    ));
                                    let max_file_len = value1.len().max(value2.len()) as f64;
                                    let change_percent = (distance as f64 / max_file_len) * 100.0;
                                    output_text.push_str(&format!("Change % = {}\n", change_percent));
                                } else {
                                    output_text.push_str(&format!("Key: {} does not exist in map2\n", key));
                                }
                            }

                            for (key, _value) in &map2 {
                                if !map1.contains_key(key) {
                                    output_text.push_str(&format!("Key: {} does not exist in map1\n", key));
                                }
                            }

                            // Store the complete output in self.output_text
                            self.output_text = output_text.clone();
                            self.should_display_output = true; // Set the flag to true when calculation is done
                            println!("Calculation complete.");
                        }
                        _ => {
                            self.levenshtein_distance = None;
                            self.error_message = Some("Failed to read one or both files.".to_string());
                            println!("Failed to read files.");
                        }
                    }
                } else {
                    self.error_message = Some("Please select both files.".to_string());
                    println!("Files not selected.");
                }
            }

            // New "Get Result" button
            if ui.button("Get Result").clicked() {
                self.should_display_output = true; // Set the flag when the button is clicked
            }

            // Display the complete output text if the flag is set
            if self.should_display_output {
                ui.label(output_text);
            }

            // // Display Results
            // match (&self.file1_name, &self.file2_name) {
            //     (None, None) => {
            //         ui.label("No file names found".to_string());
            //     }
            //     (None, Some(file2_name)) => {
            //         ui.label(format!("File 1 name not found, but File 2 name is: {}", file2_name));
            //     }
            //     (Some(file1_name), None) => {
            //         ui.label(format!("File 2 name not found, but File 1 name is: {}", file1_name));
            //     }
            //     (Some(file1_name), Some(file2_name)) => {
            //         if file1_name == file2_name {
            //             ui.label(format!("File name: {}", file1_name));
            //         } else {
            //             ui.label(format!("Files do not match: {} != {}", file1_name, file2_name));
            //         }
            //     }
            // }

            // if let Some(distance) = self.levenshtein_distance {
            //     ui.label(format!("Levenshtein Distance: {}", distance));
            // }

            // if let Some(error_message) = &self.error_message {
            //     ui.label(format!("Error: {}", error_message));
            // }
        });
    }
}





fn main() -> std::io::Result<()> {
    // let options = eframe::NativeOptions::default();
    // let _ = eframe::run_native(
    //     "File Reader GUI",
    //     options,
    //     Box::new(|_cc| Box::new(MyApp::default())),
    // );
    let file1 = "/home/cbq2kor/Desktop/DevSpace/Test/RDS/C/Cpp/Multiplication/operation_fverb_o1.asm";
    let file2 = "/home/cbq2kor/Desktop/DevSpace/Test/RDS/C/Cpp/Multiplication/operation_fverb_o2.asm";

    // let file1 = "/home/cbq2kor/Desktop/DevSpace/Test/RDS/C/Cpp/Multiplication/math_O1_fv.asm";
    // let file2 = "/home/cbq2kor/Desktop/DevSpace/Test/RDS/C/Cpp/Multiplication/math_O2_fv.asm";

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

    if let Some(file_name1) = &file_name1 {
        map1 = generate_hashmap_srcRefBlock(&data1, file_name1);
       // println!("Map from file1: {:?}", map1);
    }
    
    if let Some(file_name2) = &file_name2 {
        map2 = generate_hashmap_srcRefBlock(&data2, file_name2);
      //  println!("Map from file2: {:?}", map2);
    }

    println!("\n");

    for(key, value1) in &map1{
        if let Some( value2 ) = map2.get(key){
            let distance = optimized_levenshtein(value1.as_bytes(), value2.as_bytes());

            let (padded_str1, padded_str2) = pad_strings(value1, value2);
            let similarity = cosine_similarity(&padded_str1, &padded_str2);

            println!("Key: {}, Levenshtein distance: {} , cosine similarity: {}", key, distance, similarity);
            let max_file_len = value1.len().max(value2.len()) as f64;
            let change_percent = (distance as f64 / max_file_len) * 100.0;
            println!("Change % = {}", change_percent);
        }
        else {
            println!("Key: {} does not exist in map2", key);
        }
   }

    for (key, _value) in &map2 {
        if !map1.contains_key(key){
            println!("Key: {} does not exist in map1", key);
        }
    }
    
    Ok(())
}
