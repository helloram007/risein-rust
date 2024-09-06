fn main() {
    
    let book = Book {
        title: String::from("The way of Zen"),
        author: String::from("Alan watts"),
        publication_year: 1957,
    };

    println!(
        "The book {} is written by {} in {}",
        book.title, book.author, book.publication_year
    );

    let mut book = Book {
        title: String::from("The way of Zen"),
        author: String::from("Alan watts"),
        publication_year: 1957,
    };
    book.publication_year = 1989;
    println!(
        "The book {} is written by {} in {}",
        book.title, book.author, book.publication_year
    );

    let book_data = get_book_data(book);

    for data in book_data {
        println!("{data}");
    }

    //let my_book = create_book(title: "The Path of Zen".to_string(), author: "Simon".to_string(), publication_year: 2003);
    let my_book = create_book("The Path of Zen".to_string(),"Simon".to_string(), 2003);
    let book_data = get_book_data(my_book);
        println!("my book is {:?} ", {book_data});


    // Tuple Struct
    let tuple_book = Tuple_Book("Some book".to_string(), "Simon".to_string(), 2023);
    println!("The tuple book is {:?}", tuple_book);
    // Unit Struct
    let unit_book = Unit_Book;

    // if i have to get the title of the book from tuple struct
    let title = tuple_book.0;
    let author = tuple_book.1;
    let publication_year = tuple_book.2;

    println!("Tile: {}", title);
    println!("Author: {}", author);
    println!("Publication Year: {}", publication_year);
    
    let my_rectangle = Rectangle{
        width: 10.0,
        height: 5.0,
    };

    let area = my_rectangle.area();
    println!("Area of Rectangle is {}", area);

}

#[derive(Debug)]
struct Book {
    title: String,
    author: String,
    publication_year: u32,
}

// Tuple Struct
#[derive(Debug)]
struct Tuple_Book(String, String, u32);
struct Unit_Book;

fn get_book_data(book: Book) -> [String; 3] {
    let title = book.title;
    let author = book.author;
    let publication_year = book.publication_year;

    let data: [String; 3] = [title, author, publication_year.to_string()];
    
    data
}

fn create_book(
    title: String,
    author: String,
    publication_year: u32) -> Book {
        let book = Book{
            title, author,publication_year,
        };
        book
}

struct Rectangle {
    width: f64,
    height: f64,
}

impl Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}