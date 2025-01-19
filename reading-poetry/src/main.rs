use std::fs;

/// Helper function to get the path of a file in a project directory (`cargo run` compatible)
/// 
/// Example: 
/// ```
/// let file_path : String = collect_path(r"data\poem.txt".to_string());
/// ```
fn collect_path(file_name:String)->String{
    let mut current_dir: String =  std::env::current_dir().unwrap().display().to_string();
    current_dir.push_str(r"\");
    current_dir.push_str(file_name.trim());
    current_dir
}


fn main() {
    // declare and set file_path
    let file_path : String = collect_path(r"data\poem.txt".to_string());
    println!("File Path: {file_path}\n");
    // read the text file
    let content = fs::read_to_string(file_path).expect("Should have been able to read file");
    println!("{content}");
}
