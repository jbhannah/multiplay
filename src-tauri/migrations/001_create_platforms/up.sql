CREATE TABLE IF NOT EXISTS platforms (
    id INT PRIMARY KEY,
    name TEXT,
    acronym TEXT,
    rom_extensions TEXT,
    save_extensions TEXT
);
INSERT INTO platforms
VALUES (1, "Game Boy", "GB", "gb", "sav"),
    (2, "Game Boy Color", "GBC", "gbc", "sav"),
    (3, "Game Boy Advance", "GBA", "gba", "sav"),
    (4, "Nintendo DS", "NDS", "nds", "sav,dsv");
