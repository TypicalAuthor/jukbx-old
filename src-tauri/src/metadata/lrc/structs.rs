use serde::Serialize;
#[derive(Serialize, Debug, Clone)]
pub struct Lrc {
    pub miliseconds: i64,
    pub lyrics: String,
    pub active: bool,
}

use serde::ser::SerializeStruct;
#[derive(Debug, Clone)]
pub struct SongLyrics {
    pub lyrics: Option<Result<Vec<Lrc>, String>>,
}

impl Serialize for SongLyrics {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        let mut state = serializer.serialize_struct("lrc", 1)?;
        match &self.lyrics {
            Some(lyrics) => match lyrics {
                Ok(lrc) => state.serialize_field("lyrics", lrc)?,
                Err(ulyrics) => state.serialize_field("lyrics", ulyrics)?,
            },
            None => state.serialize_field("lyrics", &None::<String>)?,
        }
        state.end()
    }
}
