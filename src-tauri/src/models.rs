use diesel::prelude::*;
use serde::ser::{Serialize, SerializeStruct, Serializer};

#[derive(Debug, Queryable, Selectable)]
#[diesel(table_name = crate::schema::folders)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Folder {
    pub id: i32,
    pub folder_location: String,
    pub file_amount: Option<i32>,
    pub last_scan: Option<chrono::NaiveDateTime>,
    pub first_scan: Option<chrono::NaiveDateTime>,
}

impl Serialize for Folder {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("folder", 5)?;
        state.serialize_field("id", &self.id)?;
        state.serialize_field("folder_path", &self.folder_location)?;
        match &self.file_amount {
            Some(file_amount) => state.serialize_field("file_amount", file_amount)?,
            None => state.serialize_field("file_amount", &None::<i32>)?,
        }
        match &self.last_scan {
            Some(last_scan) => state.serialize_field("last_scan", last_scan)?,
            None => state.serialize_field("last_scan", &None::<chrono::NaiveDateTime>)?,
        }
        match &self.first_scan {
            Some(first_scan) => state.serialize_field("first_scan", first_scan)?,
            None => state.serialize_field("first_scan", &None::<chrono::NaiveDateTime>)?,
        }
        state.end()
    }
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::folders)]
pub struct NewFolder {
    pub folder_location: String,
    pub file_amount: Option<i32>,
    pub last_scan: Option<chrono::NaiveDateTime>,
    pub first_scan: Option<chrono::NaiveDateTime>,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::files)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct File {
    pub id: i32,
    pub library_id: i32,
    pub location: String,
    pub folder_id: i32,
    pub title: Option<String>,
    pub artist: Option<String>,
    pub artist_id: i32,
    pub file_year: Option<i32>,
    pub album: Option<String>,
    pub album_id: i32,
    pub comment: Option<String>,
    pub track: Option<i32>,
    pub disk: Option<i32>,
    pub lyricist: Option<String>,
    pub producer: Option<String>,
    pub label: Option<String>,
    pub publisher: Option<String>,
    pub mood: Option<String>,
    pub genre: Option<String>,
    pub language: Option<String>,
    pub lyrics: Option<String>,
    pub license: Option<String>,
    pub encoded_by: Option<String>,
    pub encoder_software: Option<String>,
    pub filetype: Option<String>,
    pub bitrate: Option<i32>,
    pub duration: Option<i32>,
    pub sample_rate: Option<i32>,
    pub bit_depth: Option<i32>,
    pub channels: Option<i32>,
    pub image_path: Option<String>,
}

