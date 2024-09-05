fn main() {
    // call my_vectors function
    vec_macro();
    //Call vector_new, features uses creation of vector using Vec object
    vector_new();
}

fn vec_macro() {
    // Create a Vector using a macro (vec!)
    let numbers = vec![1,2,3,4,5];
    // accessing elements
    // lets say access index 1 which is value 2 in teh above collection
    let index1 = &numbers[1];
    println!("Value of index 1 element is {}", index1);
}

fn vector_new() {
    // Create a Vector using Vector object
    let mut names: Vec<String> = Vec::new();
    // Now add elements into names vector
    names.push(String::from("Ram"));
    names.push(String::from("James"));

    // now access the first element of the vector
    let first_element = &names[0];
    println!("First Element is {}",first_element);

    // now access elements in a loop
    for name in &names {
        println!("Name : {}", name);
    }
}