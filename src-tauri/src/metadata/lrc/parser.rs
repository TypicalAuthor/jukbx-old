use regex::Regex;

use super::structs::Lrc;
pub fn try_lrc(lyrics_string: &str) -> Result<Vec<Lrc>, String> {
    let re = Regex::new(r"\[([0-9]*)\:([0-9]*)\.([0-9]*)\]([^\[]*)").unwrap();
    let mut lrc: Vec<Lrc> = vec![];
    for (_, [minutes, seconds, miliseconds, lyrics]) in
        re.captures_iter(lyrics_string).map(|r| r.extract())
    {
        lrc.push(Lrc {
            miliseconds: {
                let mil = match miliseconds.parse::<i32>() {
                    Ok(m) => m,
                    Err(_) => return Err(lyrics_string.to_owned()),
                };

                let sec = match seconds.parse::<i32>() {
                    Ok(m) => m * 1000,
                    Err(_) => return Err(lyrics_string.to_owned()),
                };

                let min = match minutes.parse::<i32>() {
                    Ok(m) => (m * 60) * 1000,
                    Err(_) => return Err(lyrics_string.to_owned()),
                };

                (mil + sec + min) as i64
            },
            lyrics: lyrics.trim().to_owned(),
            active: false,
        });
    }
    if lrc.len() != 0 && (lyrics_string.is_empty() == false) {
        Ok(lrc)
    } else {
        Err(lyrics_string.to_owned())
    }
}
