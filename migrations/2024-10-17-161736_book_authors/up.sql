-- Your SQL goes here
CREATE TABLE book_authors (
    author_id INT NOT NULL REFERENCES authors (id),
    book_id INT NOT NULL REFERENCES books (id),
    PRIMARY KEY (author_id, book_id)
);
