CREATE TABLE users (
    id SERIAL,
    name TEXT NOT NULL DEFAULT 'mate',
    external_id TEXT NOT NULL,
    PRIMARY KEY (id)
);

CREATE TABLE notes (
    id SERIAL,
    user_id INT NOT NULL,
    url TEXT NOT NULL,
    timestamp TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    published BOOLEAN NOT NULL DEFAULT FALSE,
    PRIMARY KEY (id),
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);
