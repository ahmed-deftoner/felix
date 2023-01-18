use uuid::Uuid;

#[derive(Debug)]
pub struct Song {
    id: Uuid,
    name: String,
    artist: String
}

#[derive(Debug, Clone, Copy)]
pub struct Playlist {
    id: Uuid,
    name: String,
    songs: Vec<Song>,
    is_public: bool
}

#[derive(Debug)]
pub struct User {
    id: Uuid,
    name: String,
    email: String,
    playlists: Vec<Playlist>
}