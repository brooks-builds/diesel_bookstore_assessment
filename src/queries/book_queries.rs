use diesel::{associations::HasTable, prelude::*};
use eyre::{Context, Result};

use crate::{
    models::{Book, NewBook},
    schema,
};

pub fn create_book(name: &str, connection: &mut PgConnection) -> Result<i32> {
    use schema::books::dsl::id;

    let new_book = NewBook {
        name: name.to_owned(),
    };

    let created_id = new_book
        .insert_into(Book::table())
        .returning(id)
        .get_result(connection)
        .context("Getting id back after inserting book")?;

    Ok(created_id)
}

pub fn get_all_books(connection: &mut PgConnection) -> Result<Vec<Book>> {
    use schema::books::dsl::books;

    books
        .select(Book::as_select())
        .load(connection)
        .context("getting all books")
}

pub fn get_book_by_id(id: i32, connection: &mut PgConnection) -> Result<Option<Book>> {
    use schema::books::dsl::books;

    books
        .find(id)
        .select(Book::as_select())
        .first(connection)
        .optional()
        .context("getting book by id")
}

pub fn update_book(id: i32, new_name: &str, connection: &mut PgConnection) -> Result<()> {
    use schema::books::dsl::{books, name};

    diesel::update(books.find(id))
        .set(name.eq(new_name))
        .execute(connection)
        .context("updating book")?;

    Ok(())
}

pub fn delete_book(id: i32, connection: &mut PgConnection) -> Result<()> {
    use schema::books::dsl::books;

    diesel::delete(books.find(id))
        .execute(connection)
        .context("deleting book")?;

    Ok(())
}
