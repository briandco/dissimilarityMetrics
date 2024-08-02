mod parser;
use parser::*;

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
