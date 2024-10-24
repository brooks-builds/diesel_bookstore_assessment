mod utilities;
use diesel::prelude::*;
use diesel::{QueryDsl, SelectableHelper};
use diesel_bookstore_assessment::queries::book_queries::{
    create_book, delete_book, get_all_books, get_book_by_id, update_book,
};
use diesel_bookstore_assessment::{connect::connect, models::Book};
use eyre::Result;
use utilities::random_name;

#[test]
fn create_book_test() -> Result<()> {
    use diesel_bookstore_assessment::schema::books::dsl::books;

    let connection = &mut connect()?;
    let new_book_name = random_name("new book");
    let created_book_id = create_book(&new_book_name, connection)?;
    let db_book: Option<Book> = books
        .find(created_book_id)
        .select(Book::as_select())
        .first(connection)
        .optional()?;

    assert!(db_book.is_some_and(|db_book| db_book.name == new_book_name));

    Ok(())
}

#[test]
fn get_all_books_test() -> Result<()> {
    let connection = &mut connect()?;
    let book_1_name = random_name("first book");
    let book_2_name = random_name("second book");
    let created_book_1 = create_book(&book_1_name, connection)?;
    let created_book_2 = create_book(&book_2_name, connection)?;
    let db_books = get_all_books(connection)?;

    assert!(db_books.len() >= 2);

    for db_book in db_books {
        if db_book.id == created_book_1 {
            assert_eq!(db_book.name, book_1_name);
        } else if db_book.id == created_book_2 {
            assert_eq!(db_book.name, book_2_name);
        }
    }

    Ok(())
}

#[test]
fn get_one_book_test() -> Result<()> {
    let connection = &mut connect()?;
    let book_name = random_name("book name");
    let created_book_id = create_book(&book_name, connection)?;
    let db_book = get_book_by_id(created_book_id, connection)?;

    assert!(db_book.is_some_and(|book| book.name == book_name));

    Ok(())
}

#[test]
fn update_book_test() -> Result<()> {
    let connection = &mut connect()?;
    let original_name = &random_name("new book");
    let new_name = "I am a new book";
    let book_id = create_book(original_name, connection)?;

    update_book(book_id, new_name, connection)?;

    let db_book = get_book_by_id(book_id, connection)?;

    assert!(db_book.is_some_and(|book| book.name == new_name));

    Ok(())
}

#[test]
fn delete_book_test() -> Result<()> {
    let connection = &mut connect()?;
    let book_name = &random_name("book name");
    let created_book_id = create_book(book_name, connection)?;

    delete_book(created_book_id, connection)?;

    let deleted_book = get_book_by_id(created_book_id, connection)?;

    assert!(deleted_book.is_none());

    Ok(())
}
