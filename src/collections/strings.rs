fn main() {
    
    // String Object creation
    create_string();

    // Slicing and Indexing
    str_slicing_indexing();
}

fn create_string() {
    // Create a immutable string and try to append with a text
    // This should throw a compilation error about immutable
    // error: cannot mutate immutable variable `hello`

    // now add mut to hello, the error should go away
    let mut hello = String::from("Hello, ");
    println!("{}", hello);
    hello.push_str("world!");
    println!("{}", hello);
}

fn str_slicing_indexing() {
    // Now lets experiment with String slices.
    // Strings are of 2 Types
    // 1 - String
    // 2- &str (borrowed slice)
    // Strings are encoded in UTF-8
    let example = String::from("hello");
    let slice = &example[0..2];
    println!("{}",slice);

    let text = "こんにちは";

    for c in text.chars() {
        println!("{}", c);
    }

    // Iterate overs characters
    println!("Iterate overs characters");
    for c in example.chars() {
        println!("{}",c);
    }

    // iterate over bytes
    println!("iterate over bytes");
    for b in example.bytes() {
        println!("{}", b);
    }
}