CREATE TABLE IF NOT EXISTS episodes (
       id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
       guid TEXT,
       title TEXT NOT NULL,
       notes TEXT
);
