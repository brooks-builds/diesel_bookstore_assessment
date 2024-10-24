use super::{
    author_queries::{get_all_authors, get_author_by_id},
    book_queries::get_book_by_id,
};
use crate::{
    models::{Author, Book, BookAuthor, NewBookAuthor},
    queries::book_queries::get_all_books,
};
use diesel::{associations::HasTable, prelude::*, BelongingToDsl};
use eyre::{Context, Result};

pub fn associate_book_with_author(
    book_id: i32,
    author_id: i32,
    connection: &mut PgConnection,
) -> Result<()> {
    use crate::schema::book_authors::table as BookAuthorTable;

    let new_book_author = NewBookAuthor { author_id, book_id };

    new_book_author
        .insert_into(BookAuthorTable)
        .execute(connection)
        .context("associating book with author")?;

    Ok(())
}

pub fn get_author_with_books(
    author_id: i32,
    connection: &mut PgConnection,
) -> Result<Option<(Author, Vec<Book>)>> {
    let Some(author) = get_author_by_id(author_id, connection).context("getting author")? else {
        return Ok(None);
    };

    let books: Vec<Book> = BookAuthor::belonging_to(&author)
        .inner_join(Book::table())
        .select(Book::as_select())
        .get_results(connection)
        .context("getting all books for the author")?;

    Ok(Some((author, books)))
}

pub fn get_book_with_authors(
    book_id: i32,
    connection: &mut PgConnection,
) -> Result<Option<(Book, Vec<Author>)>> {
    let Some(book) = get_book_by_id(book_id, connection).context("getting book")? else {
        return Ok(None);
    };
    let authors: Vec<Author> = BookAuthor::belonging_to(&book)
        .inner_join(Author::table())
        .select(Author::as_select())
        .get_results(connection)
        .context("getting authors belong to the book")?;

    Ok(Some((book, authors)))
}

pub fn get_all_books_and_authors(
    connection: &mut PgConnection,
) -> Result<Vec<(Book, Vec<Author>)>> {
    let all_books = get_all_books(connection)?;
    let authors_for_books: Vec<(BookAuthor, Author)> = BookAuthor::belonging_to(&all_books)
        .inner_join(Author::table())
        .select((BookAuthor::as_select(), Author::as_select()))
        .get_results(connection)
        .context("getting authors for books")?;
    let books_with_authors = authors_for_books
        .grouped_by(&all_books)
        .into_iter()
        .zip(all_books)
        .map(|(authors, book)| {
            (
                book,
                authors.into_iter().map(|(_, author)| author).collect(),
            )
        })
        .collect::<Vec<(Book, Vec<Author>)>>();

    Ok(books_with_authors)
}

pub fn get_all_authors_and_books(
    connection: &mut PgConnection,
) -> Result<Vec<(Author, Vec<Book>)>> {
    let all_authors = get_all_authors(connection)?;
    let books_with_authors: Vec<(BookAuthor, Book)> = BookAuthor::belonging_to(&all_authors)
        .inner_join(Book::table())
        .select((BookAuthor::as_select(), Book::as_select()))
        .get_results(connection)
        .context("getting all books with authors")?;
    let authors_with_books = books_with_authors
        .grouped_by(&all_authors)
        .into_iter()
        .zip(all_authors)
        .map(|(books, author)| {
            (
                author,
                books
                    .into_iter()
                    .map(|(_, book)| book)
                    .collect::<Vec<Book>>(),
            )
        })
        .collect::<Vec<(Author, Vec<Book>)>>();

    Ok(authors_with_books)
}
