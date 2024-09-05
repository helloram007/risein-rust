use std::collections::HashMap;

fn main() {
    create_hashmap();
}

fn create_hashmap() {
    // Hashmaps is a collection that associates a key with a value.
    // it uses a hashing function to compute an index into an array.

    let mut scores: HashMap<String, i32> = HashMap::new();
    scores.insert(String::from("Alice"), 10);
    scores.insert(String::from("Bob"), 20);

    // Now access Value associated with a key, you can use get method.
    // This method returns an Option<&V>, where V is the value type.
    let alice_score = scores.get(&String::from("Alice"));
    println!("Alice score is {:?}", &alice_score);

    // Now update Alice score to 30
    scores.insert(String::from("Alice"), 30);
    println!("and now Alice score is {:?}", scores);

    // Now we will try to Remove an element from the Map.
    // First add an element with say "Hello" and value as 0
    scores.insert(String::from("hello"), 0);
    println!("Scores map : {:?}", scores);
    scores.remove(&String::from("hello"));

    println!("after Removing, now Scores map : {:?}", scores);

}