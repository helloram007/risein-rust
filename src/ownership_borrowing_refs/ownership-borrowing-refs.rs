#[warn(unused_imports)]
use core::num;

fn main() {
    // Define a variable
    let message = "Hello, world!";
    println!("{message}");

    //Define Built-in Data Types

    // Define an integer signed
    let x: i32 = -42;
    println!("signed negative i32 : {}", x);
    let x: i32 = 52;
    println!("signed postive i32 : {}", x);
    let y: i32 = 52;
    let sum = add(x, y);
    println!("Sum of {}, {}, {}", x, y, sum);
    //Define an unsigned integer
    let y: u32 = 100;
    println!("unsigned integer: {}", y);

    // Define a floating number
    let pi: f64 = 3.14159;
    println!("Pi floating number {}", pi);

    // define a Boolean 
    let is_rust_fun: bool = true;
    println!("is Rust Fun: {}", is_rust_fun);

    // define a char
    let letter_a: char = 'a';
    println!("Char {}", letter_a);

    let name = String::from("Ram");
    let greeting = get_greeting(&name);
    println!("Greeting: {}", greeting);
    
    //Check with Some(n) Where some(n) is a type of option<&str>
    let greet_msg = Some("Ram");
    greet(greet_msg);

    // Now check with none
    let greet_none = None;
    greet(greet_none);

    // print conditional statements
    cond_statements();

    //while loops
    while_loop();

    //for loop
    for_loop();

    // loop block
    my_loop();

    // match statements
    match_me(4);

    //match_sum
    match_sum(3);

    // memory allocated
    ownership();

    // borrow and references
    borrow_and_references();

}

fn add(x: i32, y: i32) -> i32 {
    x + y
}

fn get_greeting(name: &str) -> String {
        let greeting = format!("Hello, {}!", name);
        return greeting;
}

fn greet(name: Option<&str> ) {
    match name {
        Some(n) => println!("Hello, {}!", n),
        None => println!("hello, rust!"),
    }
}

fn cond_statements() {
    let x = 5;

    if x > 10 {
        println!("x is greater than 10");
    } else if x < 10 {
        println!("x is less than 10");
    } else {
        println!("x is equal to 10");
    }
}

fn while_loop() {
    let mut counter = 0;

    while counter < 5 {
        println!("Counter value is {}", counter);
        counter += 1;
    }
}

fn for_loop() {

    let numbers = vec![1,2,3,4,5];

    for number in numbers {
        println!("Number is {}", number);
    }
}

fn my_loop(){
    
    let mut counter =0;
    loop {
        println!("Loop: Counter value is {}", counter);
        counter += 1;

        if counter == 5 {
            break;
        }    
    }
}

fn match_me(x: i32) {
    match x {
        1 => println!("The number is {}", x),
        2 => println!("The number is {}", x),
        3 => println!("The number is {}", x),
        4 => println!("The number is {}", x),
        _ => println!("The number is something else"),
    }
}

fn match_sum(x: i32) {
    
    let result = match x {
        1 => println!("The number is One!"),
        2 => println!("The number is Two!"),
        3 => println!("The number is Three!"),
        _ => println!("The number is something else"),
    };

    println!("Result : {:?}", result);
}

fn ownership() {
    // Store this on heap : Dynamically growing or Memory allocated at Runtime.
    let my_string = String::from("hello") ;
    println!("my_string: {}", my_string);

    // Store on Stack, fixed size allocation
    // e.g. i want to intialise a integer with 5
    let i: i32 = 5;
    println!("integer: {}", i);

    // move the owership from my_string to  next_str
    let next_str = my_string;
    println!("next str : {}", next_str);

    //This will error out if i try to print my_string.
    // will comment out after i see the error
    // Error: borrow of moved value: `my_string`
    //println!("my_string: {}", my_string);
}

fn borrow_and_references(){
    // In Rust, variables are immutable by default

    let my_string = String::from("hello, world!");
    println!("my string {}", my_string);
    print_str(&my_string);
    // Create a varialbe using a reference of my_string (&my_string)
    let my_ref = &my_string;
    println!("my ref : {}", my_ref);

    // try aliasing
    println!("==============Aliasing======================");
    aliasing();
    
}

fn print_str(s: &String) {
    println!("print str : {}", s);
}

fn aliasing() {

    // you can have multiple immutable references to the same value
    // at the same time. This is known as "aliasing"
    let my_string = String::from("mulitple immutable references example");

    // create multiple immutable references to my_string
    let my_ref_1 = &my_string;
    let my_ref_2 = &my_string;
    println!("my ref 1 : {}, my ref 2 : {}", my_ref_1, my_ref_2);

    println!("Trying to create 1 mutable reference to my_string");
    let mut my_string = String::from("mulitple immutable references example");
    let my_ref_1 = &my_string;
    let my_ref_2 = &my_string;
    
    let mut mutable_ref_1 = &mut my_string;

    println!("mutable referenec : {}", mutable_ref_1);
    //Commenting, because when a variable holds the mutable reference
    // it cannot hold the immutable reference at the same time.
    //println!("immutable reference 1 {}", my_ref_1);

    clone();
}

fn clone() {
    let original_str = String::from("Hello, World!");
    let cloned_str = original_str.clone();

    println!("Original String {}", original_str);
    println!("Cloned Str {}", cloned_str);

    let modified_str = modify_str(&cloned_str);
    println!("Modified Str {}", modified_str);
}

fn modify_str(s: &String) -> String {
    let mut cloned_string = s.clone();
    cloned_string.push_str(" modified");
    cloned_string
}