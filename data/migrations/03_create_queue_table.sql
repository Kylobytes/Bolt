CREATE TABLE IF NOT EXISTS queue (
       id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
       episode_id INTEGER NOT NULL,
       priority INTEGER NOT NULL,
       FOREIGN KEY (episode_id) REFERENCES episodes(id)
)
