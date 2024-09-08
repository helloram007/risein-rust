fn main() {
    //panic!("This is where we hit the wall!");
    chop("carrot");
    chop("celery");
    chop("apple");
    chop("tomato");

}

fn chop(vegetable: &str) {
    match vegetable {
        "carrot" => println!("Chopping a carrot."),
        "celery" => println!("Chopping a celery."),
        "tomato" => panic!("Don't know how to handle a tomato!"),
        _ => println!("Chopping some unknown vegetable."),
    }
}