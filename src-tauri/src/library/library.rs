use diesel::dsl::exists;
use diesel::{prelude::*, select};

use crate::models::{Library, NewLibrary};

pub fn create_library(
    library_size: Option<i32>,
    library_creation_date: Option<chrono::NaiveDateTime>,
    conn: &mut SqliteConnection,
) {
    use crate::schema::library;

    diesel::insert_into(library::table)
        .values(NewLibrary {
            library_name: "Library".to_string(),
            library_size: library_size,
            library_creation_date: library_creation_date,
        })
        .execute(conn)
        .unwrap();
}

pub fn get_library(conn: &mut SqliteConnection) -> Option<Library> {
    use crate::schema::library::dsl::library as dsl_library;
    let res = dsl_library
        .select(Library::as_select())
        .first(conn)
        .optional()
        .unwrap();
    res
}

pub fn update_library_size(library_id: i32, size: i32) {
    todo!()
}

pub fn update_library_name(library_id: i32, name: String) {
    todo!()
}
