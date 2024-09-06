#[warn(dead_code)]
// trait with default function implementation
trait Animal {
    
    fn make_sound(&self) -> &str;

    fn speak(&self){
        println!("The animal says: {}", self.make_sound());
    }
}

// inheritance
#[warn(dead_code)]
#[warn(unused_features)]
trait Mammal: Animal {
    fn walk(&self);
}

struct Dog;
struct Cat;

impl Animal for Dog {
    fn make_sound(&self) -> &str {
        "Woof!"
    }
}

impl Animal for Cat {
    fn make_sound(&self) -> &str {
        "Meow!"
    }
}

fn main(){

    let dog = Dog;
    let cat = Cat;
    dog.speak();
    cat.speak();
}