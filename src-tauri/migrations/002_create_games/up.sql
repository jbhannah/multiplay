CREATE TABLE IF NOT EXISTS games (
    id INT PRIMARY KEY,
    name TEXT,
    path TEXT,
    platform_id INT,
    shasum TEXT,
    FOREIGN KEY (platform_id) REFERENCES platforms (id)
);
