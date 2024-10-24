use diesel::{
    associations::Associations, deserialize::Queryable, prelude::Insertable, Identifiable,
    Selectable,
};

#[derive(Queryable, Selectable, Identifiable, Debug)]
#[diesel(table_name = crate::schema::books)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Book {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = crate::schema::books)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewBook {
    pub name: String,
}

#[derive(Queryable, Selectable, Identifiable, Debug)]
#[diesel(table_name = crate::schema::authors)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Author {
    pub id: i32,
    pub name: String,
}

#[derive(Queryable, Selectable, Associations, Debug, Identifiable)]
#[diesel(table_name = crate::schema::book_authors)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(primary_key(book_id, author_id))]
#[diesel(belongs_to(Book))]
#[diesel(belongs_to(Author))]
pub struct BookAuthor {
    pub book_id: i32,
    pub author_id: i32,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = crate::schema::authors)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewAuthor {
    pub name: String,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = crate::schema::book_authors)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewBookAuthor {
    pub book_id: i32,
    pub author_id: i32,
}
