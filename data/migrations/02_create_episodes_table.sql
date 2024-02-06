CREATE TABLE IF NOT EXISTS episodes (
       id INTEGER NOT NULL PRIMARY KEY,
       title TEXT NOT NULL,
       description TEXT,
       url TEXT,
       image_url TEXT,
       date_published INTEGER NOT NULL,
       show_id INTEGER NOT NULL,
       FOREIGN KEY (show_id) REFERENCES shows(id)
);
