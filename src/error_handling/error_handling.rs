use std::fs;

fn main() {
    let my_content = getFileContent("my_file.txt");

    match  my_content {
        Ok(item) => println!("The result is {}", item),
        Err(_) => println!("We got an Error."),
    }
}

fn getFileContent(file_name: &str) -> Result<String, std::io::Error> {
    let content = fs::read_to_string(file_name)?;
    Ok(content)
}

fn divide(numerator: f64, denominator: f64) -> Option<f64> {
    if(denominator == 0.0){
        None
    } else {
        Some(numerator / denominator)
    }
}