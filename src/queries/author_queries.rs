use crate::models::{Author, NewAuthor};
use crate::schema;
use diesel::{associations::HasTable, prelude::*};
use eyre::{Context, Result};

pub fn create_author(name: &str, connection: &mut PgConnection) -> Result<i32> {
    use schema::authors::dsl::id;

    let new_author = NewAuthor {
        name: name.to_owned(),
    };

    new_author
        .insert_into(Author::table())
        .returning(id)
        .get_result(connection)
        .context("creating author")
}

pub fn get_all_authors(connection: &mut PgConnection) -> Result<Vec<Author>> {
    use schema::authors::dsl::authors;

    authors
        .select(Author::as_select())
        .load(connection)
        .context("getting all authors")
}

pub fn get_author_by_id(id: i32, connection: &mut PgConnection) -> Result<Option<Author>> {
    use schema::authors::dsl::authors;

    authors
        .find(id)
        .get_result(connection)
        .optional()
        .context("getting author by id")
}

pub fn update_author(id: i32, new_name: &str, connection: &mut PgConnection) -> Result<()> {
    use schema::authors::dsl::{authors, name};

    diesel::update(authors.find(id))
        .set(name.eq(new_name))
        .execute(connection)
        .context("updating author")?;

    Ok(())
}

pub fn delete_author(id: i32, connection: &mut PgConnection) -> Result<()> {
    use schema::authors::dsl::authors;

    diesel::delete(authors.find(id))
        .execute(connection)
        .context("deleting author")?;

    Ok(())
}
