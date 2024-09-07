fn main() {
    sports::football();

    let messi = sports::FootballPlayer { name: "Messi".to_string(), age: 19 };
    println!("{} is {} old", messi.name, messi.age);
}

mod sports {
    pub fn football() {
        println!("let's play football...actually with our foot");
    }

    pub struct FootballPlayer {
        pub name: String,
        pub age: i32,
    }
}