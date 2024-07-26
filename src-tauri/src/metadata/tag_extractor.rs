use crate::db;
use crate::library::albums::{
    check_album_exists, create_and_return_default_album, create_return_album, select_album_name,
};
use crate::library::artists::{
    check_artist_exists, create_default_artist, create_return_artist, select_artist_name,
};
use crate::library::folders::{check_folder_exists, get_folder_by_path, register_folder};
use crate::library::library::get_library;
use crate::log::tag_error::recover_file;
use crate::models::NewFile;
use crate::{
    log::tag_error::handle_tag_extract_error, metadata::image_extractor::extract_img_metadata,
};
use lofty::{Accessor, AudioFile, Probe, TaggedFileExt};
use serde::Serialize;
use std::path::Path;
use tauri::Manager;

#[derive(Serialize, Clone)]
struct Payload {
    path: String,
}

pub fn tag_extractor(app_handle: tauri::AppHandle, location: String) -> Option<NewFile> {
    let path = Path::new(&location);

    if !path.is_file() {
        return None;
    }

    let _ = app_handle.emit_all(
        "processing_file",
        Payload {
            path: location.clone(),
        },
    );

    let tagged_file = match Probe::open(path) {
        Ok(file) => match file.read() {
            Ok(tags) => tags,
            Err(e) => return handle_tag_extract_error(e, location),
        },
        Err(_) => {
            println!("Could not open file: {}", location);
            return None;
        }
    };

    let tag = match tagged_file.primary_tag() {
        Some(primary_tag) => primary_tag,
        None => match tagged_file.first_tag() {
            Some(tag) => tag,
            None => return recover_file(location),
        },
    };

    let title = match tag.title() {
        Some(tag) => Some(tag.into_owned()),
        _ => None,
    };

    let artist = match tag.artist() {
        Some(tag) => Some(tag.into_owned()),
        _ => None,
    };

    let file_year = match tag.year() {
        Some(tag) => Some(tag as i32),
        _ => None,
    };

    let album = match tag.album() {
        Some(tag) => Some(tag.into_owned()),
        _ => None,
    };

    let image_path = if tag.picture_count() != 0 {
        let img_data = tag.pictures()[0].data();
        let img_mime = tag.pictures()[0].mime_type();
        extract_img_metadata(app_handle, img_data, img_mime)
    } else {
        None
    };

    let disk = match tag.get_string(&lofty::ItemKey::DiscNumber) {
        Some(tag) => Some(tag.parse::<i32>().unwrap()),
        _ => None,
    };

    let comment = match tag.get_string(&lofty::ItemKey::Comment) {
        Some(tag) => Some(tag.to_string()),
        _ => None,
    };

    let track = match tag.get_string(&lofty::ItemKey::TrackNumber) {
        Some(tag) => Some(tag.parse::<i32>().unwrap()),
        _ => None,
    };

    let lyricist = match tag.get_string(&lofty::ItemKey::Lyricist) {
        Some(tag) => Some(tag.to_string()),
        _ => None,
    };

    let producer = match tag.get_string(&lofty::ItemKey::Producer) {
        Some(tag) => Some(tag.to_string()),
        _ => None,
    };

    let label = match tag.get_string(&lofty::ItemKey::Label) {
        Some(tag) => Some(tag.to_string()),
        _ => None,
    };
    let publisher = match tag.get_string(&lofty::ItemKey::Publisher) {
        Some(tag) => Some(tag.to_string()),
        _ => None,
    };
    let mood = match tag.get_string(&lofty::ItemKey::Mood) {
        Some(tag) => Some(tag.to_string()),
        _ => None,
    };
    let genre = match tag.get_string(&lofty::ItemKey::Genre) {
        Some(tag) => Some(tag.to_string()),
        _ => None,
    };
    let language = match tag.get_string(&lofty::ItemKey::Language) {
        Some(tag) => Some(tag.to_string()),
        _ => None,
    };
    let lyrics = match tag.get_string(&lofty::ItemKey::Lyrics) {
        Some(tag) => Some(tag.to_string()),
        _ => None,
    };
    let license = match tag.get_string(&lofty::ItemKey::License) {
        Some(tag) => Some(tag.to_string()),
        _ => None,
    };
    let encoded_by = match tag.get_string(&lofty::ItemKey::EncodedBy) {
        Some(tag) => Some(tag.to_string()),
        _ => None,
    };
    let encoder_software = match tag.get_string(&lofty::ItemKey::EncoderSoftware) {
        Some(tag) => Some(tag.to_string()),
        _ => None,
    };
    let filetype = match tag.get_string(&lofty::ItemKey::FileType) {
        Some(tag) => Some(tag.to_string()),
        _ => None,
    };

    let properties = tagged_file.properties();

    let bitrate = match properties.audio_bitrate() {
        Some(a) => Some(a as i32),
        None => None,
    };

    let duration = Some(properties.duration().as_secs() as i32);

    let sample_rate = match properties.sample_rate() {
        Some(a) => Some(a as i32),
        None => None,
    };
    let bit_depth = match properties.bit_depth() {
        Some(a) => Some(a as i32),
        None => None,
    };
    let channels = match properties.channels() {
        Some(a) => Some(a as i32),
        None => None,
    };

    let library_id = get_library(&mut db::database::establish_connection())
        .expect("COULD NOT GET LIBRARY \n FATAL UNLOGGED ERROR!")
        .id;

    // !! skull emoji

    let folder_id = if check_folder_exists(
        path.parent().expect("").to_str().unwrap().to_string(),
        &mut db::database::establish_connection(),
    ) {
        get_folder_by_path(
            path.parent().expect("").to_str().unwrap().to_string(),
            &mut db::database::establish_connection(),
        )
        .expect("")
        .id
    } else {
        register_folder(
            path.parent().expect("").to_str().unwrap().to_string(),
            None,
            None,
            None,
            &mut db::database::establish_connection(),
        );
        get_folder_by_path(
            path.parent().expect("").to_str().unwrap().to_string(),
            &mut db::database::establish_connection(),
        )
        .expect("")
        .id
    };

    let artist_id = if artist == None {
        create_default_artist(&mut db::database::establish_connection()).id
    } else if check_artist_exists(
        artist.clone().expect(""),
        &mut db::database::establish_connection(),
    ) {
        select_artist_name(
            artist.clone().expect(""),
            &mut db::database::establish_connection(),
        )
        .expect("")
        .id
    } else {
        create_return_artist(
            artist.clone().expect(""),
            None,
            None,
            &mut db::database::establish_connection(),
        )
        .id
    };
    let album_id = if album == None {
        create_and_return_default_album(&mut db::database::establish_connection()).id
    } else if check_album_exists(
        album.clone().expect(""),
        &mut db::database::establish_connection(),
    ) {
        select_album_name(
            album.clone().expect(""),
            &mut db::database::establish_connection(),
        )
        .expect("")
        .id
    } else {
        create_return_album(
            album.clone().expect(""),
            None,
            None,
            None,
            artist_id,
            &mut db::database::establish_connection(),
        )
        .id
    };

    Some(NewFile {
        library_id,
        location,
        folder_id,
        title,
        artist,
        artist_id,
        file_year,
        album,
        album_id,
        comment,
        track,
        disk,
        lyricist,
        producer,
        label,
        publisher,
        mood,
        genre,
        language,
        lyrics,
        license,
        encoded_by,
        encoder_software,
        filetype,
        bitrate,
        duration,
        sample_rate,
        bit_depth,
        channels,
        image_path,
    })
}
