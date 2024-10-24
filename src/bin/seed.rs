use diesel::{Insertable, RunQueryDsl, SelectableHelper};
use diesel_bookstore_assessment::{
    connect::connect,
    models::{Author, Book, NewAuthor, NewBook, NewBookAuthor},
    schema,
};

const BOOK_SEEDS: [&str; 6] = [
    "Brave New World",
    "Moby Dick",
    "Omoo",
    "Rip Van Winkle",
    "The Raven and Other Poems",
    "Mastering the Art of Programming: A Comprehensive Guide for Beginners",
];

const AUTHOR_SEEDS: [&str; 6] = [
    "Aldous Huxley",
    "Herman Melville",
    "Washington Irving",
    "Edgar Allan Poe",
    "Alistair Thompson",
    "Emily Sinclair",
];

const BOOK_AUTHOR_SEEDS: [(&str, &str); 7] = [
    ("Brave New World", "Aldous Huxley"),
    ("Moby Dick", "Herman Melville"),
    ("Omoo", "Herman Melville"),
    ("Rip Van Winkle", "Washington Irving"),
    ("The Raven and Other Poems", "Edgar Allan Poe"),
    (
        "Mastering the Art of Programming: A Comprehensive Guide for Beginners",
        "Alistair Thompson",
    ),
    (
        "Mastering the Art of Programming: A Comprehensive Guide for Beginners",
        "Emily Sinclair",
    ),
];

fn main() {
    let database_connection = &mut connect().unwrap();

    let books_to_create = BOOK_SEEDS
        .iter()
        .map(|name| NewBook {
            name: name.to_string(),
        })
        .collect::<Vec<NewBook>>();

    let authors_to_create = AUTHOR_SEEDS
        .iter()
        .map(|author| NewAuthor {
            name: author.to_string(),
        })
        .collect::<Vec<NewAuthor>>();

    let books = books_to_create
        .iter()
        .map(|new_book| {
            new_book
                .insert_into(schema::books::table)
                .returning(Book::as_select())
                .get_result(database_connection)
                .expect("inserting seed book}")
        })
        .collect::<Vec<Book>>();

    let authors = authors_to_create
        .iter()
        .map(|new_author| {
            new_author
                .insert_into(schema::authors::table)
                .returning(Author::as_select())
                .get_result(database_connection)
                .expect("inserting seed author")
        })
        .collect::<Vec<Author>>();

    BOOK_AUTHOR_SEEDS
        .iter()
        .for_each(|(book_name, author_name)| {
            let Some(book) = books.iter().find(|book| book.name == *book_name) else {
                panic!("cannot find book when inserting seeds")
            };
            let Some(author) = authors.iter().find(|author| author.name == *author_name) else {
                panic!("cannot find author when inserting seeds")
            };

            NewBookAuthor {
                book_id: book.id,
                author_id: author.id,
            }
            .insert_into(schema::book_authors::table)
            .execute(database_connection)
            .expect("Inserting books author seed");
        });
}
