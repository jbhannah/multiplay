CREATE TABLE IF NOT EXISTS platforms (
    id INT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    acronym TEXT NOT NULL,
    rom_extensions TEXT NOT NULL,
    save_extensions TEXT NOT NULL
);
INSERT INTO platforms
VALUES (1, "Game Boy", "GB", "gb", "sav"),
    (2, "Game Boy Color", "GBC", "gbc", "sav"),
    (3, "Game Boy Advance", "GBA", "gba", "sav"),
    (4, "Nintendo DS", "NDS", "nds", "sav,dsv");
