use crate::models::{Folder, NewFolder};
use diesel::dsl::exists;
use diesel::{prelude::*, select};
use diesel::{RunQueryDsl, SqliteConnection};

pub fn register_folder(
    folder_location: String,
    file_amount: Option<i32>,
    last_scan: Option<chrono::NaiveDateTime>,
    first_scan: Option<chrono::NaiveDateTime>,
    conn: &mut SqliteConnection,
) {
    use crate::schema::folders;

    diesel::insert_into(folders::table)
        .values(NewFolder {
            folder_location,
            file_amount,
            last_scan,
            first_scan,
        })
        .execute(conn)
        .unwrap();
}

pub fn check_folder_exists(folder_location: String, conn: &mut SqliteConnection) -> bool {
    use crate::schema::folders;
    select(exists(
        folders::dsl::folders.filter(folders::dsl::folder_location.eq(folder_location)),
    ))
    .get_result::<bool>(conn)
    .unwrap()
}

pub fn get_folder_by_id(folder_id: i32, conn: &mut SqliteConnection) -> Option<Folder> {
    use crate::schema::folders;
    use crate::schema::folders::dsl::folders as dsl_folders;
    let res = dsl_folders
        .select(Folder::as_select())
        .filter(folders::id.eq(folder_id))
        .first(conn)
        .optional()
        .unwrap();
    res
}

pub fn get_folder_by_path(folder_location: String, conn: &mut SqliteConnection) -> Option<Folder> {
    use crate::schema::folders;
    use crate::schema::folders::dsl::folders as dsl_folders;
    let res = dsl_folders
        .select(Folder::as_select())
        .filter(folders::folder_location.eq(folder_location))
        .first(conn)
        .optional()
        .unwrap();
    res
}

pub fn query_folder_size(folder_id: i32) {
    todo!()
}

pub fn remove_folder(folder_id: i32) {
    todo!()
}
