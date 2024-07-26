// @generated automatically by Diesel CLI.

diesel::table! {
    albums (id) {
        id -> Integer,
        album_name -> Text,
        album_artist -> Nullable<Text>,
        album_year -> Nullable<Integer>,
        artist_id -> Integer,
        album_img_path -> Nullable<Text>,
        is_unknown_album -> Nullable<Bool>,
    }
}

diesel::table! {
    artist_ref_file (id) {
        id -> Integer,
        id_artist -> Integer,
        id_file -> Integer,
    }
}

diesel::table! {
    artists (id) {
        id -> Integer,
        artist_name -> Text,
        artist_description -> Nullable<Text>,
        artist_img_path -> Nullable<Text>,
        is_unknown_artist -> Nullable<Bool>,
    }
}

diesel::table! {
    files (id) {
        id -> Integer,
        library_id -> Integer,
        location -> Text,
        folder_id -> Integer,
        title -> Nullable<Text>,
        artist -> Nullable<Text>,
        artist_id -> Integer,
        file_year -> Nullable<Integer>,
        album -> Nullable<Text>,
        album_id -> Integer,
        comment -> Nullable<Text>,
        track -> Nullable<Integer>,
        disk -> Nullable<Integer>,
        lyricist -> Nullable<Text>,
        producer -> Nullable<Text>,
        label -> Nullable<Text>,
        publisher -> Nullable<Text>,
        mood -> Nullable<Text>,
        genre -> Nullable<Text>,
        language -> Nullable<Text>,
        lyrics -> Nullable<Text>,
        license -> Nullable<Text>,
        encoded_by -> Nullable<Text>,
        encoder_software -> Nullable<Text>,
        filetype -> Nullable<Text>,
        bitrate -> Nullable<Integer>,
        duration -> Nullable<Integer>,
        sample_rate -> Nullable<Integer>,
        bit_depth -> Nullable<Integer>,
        channels -> Nullable<Integer>,
        image_path -> Nullable<Text>,
    }
}

diesel::table! {
    folders (id) {
        id -> Integer,
        folder_location -> Text,
        file_amount -> Nullable<Integer>,
        last_scan -> Nullable<Timestamp>,
        first_scan -> Nullable<Timestamp>,
    }
}

diesel::table! {
    library (id) {
        id -> Integer,
        library_size -> Nullable<Integer>,
        library_name -> Text,
        library_creation_date -> Nullable<Timestamp>,
    }
}

diesel::table! {
    playlists (id) {
        id -> Integer,
        playlist_img_path -> Nullable<Text>,
        playlist_name -> Text,
    }
}

diesel::table! {
    playlists_ref_file (id) {
        id -> Integer,
        id_playlist -> Integer,
        id_file -> Integer,
    }
}

diesel::joinable!(albums -> artists (artist_id));
diesel::joinable!(artist_ref_file -> artists (id_artist));
diesel::joinable!(artist_ref_file -> files (id_file));
diesel::joinable!(files -> albums (album_id));
diesel::joinable!(files -> artists (artist_id));
diesel::joinable!(files -> folders (folder_id));
diesel::joinable!(files -> library (library_id));
diesel::joinable!(playlists_ref_file -> files (id_file));
diesel::joinable!(playlists_ref_file -> playlists (id_playlist));

diesel::allow_tables_to_appear_in_same_query!(
    albums,
    artist_ref_file,
    artists,
    files,
    folders,
    library,
    playlists,
    playlists_ref_file,
);
