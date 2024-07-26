use std::fs;
use std::path::Path;

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use tauri::api::path::home_dir;

pub fn establish_connection() -> SqliteConnection {
    if cfg!(debug_assertions) {
        match SqliteConnection::establish("jukbxdb.db") {
            Ok(res) => return res,
            Err(e) => match e {
                ConnectionError::InvalidCString(e) => {
                    println!("ERROR: INVALID C STRING:{}", e);
                    panic!();
                }
                ConnectionError::BadConnection(e) => {
                    println!("ERROR: BAD CONNECTION:{}", e);
                    panic!();
                }
                ConnectionError::InvalidConnectionUrl(e) => {
                    println!("ERROR: INVALID CONNECTION URL:{}", e);
                    panic!();
                }
                ConnectionError::CouldntSetupConfiguration(e) => {
                    println!("ERROR: COULD NOT SETUP CONFIGURATION:{}", e);
                    panic!();
                }
                _ => {
                    panic!();
                }
            },
        }
    } else {
        let home_dir = home_dir().expect("Could not get the home directory!");

        let db_dir = Path::new(&home_dir).join("jukbx");

        let db_path = Path::new(&home_dir).join("jukbx").join("jukbx.db");

        let db_str = db_path
            .to_str()
            .expect("Could not convert db path to string!");

        //This should only fail if the folder already exists, in that case we ignore the error and continue, if it fails because of something else then the app will crash
        let _ = match fs::create_dir(db_dir) {
            Ok(a) => a,
            Err(_) => (),
        };

        match SqliteConnection::establish(&db_str) {
            Ok(res) => return res,
            Err(e) => match e {
                ConnectionError::InvalidCString(e) => {
                    println!("ERROR: INVALID C STRING:{}", e);
                    panic!();
                }
                ConnectionError::BadConnection(e) => {
                    println!("ERROR: BAD CONNECTION:{}", e);
                    panic!();
                }
                ConnectionError::InvalidConnectionUrl(e) => {
                    println!("ERROR: INVALID CONNECTION URL:{}", e);
                    panic!();
                }
                ConnectionError::CouldntSetupConfiguration(e) => {
                    println!("ERROR: COULD NOT SETUP CONFIGURATION:{}", e);
                    panic!();
                }
                _ => {
                    panic!();
                }
            },
        }
    }
}
