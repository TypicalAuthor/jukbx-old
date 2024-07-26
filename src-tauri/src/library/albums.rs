use crate::library::artists::create_default_artist;
use crate::models::{Album, NewAlbum};
use diesel::dsl::exists;
use diesel::{prelude::*, select};
use diesel::{RunQueryDsl, SqliteConnection};

pub fn create_and_return_default_album(conn: &mut SqliteConnection) -> Album {
    use crate::schema::albums;
    use crate::schema::albums::dsl::albums as dsl_albums;

    let exists = select(exists(
        albums::dsl::albums.filter(albums::dsl::is_unknown_album.eq(&true)),
    ))
    .get_result::<bool>(conn)
    .unwrap();

    if !exists {
        diesel::insert_into(albums::table)
            .values(NewAlbum {
                album_name: "Unknown album".to_string(),
                album_artist: Some("Unknown artist".to_string()),
                album_year: None,
                is_unknown_album: Some(true),
                artist_id: create_default_artist(conn).id,
                album_img_path: None,
            })
            .execute(conn)
            .unwrap();
    };

    dsl_albums
        .select(Album::as_select())
        .filter(albums::is_unknown_album.eq(true))
        .first(conn)
        .unwrap()
}

///Creates a new album struct and inserts it into the albums table
/// # Panic
/// Panics if there was an error while inserting album into database
pub fn create_album(
    album_name: String,
    album_artist: Option<String>,
    album_year: Option<i32>,
    album_img_path: Option<String>,
    artist_id: i32,
    conn: &mut SqliteConnection,
) {
    use crate::schema::albums;
    diesel::insert_into(albums::table)
        .values(NewAlbum {
            album_name: album_name,
            is_unknown_album: Some(false),
            album_artist: album_artist,
            album_year: album_year,
            artist_id: artist_id,
            album_img_path,
        })
        .execute(conn)
        .unwrap();
}

///Creates a new album struct, inserts it into the albums table and returns the inserted entry
/// # Panic
/// Panics if there was an error while inserting album into database
pub fn create_return_album(
    album_name: String,
    album_artist: Option<String>,
    album_year: Option<i32>,
    album_img_path: Option<String>,
    artist_id: i32,
    conn: &mut SqliteConnection,
) -> Album {
    use crate::schema::albums;
    diesel::insert_into(albums::table)
        .values(NewAlbum {
            album_name: album_name,
            is_unknown_album: Some(false),
            album_artist: album_artist,
            album_year: album_year,
            artist_id: artist_id,
            album_img_path,
        })
        .returning(Album::as_returning())
        .get_result(conn)
        .unwrap()
}

///Check if album exists in the albums table
/// # Panic
/// Panics if there was an error while querying database
pub fn check_album_exists(album_name: String, conn: &mut SqliteConnection) -> bool {
    use crate::schema::albums;
    select(exists(
        albums::dsl::albums.filter(
            albums::dsl::is_unknown_album
                .eq(false)
                .and(albums::dsl::album_name.eq(album_name)),
        ),
    ))
    .get_result::<bool>(conn)
    .unwrap()
}

///Selects an album from the database based on the ID
/// # Panic
/// Panics if there was an error while querying database
pub fn select_album(album_id: i32, conn: &mut SqliteConnection) -> Option<Album> {
    use crate::schema::albums;
    use crate::schema::albums::dsl::albums as dsl_albums;

    dsl_albums
        .select(Album::as_select())
        .filter(albums::id.eq(album_id))
        .first(conn)
        .optional()
        .unwrap()
}

///Selects an album from the database based on the name
/// # Panic
/// Panics if there was an error while querying database
pub fn select_album_name(album_name: String, conn: &mut SqliteConnection) -> Option<Album> {
    use crate::schema::albums;
    use crate::schema::albums::dsl::albums as dsl_albums;

    dsl_albums
        .select(Album::as_select())
        .filter(albums::album_name.eq(album_name))
        .first(conn)
        .optional()
        .unwrap()
}

///Query albums based on name
/// # Panic
/// Panics if there was an error while querying database
pub fn query_name_albums(album_name: String, conn: &mut SqliteConnection) -> Vec<Album> {
    use crate::schema::albums;
    use crate::schema::albums::dsl::albums as dsl_albums;

    dsl_albums
        .select(Album::as_select())
        .filter(albums::album_name.eq(album_name))
        .load::<Album>(conn)
        .unwrap()
}

///Query albums based on description
/// # Panic
/// Panics if there was an error while querying database
pub fn query_artist_albums(album_artist: String, conn: &mut SqliteConnection) -> Vec<Album> {
    use crate::schema::albums;
    use crate::schema::albums::dsl::albums as dsl_albums;

    dsl_albums
        .select(Album::as_select())
        .filter(albums::album_artist.eq(album_artist))
        .load::<Album>(conn)
        .unwrap()
}

///Query albums based on description
/// # Panic
/// Panics if there was an error while querying database
pub fn query_year_albums(album_year: i32, conn: &mut SqliteConnection) -> Vec<Album> {
    use crate::schema::albums;
    use crate::schema::albums::dsl::albums as dsl_albums;

    dsl_albums
        .select(Album::as_select())
        .filter(albums::album_year.eq(album_year))
        .load::<Album>(conn)
        .unwrap()
}

///Query all albums
/// # Panic
/// Panics if there was an error while querying database
pub fn query_all_albums(conn: &mut SqliteConnection) -> Vec<Album> {
    use crate::schema::albums::dsl::albums as dsl_albums;

    dsl_albums
        .select(Album::as_select())
        .load::<Album>(conn)
        .unwrap()
}

///Updates album's name
/// # Panic
/// Panics if there was an error while updating entry
pub fn update_album_name() {
    todo!()
}

///Updates album's year
/// # Panic
/// Panics if there was an error while updating entry
pub fn update_album_year() {
    todo!()
}

///Updates album's description
/// # Panic
/// Panics if there was an error while updating entry
pub fn update_album_artist() {
    todo!()
}

pub fn remove_album(album_id: i32) {
    todo!()
}
