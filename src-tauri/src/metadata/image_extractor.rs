use std::{
    collections::hash_map::DefaultHasher,
    fs,
    hash::{Hash, Hasher},
    path::PathBuf,
};

use image::{load_from_memory_with_format, ImageFormat};
use lofty::MimeType;

pub fn extract_img_metadata(
    app_handle: tauri::AppHandle,
    img_data: &[u8],
    img_mime: Option<&MimeType>,
) -> Option<String> {
    let img_mime_str = match img_mime {
        Some(mime) => mime.as_str(),
        None => return None,
    };

    let mut hasher = DefaultHasher::new();
    img_data.hash(&mut hasher);
    let img_name = hasher.finish();

    let format = match ImageFormat::from_mime_type(img_mime_str) {
        Some(fmt) => fmt,
        None => return None,
    };

    //If there is no format to associate this data, then we should return nothing
    if format.extensions_str().len() == 0 {
        return None;
    };

    let img = match load_from_memory_with_format(img_data, format) {
        Ok(img) => img,
        Err(_) => todo!(),
    };

    let mut data_dir = match app_handle.path_resolver().app_cache_dir() {
        Some(path) => path,
        None => ["./"].iter().collect::<PathBuf>(),
    };

    data_dir.push("imgcache");

    if !data_dir.is_dir() {
        match fs::create_dir_all(&data_dir) {
            Ok(_) => (),
            Err(_) => return None,
        }
    };

    data_dir.push(img_name.to_string());
    data_dir.set_extension(format.extensions_str()[0]);

    let data_dir_str = match data_dir.to_str() {
        Some(string) => string.to_owned(),
        _ => return None,
    };

    if data_dir.is_file() {
        return Some(data_dir_str);
    }

    match img.save_with_format(&data_dir, format) {
        Ok(_) => (),
        Err(_) => todo!(),
    }

    Some(data_dir_str)
}
