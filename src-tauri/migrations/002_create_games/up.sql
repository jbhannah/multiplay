CREATE TABLE IF NOT EXISTS games (
    id INT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    path TEXT NOT NULL,
    platform_id INT NOT NULL,
    shasum TEXT NOT NULL,
    FOREIGN KEY (platform_id) REFERENCES platforms (id)
);
