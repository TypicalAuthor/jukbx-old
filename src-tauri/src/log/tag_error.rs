use std::path::Path;

use lofty::LoftyError;

use crate::models::NewFile;

pub fn handle_tag_extract_error(e: LoftyError, file_path: String) -> Option<NewFile> {
    match e.kind() {
        //There are cases where recovery is not possible at the moment, so we skip the file
        lofty::error::ErrorKind::UnknownFormat => return None,
        lofty::error::ErrorKind::TooMuchData => return None,
        lofty::error::ErrorKind::SizeMismatch => return None,
        //UnknownFormat hasnt been matched, so file is still an audio file, but for wherever reason we cant decode it (Corrupted?)
        lofty::error::ErrorKind::FileDecoding(_) => {
            println!("FILE DECODING ERROR: {}", file_path);
            return recover_file(file_path);
        }
        //Unrecoverable
        lofty::error::ErrorKind::FileEncoding(_) => {
            println!("FILE ENCODING ERROR: {}", file_path);
            return None;
        }
        //If we cant access the picture, the frontend should fallback to its default picture (There might still be other tags though)
        lofty::error::ErrorKind::NotAPicture => {
            println!("NOT A PICTURE ERROR: {}", file_path);
            return recover_file(file_path);
        }
        lofty::error::ErrorKind::UnsupportedPicture => {
            println!("UNSUPPORTED PICTURE ERROR: {}", file_path);
            return None;
        }
        //Unrecoverable
        lofty::error::ErrorKind::UnsupportedTag => {
            println!("UNSUPPORTED TAG ERROR: {}", file_path);
            return None;
        }
        //Unrecoverable
        lofty::error::ErrorKind::FakeTag => {
            println!("FAKE TAG ERROR: {}", file_path);
            return None;
        }
        //Probably an error for string that arent UTF8
        lofty::error::ErrorKind::TextDecode(_) => {
            println!("TEXT DECODE ERROR: {}", file_path);
            return recover_file(file_path);
        }
        lofty::error::ErrorKind::Id3v2(_) => {
            println!("ID3V2 ERROR: {}", file_path);
            return None;
        }
        //Unrecoverable
        lofty::error::ErrorKind::BadAtom(_) => {
            println!("BAD ATOM ERROR: {}", file_path);
            return None;
        }
        //Unrecoverable
        lofty::error::ErrorKind::AtomMismatch => {
            println!("ATOM MISMATCH ERROR: {}", file_path);
            return None;
        }
        lofty::error::ErrorKind::OggPage(_) => {
            println!("OGG PAGE ERROR: {}", file_path);
            return recover_file(file_path);
        }
        lofty::error::ErrorKind::StringFromUtf8(_) => {
            println!("STRING FROM UTF8 ERROR: {}", file_path);
            return recover_file(file_path);
        }
        lofty::error::ErrorKind::StrFromUtf8(_) => {
            println!("STR FROM UTF8 ERROR: {}", file_path);
            return recover_file(file_path);
        }
        //Unrecoverable
        lofty::error::ErrorKind::Io(_) => {
            println!("IO ERROR: {}", file_path);
            return None;
        }
        //Unrecoverable
        lofty::error::ErrorKind::Alloc(_) => {
            println!("ALLOC ERROR: {}", file_path);
            return None;
        }
        _ => return None,
    }
}

pub fn recover_file(file_path: String) -> Option<NewFile> {
    let path = Path::new(&file_path);

    if !path.is_file() {
        return None;
    }

    let file_name = match path.file_name() {
        Some(str) => match str.to_str() {
            Some(string) => string.to_string(),
            None => "Unknown Title".to_string(),
        },
        None => "Unknown Title".to_string(),
    };
    //File still is an audio file, but it probably has no tags or they are corrupted, either way we should return it!
    return Some(NewFile {
        title: Some(file_name),
        artist: Some("Unknown Artist".to_string()),
        album: Some("Unknown album".to_string()),
        image_path: None,
        location: file_path,
        file_year: None,
        comment: None,
        track: None,
        disk: None,
        lyricist: None,
        producer: None,
        label: None,
        publisher: None,
        mood: None,
        genre: None,
        language: None,
        lyrics: None,
        license: None,
        encoded_by: None,
        encoder_software: None,
        filetype: None,
        bitrate: None,
        duration: None,
        sample_rate: None,
        bit_depth: None,
        channels: None,
        library_id: todo!(),
        folder_id: todo!(),
        album_id: todo!(),
        artist_id: todo!(),
    });
}
