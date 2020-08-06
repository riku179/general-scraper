-- Your SQL goes here
CREATE TABLE sources(
    id INTEGER PRIMARY KEY NOT NULL,
    name VARCHAR(255) NOT NULL,
    url VARCHAR(255) NOT NULL,
    selectors TEXT NOT NULL,
    last_accessed TIMESTAMP NOT NULL,
    last_accessed_urls TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL
);

CREATE TABLE contents(
    id VARCHAR(255) PRIMARY KEY NOT NULL,
    url VARCHAR(1000) NOT NULL,
    source_id INTEGER NOT NULL,
    title VARCHAR(1000) NOT NULL,
    body TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    FOREIGN KEY(source_id) REFERENCES sources(id)
);
