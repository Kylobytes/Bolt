CREATE TABLE IF NOT EXISTS subscriptions (
       id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
       show_id INTEGER NOT NULL,
       created_at INTEGER NOT NULL DEFAULT (STRFTIME('%s', 'NOW')),
       updated_at INTEGER NOT NULL DEFAULT (STRFTIME('%s', 'NOW')),
       FOREIGN KEY(show_id) REFERENCES shows(id)
);
