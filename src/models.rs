use uuid::Uuid;

pub struct Song {
    id: Uuid,
    name: String,
    artist: String
}

pub struct Playlist {
    id: Uuid,
    name: String,
    songs: Vec<Song>,
    is_public: bool
}

pub struct User {
    id: Uuid,
    name: String,
    email: String,
    playlists: Vec<Playlist>
}