trait Printable {
    fn print(&self);
}

fn print_something<T: Printable>(item: &T) {
    item.print();
}

struct Book {
    title: String,
}

impl Printable for Book {
    fn print(&self) {
        println!("Book : {}", self.title);
    }
}

struct Magazine {
    name: String,
}

impl Printable for Magazine {
    fn print(&self) {
        println!("Magazine: {}", self.name);
    }
}

fn main() {
    let book = Book {
        title: String::from("The Rust Book"),
    };
    let magazine = Magazine { 
        name: String::from("Rust Monthly"),
    };

    print_something(&book);
    print_something(&magazine);
}