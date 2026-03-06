use std::fs::File;
use std::io::{Write, BufReader, BufRead};

struct Book {
    title: String,
    author: String,
    year: u16,
}

fn save_books(books: &Vec<Book>, filename: &str) {
    let mut file = File::create(filename).expect("Failed to create file");

    for book in books {
        writeln!(file, "{},{},{}", book.title, book.author, book.year)
            .expect("Failed to write to file");
    }
}

fn load_books(filename: &str) -> Vec<Book> {
    let file = File::open(filename).expect("Failed to open file");
    let reader = BufReader::new(file);

    let mut books = Vec::new();

    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        let parts: Vec<&str> = line.split(',').collect();

        if parts.len() == 3 {
            let book = Book {
                title: parts[0].to_string(),
                author: parts[1].to_string(),
                year: parts[2].parse::<u16>().expect("Invalid year"),
            };
            books.push(book);
        }
    }

    books
}

fn main() {
    let books = vec![
        Book { title: "1984".to_string(), author: "George Orwell".to_string(), year: 1949 },
        Book { title: "To Kill a Mockingbird".to_string(), author: "Harper Lee".to_string(), year: 1960 },
    ];

    save_books(&books, "books.txt");
    println!("Books saved to file.");

    let loaded_books = load_books("books.txt");
    println!("Loaded books:");
    for book in loaded_books {
        println!("{} by {}, published in {}", book.title, book.author, book.year);
    }

       println!("");

    let books2 = vec![
        Book { title: "Wonder".to_string(), author: "R. J. Palacio".to_string(), year: 2012 },
        Book { title: "The Lightning Thief".to_string(), author: "Rick Riordan".to_string(), year: 2005 },
    ];

    save_books(&books2, "books2.txt");
    println!("Books saved to file.");

    let loaded_books2 = load_books("books2.txt");
    println!("Loaded books:");
    for book in loaded_books2 {
        println!("{} by {}, published in {}", book.title, book.author, book.year);
    }

       println!("");

    let books3 = vec![
        Book { title: "The Sea of Monsters".to_string(), author: "Rick Riordan".to_string(), year: 2006 },
        Book { title: "The Titan's Curse".to_string(), author: "Rick Riordan".to_string(), year: 2007 },
        Book { title: "The Battle of the Labyrinth".to_string(), author: "Rick Riordan".to_string(), year: 2008 },
    ];

    save_books(&books3, "books3.txt");
    println!("Books saved to file.");

    let loaded_books3 = load_books("books.txt");
    println!("Loaded books:");
    for book in loaded_books3 {
        println!("{} by {}, published in {}", book.title, book.author, book.year);
    }
}