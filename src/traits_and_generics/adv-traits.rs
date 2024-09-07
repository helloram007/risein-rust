fn main(){
    basic();
    let my_trait_object: Box<dyn MakeNoise> = Box::new(Bird { 
        name: "Tweety".to_string(),
        color: "yellow".to_string(),
    });
    my_trait_object.talk();
    invite_to_animal_talks(my_trait_object);

    // lets create a vector
    let mut speakers: Vec<Box<dyn MakeNoise>> = Vec::new();
    speakers.push(Box::new(Bird { name: "bird1".to_string(), color: "yellow".to_string() }));
    speakers.push(Box::new(Dog { name: "dog1".to_string(), breed: "terrier".to_string() }));

    println!("===========================================================");
    for speaker in speakers {
        speaker.talk();
    }
}


trait MakeNoise{
    fn talk(&self);
}

struct Bird {
    name: String,
    color: String,
}

struct Dog {
    name: String,
    breed: String,
}
impl MakeNoise for Dog {
    fn talk(&self) {
        println!("Thats me, the doggy is talking.");
    }
}

impl MakeNoise for Bird {
    fn talk(&self) {
        println!("Thats me, the birdy is talking.");
    }
}

fn invite_to_animal_talks(speaker: Box<dyn MakeNoise>) {
    println!("Ladies and gentleman, please welcome to our next speaker:");
    speaker.talk();
}

// How to use trait objecsts safely.
// 1. Cannot have any associated constants in traits
// 2. In methods we use in traits, you should have ref to self or mutable ref to self as ap parameters
// 3. We cannot use Genrics in our methods
// Below is a bad implementation and not Safe.
trait Calculate<T> {
    const PI: f64;

    fn calculate_area(&self, value: T) -> f64;
}

fn basic() {
    
    // Create a mutable Boxed integer
    let mut pointer = Box::new(5);
    // Try to modify with *, you will get an error
    // When you deference the pointer and update, it will allow, because
    // you are not working on the reference, but you are working on the Value of the pointer.
    *pointer = 10;
    println!("{}", pointer);
}