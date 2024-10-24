mod utilities;

use diesel::prelude::*;
use diesel::{QueryDsl, SelectableHelper};
use diesel_bookstore_assessment::connect::connect;
use diesel_bookstore_assessment::models::Author;
use diesel_bookstore_assessment::queries::author_queries::*;
use eyre::Result;
use utilities::random_name;

#[test]
fn create_author_test() -> Result<()> {
    use diesel_bookstore_assessment::schema::authors::dsl::authors;

    let connection = &mut connect()?;
    let new_author_name = &random_name("New Author Test");
    let created_author_id = create_author(new_author_name, connection)?;
    let db_author: Option<Author> = authors
        .find(created_author_id)
        .select(Author::as_select())
        .first(connection)
        .optional()?;

    assert!(db_author.is_some_and(|db_author| db_author.name == *new_author_name));

    Ok(())
}

#[test]
fn get_all_authors_test() -> Result<()> {
    let connection = &mut connect()?;
    let author_1_name = random_name("first author");
    let author_2_name = random_name("second author");
    let created_author_1 = create_author(&author_1_name, connection)?;
    let created_author_2 = create_author(&author_2_name, connection)?;
    let db_authors = get_all_authors(connection)?;

    assert!(db_authors.len() >= 2);

    for db_author in db_authors {
        if db_author.id == created_author_1 {
            assert_eq!(db_author.name, author_1_name);
        } else if db_author.id == created_author_2 {
            assert_eq!(db_author.name, author_2_name);
        }
    }

    Ok(())
}

#[test]
fn get_one_author_test() -> Result<()> {
    let connection = &mut connect()?;
    let author_name = random_name("author name");
    let created_author_id = create_author(&author_name, connection)?;
    let db_author = get_author_by_id(created_author_id, connection)?;

    assert!(db_author.is_some_and(|author| author.name == author_name));

    Ok(())
}

#[test]
fn update_author_test() -> Result<()> {
    let connection = &mut connect()?;
    let original_name = random_name("new author");
    let new_name = random_name("I am a new author");
    let author_id = create_author(&original_name, connection)?;

    update_author(author_id, &new_name, connection)?;

    let db_author = get_author_by_id(author_id, connection)?;

    assert!(db_author.is_some_and(|author| author.name == new_name));

    Ok(())
}

#[test]
fn delete_author_test() -> Result<()> {
    let connection = &mut connect()?;
    let author_name = random_name("author name");
    let created_author_id = create_author(&author_name, connection)?;

    delete_author(created_author_id, connection)?;

    let deleted_author = get_author_by_id(created_author_id, connection)?;

    assert!(deleted_author.is_none());

    Ok(())
}
