use crate::{
    db,
    metadata::lrc::structs::SongLyrics,
    models::{File, Folder},
    schema::files::title,
};
use db::add::add_file_batch;
use diesel::{dsl::exists, select};
use tauri::{command, Manager};
use window_shadows::set_shadow;

#[command]
pub fn set_shadows(app_handler: tauri::AppHandle, label: String) {
    let window = app_handler.get_window(&label).unwrap();
    match set_shadow(&window, true) {
        Ok(_) => (),
        Err(e) => {
            println!("Trying to set shadows on an unsupported platform!");
            println!("{}", e.to_string());
            let _ = window.set_decorations(true);
        }
    }
}

use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");

#[command]
pub fn run_migrations() {
    let mut conn = db::database::establish_connection();
    conn.run_pending_migrations(MIGRATIONS)
        .expect("ERROR!: Error while migrating !");
}

#[command]
pub fn show_window(app: tauri::AppHandle) {
    let splash_screen_window = app.get_window("JUKBX-splashscreen").unwrap();
    let main_window = app.get_window("main").unwrap();
    main_window.show().unwrap();
    splash_screen_window.close().unwrap();
}

#[command]
pub async fn register_folder(
    app_handle: tauri::AppHandle,
    folder_location: String,
) -> Result<(), String> {
    use crate::library::folders::register_folder;
    use crate::schema::folders;
    use diesel::prelude::*;
    let mut conn = db::database::establish_connection();

    use crate::library::library::create_library;

    create_library(None, None, &mut conn);

    let folders = match select(exists(
        folders::dsl::folders.filter(folders::dsl::folder_location.eq(&folder_location)),
    ))
    .get_result::<bool>(&mut conn)
    {
        Ok(res) => res,
        Err(e) => match e {
            diesel::result::Error::DatabaseError(_, _) => todo!(),
            diesel::result::Error::NotFound => false,
            _ => return Err("Something happened while querying db, check logs!".to_string()),
        },
    };

    if folders == true {
        println!("Folder already registered!");
    } else {
        register_folder(folder_location.clone(), None, None, None, &mut conn);
        add_file_batch(app_handle, folder_location, &mut conn);
    }
    Ok(())
}

#[command]
pub async fn request_all_libraries() -> Vec<File> {
    use diesel::prelude::*;
    let mut conn = db::database::establish_connection();
    use crate::models::File;
    use crate::schema::files::dsl::files;

    let res = files
        .select(File::as_select())
        .load::<File>(&mut conn)
        .unwrap();
    res
}

#[command]
pub async fn search_string(search_factor: String) -> Result<Vec<File>, String> {
    use diesel::prelude::*;
    let mut conn = db::database::establish_connection();
    use crate::models::File;
    use crate::schema::files::dsl::files;

    if search_factor.trim() != "" {
        let res = files
            .limit(5)
            .select(File::as_select())
            .filter(title.like(format!("%{}%", search_factor)))
            .load::<File>(&mut conn);

        match res {
            Ok(a) => Ok(a),
            Err(e) => match e {
                diesel::result::Error::DatabaseError(_, _) => todo!(),
                _ => return Err("Something happened while querying db, check logs!".to_string()),
            },
        }
    } else {
        Ok(Vec::new())
    }
}

#[command]
pub async fn request_folders() -> Vec<Folder> {
    use diesel::prelude::*;
    let mut conn = db::database::establish_connection();
    use crate::models::Folder;
    use crate::schema::folders::dsl::folders;

    let res = folders
        .select(Folder::as_select())
        .load::<Folder>(&mut conn)
        .unwrap();
    res
}

#[command]
pub async fn request_lyrics(file_location: String) -> Option<SongLyrics> {
    use crate::metadata::lrc::get::request_lyrics;
    request_lyrics(file_location)
}

#[command]
pub fn request_song(song_id: i32) -> Option<File> {
    use crate::models::File;
    use crate::schema::files::dsl::files;
    use diesel::prelude::*;
    let mut conn = db::database::establish_connection();
    let song = files
        .find(song_id)
        .select(File::as_select())
        .first::<File>(&mut conn)
        .optional();

    match song {
        Ok(Some(song)) => Some(song),
        Ok(None) => None,
        Err(_) => None,
    }
}
