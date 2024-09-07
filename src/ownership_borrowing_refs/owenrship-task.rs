#[warn(unused_imports)]

fn main() {
    let string1 = String::from("hello");
    let string2 = String::from("world");
    let concatenated_string = concatenate_strings(&string1, &string2);
    println!("Concatenated String : {}", concatenated_string);
}

fn concatenate_strings (
    s1: &str,
    s2: &str
) -> String {
    let mut result = String::new();
    result.push_str(s1);
    result.push_str(", ");
    result.push_str(s2);
    result
}