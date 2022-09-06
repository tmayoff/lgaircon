-- Your SQL goes here
CREATE TABLE temperature (
    timestamp DATETIME NOT NULL DEFAULT(STRFTIME('%Y-%m-%d %H:%M:%f', 'NOW')) PRIMARY KEY,
    value DECIMAL NOT NULL
)