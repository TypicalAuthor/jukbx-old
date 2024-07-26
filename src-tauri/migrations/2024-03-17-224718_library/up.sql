CREATE TABLE folders (
	id	INTEGER NOT NULL UNIQUE,
	folder_location	TEXT NOT NULL UNIQUE,
	file_amount	INTEGER DEFAULT 0,
  	last_scan TIMESTAMP,
  	first_scan TIMESTAMP,
	PRIMARY KEY(id AUTOINCREMENT)
);

CREATE TABLE playlists (
	id				INTEGER NOT NULL UNIQUE,
	playlist_img_path 	TEXT,
	playlist_name	TEXT NOT NULL UNIQUE,
	PRIMARY KEY(id AUTOINCREMENT)
);

CREATE TABLE playlists_ref_file (
	id	INTEGER NOT NULL UNIQUE,
	id_playlist	INTEGER NOT NULL,
	id_file		INTEGER NOT NULL,
  FOREIGN KEY (id_playlist) REFERENCES playlists(id)
  FOREIGN KEY (id_file) REFERENCES files(id)
  PRIMARY KEY(id AUTOINCREMENT)
);

CREATE TABLE artist_ref_file (
	id	INTEGER NOT NULL UNIQUE,
	id_artist	INTEGER NOT NULL,
	id_file		INTEGER NOT NULL,
  FOREIGN KEY (id_artist) REFERENCES artists(id)
  FOREIGN KEY (id_file) REFERENCES files(id)
  PRIMARY KEY(id AUTOINCREMENT)
);

CREATE TABLE artists (
	id	INTEGER NOT NULL UNIQUE,
	artist_name			TEXT NOT NULL,
	artist_description	TEXT,
  	artist_img_path 	TEXT,
	is_unknown_artist BOOLEAN,
	PRIMARY KEY(id AUTOINCREMENT)
);

CREATE TABLE albums (
	id	INTEGER NOT NULL UNIQUE,
	album_name		TEXT NOT NULL,
	album_artist	TEXT,
  	album_year 		INTEGER,
	artist_id		INTEGER NOT NULL,
	album_img_path 	TEXT,
	is_unknown_album BOOLEAN,
	PRIMARY KEY(id AUTOINCREMENT)
	FOREIGN KEY (artist_id) REFERENCES artists(id)
);

CREATE TABLE library (
	id	INTEGER NOT NULL UNIQUE,
	library_size	INTEGER,
	library_name	TEXT NOT NULL,
  	library_creation_date TIMESTAMP,
	PRIMARY KEY(id AUTOINCREMENT)
);

CREATE TABLE files (
	id	INTEGER NOT NULL UNIQUE,
  	library_id INTEGER NOT NULL,
  	location TEXT NOT NULL UNIQUE,
  	folder_id INTEGER NOT NULL,
	title		TEXT,
	artist		TEXT,
	artist_id 	INTEGER NOT NULL,
	file_year	INTEGER,
	album		TEXT,
  	album_id 	INTEGER NOT NULL,
  	comment 	TEXT,
	track 		INTEGER,
  	disk 		INTEGER,
  	lyricist 	TEXT,
  	producer 	TEXT,
  	label 		TEXT,
  	publisher 	TEXT,
  	mood 		TEXT,
  	genre 		TEXT,
  	language 	TEXT,
  	lyrics 		TEXT,
  	license 	TEXT,
  	encoded_by 	TEXT,
  	encoder_software TEXT,
  	filetype 	TEXT,
  	bitrate 	INTEGER,
  	duration 	INTEGER,
  	sample_rate INTEGER,
  	bit_depth 	INTEGER,
  	channels 	INTEGER,
	image_path 	TEXT,
	PRIMARY KEY(id AUTOINCREMENT)
  	FOREIGN KEY (folder_id) REFERENCES folders(id)
  FOREIGN KEY (library_id) REFERENCES library(id)
  FOREIGN KEY (album_id) REFERENCES albums(id)
  FOREIGN KEY (artist_id) REFERENCES artists(id)
);