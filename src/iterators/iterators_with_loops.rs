use core::num;
use std::collections::HashMap;
fn main() {
    let mut my_map = HashMap::new();

    my_map.insert("Alice".to_string(), 10);
    my_map.insert("Bob".to_string(), 20);

    for (key,value) in my_map.iter() {
        println!("{} has {}", key,value);
    }

    println!("=================Transform======================");

    let numbers = vec![1,2,3,4,5];

    let doubled: Vec<i32> = numbers.iter().map(|x| x * 2).collect();

    
    println!("{:?}", doubled);

    println!("=================Filter======================");
    let even_numbers: Vec<i32> =  numbers.into_iter().filter(|x| x % 2 == 0).collect();
    println!("{:?}", even_numbers);

    println!("=================Sum with Fold======================");
    let numbers = vec![1,2,3,4,5];
    let sum: i32 =  numbers.iter().fold(0, | acc, x | acc + x);
    println!("{:?}", sum);

    println!("=================Chained======================");
    let numbers = vec![1,2,3,4,5];
    let chained: Vec<i32> = numbers.into_iter()
                            .filter(|x| x % 2 == 0)
                            .map(|x| x * 2)
                            .collect();
    println!("{:?}", chained);

    println!("=================Chained/ Convert Vector into Hashmap======================");
    let numbers = vec![1,2,3,4,5];
    let vec_map: HashMap<_,_> = numbers.into_iter()
                                    .map(|n| (n, n * 2))
                                    .collect();
    println!("{:?}", vec_map);

    println!("=================Enumerate======================");
    let words = vec!["apple", "banana", "cherry", "date", "fig"];
    let result: Vec<_> = words
                        .iter()
                        .enumerate()
                        .filter(|(i,_)|i % 2 == 0)
                        .map(|(i,w)| format!("{}: {}", i + 1, w.to_uppercase()))
                        .collect();
    println!("{:?}", result);

}