impl Serialize for File {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("file", 32)?;
        state.serialize_field("id", &self.id)?;
        state.serialize_field("library_id", &self.library_id)?;
        state.serialize_field("location", &self.location)?;
        state.serialize_field("folder_id", &self.folder_id)?;
        match &self.title {
            Some(title) => state.serialize_field("title", title)?,
            None => state.serialize_field("title", &None::<String>)?,
        }
        match &self.artist {
            Some(artist) => state.serialize_field("artist", artist)?,
            None => state.serialize_field("artist", &None::<String>)?,
        }
        state.serialize_field("artist_id", &self.artist_id)?;
        match &self.file_year {
            Some(file_year) => state.serialize_field("year", file_year)?,
            None => state.serialize_field("year", &None::<i32>)?,
        }
        match &self.album {
            Some(album) => state.serialize_field("album", album)?,
            None => state.serialize_field("album", &None::<String>)?,
        }
        state.serialize_field("album_id", &self.album_id)?;
        match &self.comment {
            Some(comment) => state.serialize_field("comment", comment)?,
            None => state.serialize_field("comment", &None::<String>)?,
        }
        match &self.track {
            Some(track) => state.serialize_field("track", track)?,
            None => state.serialize_field("track", &None::<i32>)?,
        }
        match &self.disk {
            Some(disk) => state.serialize_field("disk", disk)?,
            None => state.serialize_field("disk", &None::<i32>)?,
        }
        match &self.lyricist {
            Some(lyricist) => state.serialize_field("lyricist", lyricist)?,
            None => state.serialize_field("lyricist", &None::<String>)?,
        }
        match &self.producer {
            Some(producer) => state.serialize_field("producer", producer)?,
            None => state.serialize_field("producer", &None::<String>)?,
        }
        match &self.publisher {
            Some(publisher) => state.serialize_field("publisher", publisher)?,
            None => state.serialize_field("publisher", &None::<String>)?,
        }
        match &self.mood {
            Some(mood) => state.serialize_field("mood", mood)?,
            None => state.serialize_field("mood", &None::<String>)?,
        }
        match &self.genre {
            Some(genre) => state.serialize_field("genre", genre)?,
            None => state.serialize_field("genre", &None::<String>)?,
        }
        match &self.language {
            Some(language) => state.serialize_field("language", language)?,
            None => state.serialize_field("label", &None::<String>)?,
        }
        match &self.lyrics {
            Some(lyrics) => state.serialize_field("lyrics", lyrics)?,
            None => state.serialize_field("lyrics", &None::<String>)?,
        }
        match &self.license {
            Some(license) => state.serialize_field("license", license)?,
            None => state.serialize_field("license", &None::<String>)?,
        }
        match &self.encoded_by {
            Some(encoded_by) => state.serialize_field("encoded_by", encoded_by)?,
            None => state.serialize_field("encoded_by", &None::<String>)?,
        }
        match &self.encoder_software {
            Some(encoder_software) => {
                state.serialize_field("encoder_software", encoder_software)?
            }
            None => state.serialize_field("encoder_software", &None::<String>)?,
        }
        match &self.filetype {
            Some(filetype) => state.serialize_field("filetype", filetype)?,
            None => state.serialize_field("filetype", &None::<String>)?,
        }
        match &self.bitrate {
            Some(bitrate) => state.serialize_field("bitrate", bitrate)?,
            None => state.serialize_field("bitrate", &None::<i32>)?,
        }
        match &self.duration {
            Some(duration) => state.serialize_field("duration", duration)?,
            None => state.serialize_field("duration", &None::<i32>)?,
        }
        match &self.sample_rate {
            Some(sample_rate) => state.serialize_field("sample_rate", sample_rate)?,
            None => state.serialize_field("sample_rate", &None::<i32>)?,
        }
        match &self.bit_depth {
            Some(bit_depth) => state.serialize_field("bit_depth", bit_depth)?,
            None => state.serialize_field("bit_depth", &None::<i32>)?,
        }
        match &self.channels {
            Some(channels) => state.serialize_field("channels", channels)?,
            None => state.serialize_field("channels", &None::<i32>)?,
        }
        match &self.image_path {
            Some(image_path) => state.serialize_field("image_path", image_path)?,
            None => state.serialize_field("image_path", &None::<String>)?,
        }
        state.end()
    }
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::files)]
pub struct NewFile {
    pub library_id: i32,
    pub location: String,
    pub folder_id: i32,
    pub title: Option<String>,
    pub artist: Option<String>,
    pub artist_id: i32,
    pub file_year: Option<i32>,
    pub album: Option<String>,
    pub album_id: i32,
    pub comment: Option<String>,
    pub track: Option<i32>,
    pub disk: Option<i32>,
    pub lyricist: Option<String>,
    pub producer: Option<String>,
    pub label: Option<String>,
    pub publisher: Option<String>,
    pub mood: Option<String>,
    pub genre: Option<String>,
    pub language: Option<String>,
    pub lyrics: Option<String>,
    pub license: Option<String>,
    pub encoded_by: Option<String>,
    pub encoder_software: Option<String>,
    pub filetype: Option<String>,
    pub bitrate: Option<i32>,
    pub duration: Option<i32>,
    pub sample_rate: Option<i32>,
    pub bit_depth: Option<i32>,
    pub channels: Option<i32>,
    pub image_path: Option<String>,
}

#[derive(Debug, Queryable, Selectable)]
#[diesel(table_name = crate::schema::albums)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Album {
    pub id: i32,
    pub album_name: String,
    pub album_artist: Option<String>,
    pub album_year: Option<i32>,
    pub album_img_path: Option<String>,
    pub artist_id: i32,
    pub is_unknown_album: Option<bool>,
}

impl Serialize for Album {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("album", 4)?;
        state.serialize_field("id", &self.id)?;
        state.serialize_field("album_name", &self.album_name)?;
        match &self.album_artist {
            Some(album_artist) => state.serialize_field("album_artist", album_artist)?,
            None => state.serialize_field("album_artist", &None::<String>)?,
        }
        match &self.album_year {
            Some(album_year) => state.serialize_field("album_year", album_year)?,
            None => state.serialize_field("album_year", &None::<i32>)?,
        }
        match &self.album_img_path {
            Some(album_img_path) => state.serialize_field("album_img_path", album_img_path)?,
            None => state.serialize_field("album_img_path", &None::<String>)?,
        }
        state.serialize_field("artist_id", &self.artist_id)?;
        match &self.is_unknown_album {
            Some(is_unknown_album) => {
                state.serialize_field("is_unknown_album", is_unknown_album)?
            }
            None => state.serialize_field("is_unknown_album", &false)?,
        }
        state.end()
    }
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::albums)]
pub struct NewAlbum {
    pub album_name: String,
    pub album_artist: Option<String>,
    pub album_year: Option<i32>,
    pub artist_id: i32,
    pub album_img_path: Option<String>,
    pub is_unknown_album: Option<bool>,
}

