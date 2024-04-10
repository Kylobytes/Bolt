CREATE TABLE IF NOT EXISTS podcasts (
       id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
       name TEXT NOT NULL,
       description TEXT,
       url TEXT NOT NULL,
       image_url TEXT,
);
