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
pub struct MyApp {
    file1_path: Option<std::path::PathBuf>,
    file2_path: Option<std::path::PathBuf>,
    file1_name: Option<String>,
    file2_name: Option<String>,
    levenshtein_distance: i32,
    cosine_similarity: f64,
    change_percentage: f64,
    error_message: Option<String>,
    should_display_output: bool,
    file1_text: String,
    file2_text: String,
    file_length: f64,
    key_distance: i32,
    results: Vec<(String, i32, i32, f64, f64)>, // Add this field to store the results
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            file1_path: None,
            file2_path: None,
            file1_name: None,
            file2_name: None,
            levenshtein_distance: 0,
            cosine_similarity: 0.0,
            change_percentage:0.0,
            error_message: None,
            should_display_output: false,
            file1_text: String::new(),
            file2_text: String::new(),
            file_length: 0.0,
            key_distance: 0,
            results: Vec::new(), // Initialize the results field
        }
    }
}

impl MyApp {
    // Add a reset method that resets all values to their default state
    pub fn reset(&mut self) {
        *self = MyApp::default(); // Set all fields to default values
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
       
        let mut map1 = HashMap::new();
        let mut map2 = HashMap::new();

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
                                    self.file1_name = Some(name1.to_string());
                                    self.file2_name = Some(name2.to_string());
                                    println!("File names extracted: {} and {}", name1, name2);
                                }
                                _ => println!("One or both file names not found."),
                            }

                            // Generate hashmaps for both files
                            if let Some(file_name1) = &file_name1 {
                                (map1, self.file1_text) = generate_hashmap_srcRefBlock(&data1, file_name1);
                               println!("{:?}", self.file1_text);
                            }

                            if let Some(file_name2) = &file_name2 {
                                (map2, self.file2_text) = generate_hashmap_srcRefBlock(&data2, file_name2);
                               println!("{:?}", self.file2_text);
                            }

                            // Clear previous results
                            self.results.clear();

                            // Perform calculations and store the results
                            for (key, value1) in &map1 {
                                if let Some(value2) = map2.get(key) {
                                    let distance = optimized_levenshtein(value1.as_bytes(), value2.as_bytes());
                                    self.levenshtein_distance += distance as i32;

                                    let (padded_str1, padded_str2) = pad_strings(value1, value2);
                                    let similarity = cosine_similarity(&padded_str1, &padded_str2);

                                    let max_file_len = value1.len().max(value2.len()) as f64;
                                    self.file_length += max_file_len;
                                    let change_percent = (distance as f64 / max_file_len) * 100.0;
                                    
                                    // Store results in the vector
                                    self.results.push((
                                        key.clone(),
                                        distance as i32,
                                        max_file_len as i32,
                                        similarity,
                                        change_percent,
                                    ));
                                } else {
                                    // Handle missing key in map2
                                    self.results.push((
                                        key.clone(),
                                        -1, // Indicate missing value with a sentinel
                                        -1,
                                        -1.0,
                                        -1.0,
                                    ));
                                }
                            }

                            for (key, _value) in &map2 {
                                if !map1.contains_key(key) {
                                    // Handle missing key in map1
                                    self.results.push((
                                        key.clone(),
                                        -1, // Indicate missing value with a sentinel
                                        -1,
                                        -1.0,
                                        -1.0,
                                    ));
                                }
                            }

                            self.should_display_output = true; // Set the flag to true when calculation is done
                            
                            let (padded_str1, padded_str2) = pad_strings(&self.file1_text, &self.file2_text);
                            self.cosine_similarity = cosine_similarity(&padded_str1, &padded_str2);
                            //println!("Cosine of file {}",similarity);
                            self.change_percentage = (self.levenshtein_distance as f64 / self.file_length) * 100.0;
                            // println!("% change {}",change_percent);
                            println!("Calculation complete.");
                        }
                        _ => {
                            self.error_message = Some("Failed to read one or both files.".to_string());
                            println!("Failed to read files.");
                        }
                    }
                } else {
                    self.error_message = Some("Please select both files.".to_string());
                    println!("Files not selected.");
                }
            }

            // Add space after the Calculate button
            ui.add_space(10.0);

            // Display Results
            match (&self.file1_name, &self.file2_name) {
                (None, None) => {
                    //ui.label("No file names found".to_string());
                }
                (None, Some(file2_name)) => {
                    ui.label(format!("File 1 name not found, but File 2 name is: {}", file2_name));
                }
                (Some(file1_name), None) => {
                    ui.label(format!("File 2 name not found, but File 1 name is: {}", file1_name));
                }
                (Some(file1_name), Some(file2_name)) => {
                    if file1_name == file2_name {
                        ui.label(format!("File name: {}", file1_name));
                    } else {
                        ui.label(format!("Files do not match: {} != {}", file1_name, file2_name));
                    }
                }
            }

            // Add space after displaying file names
            ui.add_space(10.0);

            if let Some(error_message) = &self.error_message {
                ui.label(format!("Error: {}", error_message));
            }

            // Add space before the results table
            ui.add_space(10.0);

            // Display the results in a table if the flag is set
            if self.should_display_output {
                // First table with summary results
                egui::Grid::new("summary_table")
                    .striped(true)
                    .min_col_width(150.0)
                    .show(ui, |ui| {
                        ui.label("File Name");
                        ui.label("Levenshtein Distance");
                        ui.label("Cosine Similarity");
                        ui.label("Change %");
                        ui.end_row();

                        // Display file 1 results
                        ui.label(self.file1_name.as_deref().unwrap_or("File 1 name not found"));
                        ui.label(format!("{}", self.levenshtein_distance));
                        ui.label(format!("{:.2}", self.cosine_similarity));
                        ui.label(format!("{:.2}%", self.change_percentage));
                        ui.end_row();
                    });

                // Add space between the tables
                ui.add_space(20.0);

                egui::Grid::new("dissimilarity_metrics_table")
                    .striped(true)
                    .min_col_width(100.0)
                    .show(ui, |ui| {
                        ui.label("Key");
                        ui.label("Levenshtein Distance");
                        ui.label("Cosine Similarity");
                        ui.label("Block Length");
                        ui.label("Change %");
                        ui.end_row();

                        for (key, distance,block_length, similarity, change_percent) in &self.results {
                            if *distance != -1 {
                                ui.label(key);
                                ui.label(format!("{}", distance));
                                ui.label(format!("{:.2}", similarity));
                                ui.label(format!("{:.2}", block_length));
                                ui.label(format!("{:.2}%", change_percent));
                            } else {
                                ui.label(key);
                                ui.label("N/A");
                                ui.label("N/A");
                                ui.label("N/A");
                                ui.label("N/A");
                            }
                            ui.end_row();
                        }
                    });
                    
            }
            
        });
    }
}







