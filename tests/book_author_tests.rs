mod utilities;

use diesel::prelude::*;
use diesel::QueryDsl;
use diesel_bookstore_assessment::models::BookAuthor;
use diesel_bookstore_assessment::queries::book_author_queries::get_all_authors_and_books;
use diesel_bookstore_assessment::queries::book_author_queries::get_all_books_and_authors;
use diesel_bookstore_assessment::queries::book_author_queries::{
    get_author_with_books, get_book_with_authors,
};
use diesel_bookstore_assessment::{
    connect::connect,
    queries::{
        author_queries::create_author, book_author_queries::associate_book_with_author,
        book_queries::create_book,
    },
    schema,
};
use eyre::Result;
use utilities::random_name;

#[test]
fn associate_book_with_author_test() -> Result<()> {
    use schema::book_authors::dsl::book_authors;

    let connection = &mut connect()?;
    let author_name = random_name("New Author");
    let book_name = random_name("New Book");
    let author_id = create_author(&author_name, connection)?;
    let book_id = create_book(&book_name, connection)?;

    associate_book_with_author(book_id, author_id, connection)?;

    let db_book_author: Option<BookAuthor> = book_authors
        .find((author_id, book_id))
        .select(BookAuthor::as_select())
        .first(connection)
        .optional()?;

    assert!(db_book_author.is_some());

    Ok(())
}

#[test]
fn get_author_with_its_one_book() -> Result<()> {
    let connection = &mut connect()?;
    let author_name = random_name("New Author");
    let book_name = random_name("New Book");
    let author_id = create_author(&author_name, connection)?;
    let book_id = create_book(&book_name, connection)?;

    associate_book_with_author(book_id, author_id, connection)?;

    let author_with_books = get_author_with_books(author_id, connection)?;

    assert!(author_with_books.is_some_and(|(author, books)| {
        author_name == author.name && books.len() == 1 && books[0].name == book_name
    }));

    Ok(())
}

#[test]
fn get_author_with_its_multiple_books_test() -> Result<()> {
    let connection = &mut connect()?;
    let author_name = random_name("New Author");
    let book_name = random_name("New Book");
    let second_book_name = random_name("Another new book");
    let author_id = create_author(&author_name, connection)?;
    let book_id = create_book(&book_name, connection)?;
    let second_book_id = create_book(&second_book_name, connection)?;

    associate_book_with_author(book_id, author_id, connection)?;
    associate_book_with_author(second_book_id, author_id, connection)?;

    let author_with_books = get_author_with_books(author_id, connection)?;

    assert!(author_with_books.is_some());

    let (_author, books) = author_with_books.unwrap();

    assert_eq!(books.len(), 2);

    assert!(books
        .iter()
        .all(|book| book.name == book_name || book.name == second_book_name));

    Ok(())
}

#[test]
fn get_book_with_its_one_author() -> Result<()> {
    let connection = &mut connect()?;
    let author_name = random_name("New Author");
    let book_name = random_name("New Book");
    let author_id = create_author(&author_name, connection)?;
    let book_id = create_book(&book_name, connection)?;

    associate_book_with_author(book_id, author_id, connection)?;

    let book_with_authors = get_book_with_authors(book_id, connection)?;

    assert!(book_with_authors.is_some());

    let (book, authors) = book_with_authors.unwrap();

    assert_eq!(book.name, book_name);
    assert_eq!(authors.len(), 1);

    assert!(authors.iter().all(|author| author.name == author_name));

    Ok(())
}

#[test]
fn get_book_with_its_multiple_authors() -> Result<()> {
    let connection = &mut connect()?;
    let author_name_1 = random_name("New Author");
    let author_name_2 = random_name("Second new author");
    let book_name = random_name("New Book");
    let author_id_1 = create_author(&author_name_1, connection)?;
    let author_id_2 = create_author(&author_name_2, connection)?;
    let book_id = create_book(&book_name, connection)?;

    associate_book_with_author(book_id, author_id_1, connection)?;
    associate_book_with_author(book_id, author_id_2, connection)?;

    let book_with_authors = get_book_with_authors(book_id, connection)?;

    assert!(book_with_authors.is_some());

    let (book, authors) = book_with_authors.unwrap();

    assert_eq!(book.name, book_name);
    assert_eq!(authors.len(), 2);

    assert!(authors
        .iter()
        .all(|author| author.name == author_name_1 || author.name == author_name_2));

    Ok(())
}

