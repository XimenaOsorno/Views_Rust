use std::fs; 
pub fn read_file(file_path: &str) -> String { 
    let data: String = fs::read_to_string( 
        file_path).expect("Unable to read file"); 
    return data 
}