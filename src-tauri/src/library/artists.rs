use crate::models::{Artist, NewArtist};
use diesel::dsl::exists;
use diesel::{prelude::*, select};
use diesel::{RunQueryDsl, SqliteConnection};

pub fn create_default_artist(conn: &mut SqliteConnection) -> Artist {
    use crate::schema::artists;
    use crate::schema::artists::dsl::artists as dsl_artists;

    let exists = select(exists(
        artists::dsl::artists.filter(artists::dsl::is_unknown_artist.eq(&true)),
    ))
    .get_result::<bool>(conn)
    .unwrap();

    if !exists {
        diesel::insert_into(artists::table)
            .values(NewArtist {
                artist_name: "Unknown artist".to_string(),
                artist_description: None,
                artist_img_path: None,
                is_unknown_artist: Some(true),
            })
            .execute(conn)
            .unwrap();
    }

    let res = dsl_artists
        .select(Artist::as_select())
        .filter(artists::is_unknown_artist.eq(true))
        .first(conn)
        .unwrap();
    res
}

///Creates a new artist struct and inserts it into the artists table
/// # Panic
/// Panics if there was an error while inserting artist into database
pub fn create_artist(
    artist_name: String,
    artist_description: Option<String>,
    artist_img_path: Option<String>,
    conn: &mut SqliteConnection,
) {
    use crate::schema::artists;
    diesel::insert_into(artists::table)
        .values(NewArtist {
            artist_name: artist_name,
            artist_description: artist_description,
            artist_img_path: artist_img_path,
            is_unknown_artist: Some(false),
        })
        .execute(conn)
        .unwrap();
}

///Creates a new artist struct, inserts it into the artists table and returns the inserted entry
/// # Panic
/// Panics if there was an error while inserting artist into database
pub fn create_return_artist(
    artist_name: String,
    artist_description: Option<String>,
    artist_img_path: Option<String>,
    conn: &mut SqliteConnection,
) -> Artist {
    use crate::schema::artists;
    diesel::insert_into(artists::table)
        .values(NewArtist {
            artist_name: artist_name,
            artist_description: artist_description,
            artist_img_path: artist_img_path,
            is_unknown_artist: Some(false),
        })
        .returning(Artist::as_returning())
        .get_result(conn)
        .unwrap()
}

///Check if artist exists in the artists table
/// # Panic
/// Panics if there was an error while querying database
pub fn check_artist_exists(artist_name: String, conn: &mut SqliteConnection) -> bool {
    use crate::schema::artists;
    select(exists(
        artists::dsl::artists.filter(
            artists::dsl::is_unknown_artist
                .eq(false)
                .and(artists::dsl::artist_name.eq(artist_name)),
        ),
    ))
    .get_result::<bool>(conn)
    .unwrap()
}

///Selects an artist from the database based on the ID
/// # Panic
/// Panics if there was an error while querying database
pub fn select_artist(artist_id: i32, conn: &mut SqliteConnection) -> Option<Artist> {
    use crate::schema::artists;
    use crate::schema::artists::dsl::artists as dsl_artists;

    dsl_artists
        .select(Artist::as_select())
        .filter(artists::id.eq(artist_id))
        .first(conn)
        .optional()
        .unwrap()
}

///Selects an artist from the database based on the name
/// # Panic
/// Panics if there was an error while querying database
pub fn select_artist_name(artist_name: String, conn: &mut SqliteConnection) -> Option<Artist> {
    use crate::schema::artists;
    use crate::schema::artists::dsl::artists as dsl_artists;

    dsl_artists
        .select(Artist::as_select())
        .filter(artists::artist_name.eq(artist_name))
        .first(conn)
        .optional()
        .unwrap()
}

///Query artists based on name
/// # Panic
/// Panics if there was an error while querying database
pub fn query_name_artists(artist_name: String, conn: &mut SqliteConnection) -> Vec<Artist> {
    use crate::schema::artists;
    use crate::schema::artists::dsl::artists as dsl_artists;

    dsl_artists
        .select(Artist::as_select())
        .filter(artists::artist_name.eq(artist_name))
        .load::<Artist>(conn)
        .unwrap()
}

///Query artists based on description
/// # Panic
/// Panics if there was an error while querying database
pub fn query_description_artists(
    artist_description: String,
    conn: &mut SqliteConnection,
) -> Vec<Artist> {
    use crate::schema::artists;
    use crate::schema::artists::dsl::artists as dsl_artists;

    dsl_artists
        .select(Artist::as_select())
        .filter(artists::artist_description.eq(artist_description))
        .load::<Artist>(conn)
        .unwrap()
}

///Query all artists
/// # Panic
/// Panics if there was an error while querying database
pub fn query_all_artists(conn: &mut SqliteConnection) -> Vec<Artist> {
    use crate::schema::artists::dsl::artists as dsl_artists;

    dsl_artists
        .select(Artist::as_select())
        .load::<Artist>(conn)
        .unwrap()
}

///Updates artist's name
/// # Panic
/// Panics if there was an error while updating entry
pub fn update_artist_name() {
    todo!()
}

///Updates artist's year
/// # Panic
/// Panics if there was an error while updating entry
pub fn update_artist_year() {
    todo!()
}

///Updates artist's description
/// # Panic
/// Panics if there was an error while updating entry
pub fn update_artist_description() {
    todo!()
}

pub fn remove_artist(artist_id: i32) {
    todo!()
}
