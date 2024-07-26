use std::path::Path;

use lofty::{Probe, TaggedFileExt};

use super::parser::try_lrc;
use super::structs::SongLyrics;

pub fn request_lyrics(file_path: String) -> Option<SongLyrics> {
    //TODO: Delete this function and pass lyrics through the SongMetadata Struct
    //? This will have to do while I figure out how to implement Queryable and Insertable for nested structs in Diesel
    //? Fortunately lyrics shouldnt be that costly to query

    let path = Path::new(&file_path);

    if !path.is_file() {
        return None;
    }

    let tagged_file = match Probe::open(path) {
        Ok(file) => match file.read() {
            Ok(tags) => tags,
            Err(_) => return None,
        },
        Err(_) => return None,
    };

    let tag = match tagged_file.primary_tag() {
        Some(primary_tag) => primary_tag,
        None => match tagged_file.first_tag() {
            Some(tag) => tag,
            None => return None,
        },
    };

    let lyrics = match tag.get_string(&lofty::ItemKey::Lyrics) {
        Some(l) => Some(try_lrc(l)),
        //Here there might still be lyrics, just under a different tag key so we make sure to cover every possibility
        None => match tag.get_string(&lofty::ItemKey::Unknown("LYRICS".to_owned())) {
            Some(ly) => Some(try_lrc(ly)),
            None => match tag.get_string(&lofty::ItemKey::Unknown("UNSYNCEDLYRICS".to_owned())) {
                Some(ul) => Some(try_lrc(ul)),
                None => {
                    match tag.get_string(&lofty::ItemKey::Unknown("UNSYNCED LYRICS".to_owned())) {
                        Some(u_l) => Some(try_lrc(u_l)),
                        None => match tag.get_string(&lofty::ItemKey::Unknown("SYLT".to_owned())) {
                            Some(sylt) => Some(try_lrc(sylt)),
                            None => {
                                match tag.get_string(&lofty::ItemKey::Unknown("USLT".to_owned())) {
                                    Some(uslt) => Some(try_lrc(uslt)),
                                    None => None,
                                }
                            }
                        },
                    }
                }
            },
        },
    };
    Some(SongLyrics { lyrics })
}
