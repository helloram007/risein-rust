use std::fmt::format;

fn main() {

    // Basic example of trait
    let dog = Dog { name: String::from("Hey Doggy") };
    dog.speak();

    // now construct a trait for struct Circle
    let circle = Circle { radius: 10.0 };
    let result = circle.display();
    println!("Result for circle display: {}", result);

    // now construct a trait for struct Rectangle
    let rec = Rectangle { width: 1.0, height: 2.0 };
    let result = rec.display();
    println!("Result for rectanlge display: {}", result);

    // now construct for String
    let name = String::from("Ram");
    let result = name.display();
    println!("Display for string: {}", result);
}

trait Speak {
    fn speak(&self);
}

struct Dog {
    name: String,
}

impl Speak for Dog {
    fn speak(&self) {
        println!("{} says: Woof!", self.name);
    }
}

trait Display {
    fn display(&self) -> String;
}

struct Circle {
    radius: f64,
}

struct Rectangle {
    width: f64,
    height: f64,
}

impl Display for Circle {
    fn display(&self) -> String {
        format!("Circle with radius: {}", self.radius)
    }
}

impl Display for Rectangle {
    fn display(&self) -> String {
        format!("Rectangle with width: {} and height: {}", self.width, self.height)
    }
}

// Sometime we want to implement for existing type like String
impl Display for String {
    fn display(&self) -> String {
        self.clone()
    }
}
