-- This file should undo anything in `up.sql`
PRAGMA foreign_keys = OFF;
DROP TABLE folders;
DROP TABLE playlists;
DROP TABLE playlists_ref_file;
DROP TABLE artist_ref_file;
DROP TABLE artists;
DROP TABLE albums;
DROP TABLE library;
DROP TABLE files;

PRAGMA foreign_keys = ON;