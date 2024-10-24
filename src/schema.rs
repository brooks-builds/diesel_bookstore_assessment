// @generated automatically by Diesel CLI.

diesel::table! {
    authors (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Varchar,
    }
}

diesel::table! {
    book_authors (author_id, book_id) {
        author_id -> Int4,
        book_id -> Int4,
    }
}

diesel::table! {
    books (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Varchar,
    }
}

diesel::joinable!(book_authors -> authors (author_id));
diesel::joinable!(book_authors -> books (book_id));

diesel::allow_tables_to_appear_in_same_query!(
    authors,
    book_authors,
    books,
);