#[test]
fn get_all_books_with_their_authors_test() -> Result<()> {
    let connection = &mut connect()?;
    let book_1_name = random_name("book without an author");
    let book_2_name = random_name("book with 1 author");
    let book_3_name = random_name("book with 2 authors");
    let book_4_name = random_name("another book with 1 author");
    let author_1_name = random_name("author without a book");
    let author_2_name = random_name("wrote book 2 and 4");
    let author_3_name = random_name("wrote book 3");
    let author_4_name = random_name("co-wrote book 3");
    let book_1_id = create_book(&book_1_name, connection)?;
    let book_2_id = create_book(&book_2_name, connection)?;
    let book_3_id = create_book(&book_3_name, connection)?;
    let book_4_id = create_book(&book_4_name, connection)?;
    let _author_1_id = create_author(&author_1_name, connection)?;
    let author_2_id = create_author(&author_2_name, connection)?;
    let author_3_id = create_author(&author_3_name, connection)?;
    let author_4_id = create_author(&author_4_name, connection)?;

    associate_book_with_author(book_2_id, author_2_id, connection)?;
    associate_book_with_author(book_3_id, author_3_id, connection)?;
    associate_book_with_author(book_3_id, author_4_id, connection)?;
    associate_book_with_author(book_4_id, author_2_id, connection)?;

    let all_books_with_authors = get_all_books_and_authors(connection)?;

    let mut found_books = 0;

    for (book, authors) in all_books_with_authors {
        if book.id == book_1_id {
            found_books += 1;

            assert_eq!(authors.len(), 0);
            assert_eq!(book.name, book_1_name);
        } else if book.id == book_2_id {
            found_books += 1;

            assert_eq!(book.name, book_2_name);
            assert_eq!(authors.len(), 1);
            assert_eq!(authors[0].name, author_2_name);
        } else if book.id == book_3_id {
            found_books += 1;

            assert_eq!(book.name, book_3_name);
            assert_eq!(authors.len(), 2);
            assert!(authors
                .iter()
                .all(|author| author.name == author_3_name || author.name == author_4_name));
        } else if book.id == book_4_id {
            found_books += 1;

            assert_eq!(book.name, book_4_name);
            assert_eq!(authors.len(), 1);
            assert_eq!(authors[0].name, author_2_name);
        }
    }

    assert_eq!(found_books, 4);

    Ok(())
}
#[test]
fn get_all_authors_with_their_books_test() -> Result<()> {
    let connection = &mut connect()?;
    let book_1_name = random_name("book without an author");
    let book_2_name = random_name("book with 1 author");
    let book_3_name = random_name("book with 2 authors");
    let book_4_name = random_name("another book with 1 author");
    let author_1_name = random_name("author without a book");
    let author_2_name = random_name("wrote book 2 and 4");
    let author_3_name = random_name("wrote book 3");
    let author_4_name = random_name("co-wrote book 3");
    let _book_1_id = create_book(&book_1_name, connection)?;
    let book_2_id = create_book(&book_2_name, connection)?;
    let book_3_id = create_book(&book_3_name, connection)?;
    let book_4_id = create_book(&book_4_name, connection)?;
    let author_1_id = create_author(&author_1_name, connection)?;
    let author_2_id = create_author(&author_2_name, connection)?;
    let author_3_id = create_author(&author_3_name, connection)?;
    let author_4_id = create_author(&author_4_name, connection)?;

    associate_book_with_author(book_2_id, author_2_id, connection)?;
    associate_book_with_author(book_3_id, author_3_id, connection)?;
    associate_book_with_author(book_3_id, author_4_id, connection)?;
    associate_book_with_author(book_4_id, author_2_id, connection)?;

    let all_authors_with_books = get_all_authors_and_books(connection)?;

    let mut found_authors = 0;

    for (author, books) in all_authors_with_books {
        if author.id == author_1_id {
            found_authors += 1;

            assert_eq!(books.len(), 0);
            assert_eq!(author.name, author_1_name);
        } else if author.id == author_2_id {
            found_authors += 1;

            assert_eq!(author.name, author_2_name);
            assert_eq!(books.len(), 2);
            assert!(books
                .iter()
                .all(|book| book.name == book_2_name || book.name == book_4_name));
        } else if author.id == author_3_id {
            found_authors += 1;

            assert_eq!(author.name, author_3_name);
            assert_eq!(books.len(), 1);
            assert_eq!(books[0].name, book_3_name);
        } else if author.id == author_4_id {
            found_authors += 1;

            assert_eq!(author.name, author_4_name);
            assert_eq!(books.len(), 1);
            assert_eq!(books[0].name, book_3_name);
        }
    }

    assert_eq!(found_authors, 4);

    Ok(())
}
