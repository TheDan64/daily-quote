CREATE TABLE author(
    id BIGSERIAL PRIMARY KEY,
    name TEXT UNIQUE NOT NULL,
    note TEXT NOT NULL
);
CREATE TABLE quote(
    id BIGSERIAL PRIMARY KEY,
    author_id BIGINT REFERENCES author (id) NOT NULL,
    text TEXT NOT NULL,
    note TEXT NOT NULL,
    retrieved BOOLEAN NOT NULL
);