fn main() -> std::io::Result<()> {
    let options = eframe::NativeOptions::default();
    let _ = eframe::run_native(

        "Disimilarity Metrics",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    );
//     let file1 = "/home/cbq2kor/Desktop/DevSpace/Test/RDS/C/Cpp/Multiplication/operation_fverb_o1.asm";
//     let file2 = "/home/cbq2kor/Desktop/DevSpace/Test/RDS/C/Cpp/Multiplication/operation_fverb_o2.asm";

//     // let file1 = "/home/cbq2kor/Desktop/DevSpace/Test/RDS/C/Cpp/Multiplication/math_O1_fv.asm";
//     // let file2 = "/home/cbq2kor/Desktop/DevSpace/Test/RDS/C/Cpp/Multiplication/math_O2_fv.asm";

//     let data1 = read_binary_file(file1)?;
//     let data2 = read_binary_file(file2)?;

//     let file_name1 = extract_file_name(&data1);
//     let file_name2 = extract_file_name(&data2);

//     let mut map1 = HashMap::new();
//     let mut map2 = HashMap::new();

//     match (file_name1.as_deref(), file_name2.as_deref()) {
//         (Some(name1), Some(name2)) => {
//             if name1 == name2 {
//                 println!("File name = {}", name1);
//             } else {
//                 println!("Files do not match: {} != {}", name1, name2);
//             }
//         }
//         (None, None) => println!("File names not found in either files"),
//         (None, Some(_)) => println!("File1 name not found"),
//         (Some(_), None) => println!("File2 name not found"),
//     }

//     if let Some(file_name1) = &file_name1 {
//         map1 = generate_hashmap_srcRefBlock(&data1, file_name1);
//         println!("Map from file1: {:?}", map1);
//     }
    
//     if let Some(file_name2) = &file_name2 {
//         map2 = generate_hashmap_srcRefBlock(&data2, file_name2);
//        println!("Map from file2: {:?}", map2);
//     }

//     println!("\n");

//     for(key, value1) in &map1{
//         if let Some( value2 ) = map2.get(key){
//             let distance = optimized_levenshtein(value1.as_bytes(), value2.as_bytes());

//             let (padded_str1, padded_str2) = pad_strings(value1, value2);
//             let similarity = cosine_similarity(&padded_str1, &padded_str2);

//             println!("Key: {}, Levenshtein distance: {} , cosine similarity: {}", key, distance, similarity);
//             let max_file_len = value1.len().max(value2.len()) as f64;
//             let change_percent = (distance as f64 / max_file_len) * 100.0;
//             println!("Change % = {}", change_percent);
//         }
//         else {
//             println!("Key: {} does not exist in map2", key);
//         }
//    }

//     for (key, _value) in &map2 {
//         if !map1.contains_key(key){
//             println!("Key: {} does not exist in map1", key);
//         }
//     }
    
    Ok(())
}
