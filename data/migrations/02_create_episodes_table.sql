CREATE TABLE IF NOT EXISTS episodes (
       id INTEGER NOT NULL PRIMARY KEY,
       title TEXT NOT NULL,
       description TEXT,
       url TEXT,
       guid TEXT,
       image_url TEXT,
       date_published INTEGER,
       show_id INTEGER,
       FOREIGN KEY (show_id) REFERENCES shows(id)
);
