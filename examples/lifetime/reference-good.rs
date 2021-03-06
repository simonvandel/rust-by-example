struct Book {
    title: String,
    author: String,
    year: uint,
}

fn get_title<'a>(book: &'a Book) -> &'a str {
    book.title.as_slice()
}

fn main() {
    let geb = Book {
        // construct a `String` (heap allocated string) from a reference to a
        // string (&'static str) by doing a copy of the data
        author: String::from_str("Douglas Hofstadter"),
        title: String::from_str("Gödel, Escher, Bach"),
        year: 1979,
    };

    let title: &str = get_title(&geb);

    println!("I just read {}", title);
}
