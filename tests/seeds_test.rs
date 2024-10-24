use assert_cmd::Command;
use diesel::{associations::HasTable, QueryDsl, RunQueryDsl, SelectableHelper};
use diesel_bookstore_assessment::{
    connect::connect,
    models::{Author, Book, BookAuthor},
};
use eyre::Result;

#[test]
fn seed_test() -> Result<()> {
    use diesel_bookstore_assessment::schema::authors::dsl::*;
    use diesel_bookstore_assessment::schema::book_authors::dsl::*;
    use diesel_bookstore_assessment::schema::books::dsl::*;

    let connection = &mut connect()?;

    diesel::delete(book_authors::table()).execute(connection)?;
    diesel::delete(authors::table()).execute(connection)?;
    diesel::delete(books::table()).execute(connection)?;

    let mut seed_command = Command::cargo_bin("seed")?;
    seed_command.assert().success();

    let book_names = [
        "Brave New World",
        "Moby Dick",
        "Omoo",
        "Rip Van Winkle",
        "The Raven and Other Poems",
        "Mastering the Art of Programming: A Comprehensive Guide for Beginners",
    ];
    let author_names = [
        "Aldous Huxley",
        "Herman Melville",
        "Washington Irving",
        "Edgar Allan Poe",
        "Alistair Thompson",
        "Emily Sinclair",
    ];
    let book_author_names = [
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

    let all_books = books.select(Book::as_select()).load(connection)?;
    let all_authors = authors.select(Author::as_select()).load(connection)?;
    let all_book_authors: Vec<(BookAuthor, Book, Author)> = book_authors
        .inner_join(books::table())
        .inner_join(authors::table())
        .select((
            BookAuthor::as_select(),
            Book::as_select(),
            Author::as_select(),
        ))
        .load(connection)?;

    assert_eq!(all_books.len(), book_names.len());
    assert_eq!(all_authors.len(), author_names.len());
    assert_eq!(all_book_authors.len(), book_author_names.len());

    assert!(
        all_books
            .iter()
            .all(|db_book| { book_names.contains(&db_book.name.as_str()) }),
        "Not all book names found in database, check your seeds"
    );

    assert!(
        all_authors
            .iter()
            .all(|db_author| author_names.contains(&db_author.name.as_str())),
        "Not all author names found in the database, check your seeds"
    );

    assert!(
        all_book_authors.iter().all(|(_, db_book, db_author)| {
            book_author_names.contains(&(&db_book.name.as_str(), &db_author.name.as_str()))
        }),
        "Not all the books are associated correctly with the authors. Check your seeds"
    );

    Ok(())
}
