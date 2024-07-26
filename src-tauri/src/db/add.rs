use crate::metadata::tag_extractor::tag_extractor;
use crate::models::NewFile;
use diesel::{RunQueryDsl, SqliteConnection};
use std::fs;
use std::path::Path;
use tauri::Manager;

pub fn add_file_batch(app_handle: tauri::AppHandle, path: String, conn: &mut SqliteConnection) {
    use crate::schema::files;

    let _ = app_handle.emit_all("begin_proccessing", ());

    let path = Path::new(&path);
    let mut res_vec: Vec<NewFile> = vec![];
    if path.is_dir() {
        for entry in fs::read_dir(&path).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path().to_str().unwrap().to_string();
            match tag_extractor(app_handle.clone(), path) {
                Some(a) => res_vec.push(a),
                None => continue,
            }
        }
    }

    diesel::insert_into(files::table)
        .values(&res_vec)
        .execute(conn)
        .unwrap();

    let _ = app_handle.emit_all("proccessing_finished", ());
}
