use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Song {
    id: Uuid,
    name: String,
    artist: String
}

#[derive(Debug, Clone)]
pub struct Playlist {
    id: Uuid,
    name: String,
    songs: Vec<Song>,
    is_public: bool
}

#[derive(Debug)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub playlists: Vec<Playlist>
}