//TODO: ARTISTS

#[derive(Debug, Queryable, Selectable)]
#[diesel(table_name = crate::schema::artists)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Artist {
    pub id: i32,
    pub artist_name: String,
    pub artist_description: Option<String>,
    pub artist_img_path: Option<String>,
    pub is_unknown_artist: Option<bool>,
}

impl Serialize for Artist {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("artist", 4)?;
        state.serialize_field("id", &self.id)?;
        state.serialize_field("artist_name", &self.artist_name)?;
        match &self.artist_description {
            Some(artist_description) => {
                state.serialize_field("artist_description", artist_description)?
            }
            None => state.serialize_field("artist_description", &None::<String>)?,
        }
        match &self.artist_img_path {
            Some(artist_img_path) => state.serialize_field("artist_img_path", artist_img_path)?,
            None => state.serialize_field("artist_img_path", &None::<String>)?,
        }
        match &self.is_unknown_artist {
            Some(is_unknown_artist) => {
                state.serialize_field("is_unknown_artist", is_unknown_artist)?
            }
            None => state.serialize_field("is_unknown_artist", &false)?,
        }
        state.end()
    }
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::artists)]
pub struct NewArtist {
    pub artist_name: String,
    pub artist_description: Option<String>,
    pub artist_img_path: Option<String>,
    pub is_unknown_artist: Option<bool>,
}

//TODO: LIBRARY

#[derive(Debug, Queryable, Selectable)]
#[diesel(table_name = crate::schema::library)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Library {
    pub id: i32,
    pub library_name: String,
    pub library_size: Option<i32>,
    pub library_creation_date: Option<chrono::NaiveDateTime>,
}

impl Serialize for Library {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("library", 4)?;
        state.serialize_field("id", &self.id)?;
        state.serialize_field("library_name", &self.library_name)?;
        match &self.library_size {
            Some(library_size) => state.serialize_field("library_size", library_size)?,
            None => state.serialize_field("library_size", &None::<i32>)?,
        }
        match &self.library_creation_date {
            Some(library_creation_date) => {
                state.serialize_field("library_creation_date", library_creation_date)?
            }
            None => {
                state.serialize_field("library_creation_date", &None::<chrono::NaiveDateTime>)?
            }
        }
        state.end()
    }
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::library)]
pub struct NewLibrary {
    pub library_name: String,
    pub library_size: Option<i32>,
    pub library_creation_date: Option<chrono::NaiveDateTime>,
}

//TODO: PLAYLISTS

#[derive(Debug, Queryable, Selectable)]
#[diesel(table_name = crate::schema::playlists)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Playlist {
    pub id: i32,
    pub playlist_name: String,
    pub playlist_img_path: Option<String>,
}

impl Serialize for Playlist {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("playlist", 2)?;
        state.serialize_field("id", &self.id)?;
        state.serialize_field("playlist_name", &self.playlist_name)?;
        match &self.playlist_img_path {
            Some(playlist_img_path) => {
                state.serialize_field("playlist_img_path", playlist_img_path)?
            }
            None => state.serialize_field("playlist_img_path", &None::<String>)?,
        }
        state.end()
    }
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::playlists)]
pub struct NewPlaylist {
    pub playlist_name: String,
    pub playlist_img_path: Option<String>,
}

//? --------------- Ref tables

#[derive(Debug, Queryable, Selectable)]
#[diesel(table_name = crate::schema::playlists_ref_file)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct PlaylistsRefFile {
    pub id: i32,
    pub id_playlist: i32,
    pub id_file: i32,
}

impl Serialize for PlaylistsRefFile {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("playlistsreffile", 3)?;
        state.serialize_field("id", &self.id)?;
        state.serialize_field("id_playlist", &self.id_playlist)?;
        state.serialize_field("id_file", &self.id_file)?;
        state.end()
    }
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::playlists_ref_file)]
pub struct NewPlaylistsRefFile {
    pub id_playlist: i32,
    pub id_file: i32,
}

#[derive(Debug, Queryable, Selectable)]
#[diesel(table_name = crate::schema::artist_ref_file)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct ArtistRefFile {
    pub id: i32,
    pub id_artist: i32,
    pub id_file: i32,
}

impl Serialize for ArtistRefFile {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("artistreffile", 3)?;
        state.serialize_field("id", &self.id)?;
        state.serialize_field("id_artist", &self.id_artist)?;
        state.serialize_field("id_file", &self.id_file)?;
        state.end()
    }
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::artist_ref_file)]
pub struct NewArtistRefFile {
    pub id_artist: i32,
    pub id_file: i32,
}
