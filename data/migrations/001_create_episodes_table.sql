CREATE TABLE IF NOT EXISTS episodes (
       id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
       guid TEXT NOT NULL,
       title TEXT NOT NULL,
       description TEXT,
       content TEXT,
       url TEXT NOT NULL,
       image_url TEXT,
       enclosure_url TEXT NOT NULL,
       queued INTEGER NOT NULL DEFAULT 0,
       date_published INTEGER,
       podcast_id INTEGER
       FOREIGN KEY (podcast_id) REFERENCES podcasts(id)
);
