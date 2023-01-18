use clap::{
    Args, 
    Parser, 
    Subcommand
};
use uuid::Uuid;

#[derive(Parser, Debug)]
#[clap(author, version, about)]
pub struct FelixArgs {
    #[clap(subcommand)]
    pub entity_type: EntityType,
}

#[derive(Debug, Subcommand)]
pub enum EntityType {
    /// Create, update, delete or show songs
    Song(SongCommand),

    /// Create, update, delete or show playlists
    Playlist(PlaylistCommand),

    /// Create, update, delete or show users
    User(UserCommand),
}

#[derive(Debug, Args)]
pub struct UserCommand {
    #[clap(subcommand)]
    pub command: UserSubcommand,
}

#[derive(Debug, Subcommand)]
pub enum UserSubcommand {
    /// Create a new user
    Create(CreateUser),

    /// Update an existing user
    Update(UpdateUser),

    /// Delete a user
    Delete(DeleteEntity),

    /// Show all users
    Show,
}

#[derive(Debug, Args)]
pub struct CreateUser {
    /// The name of the user
    pub name: String,

    /// The email of the user
    pub email: String,
}

#[derive(Debug, Args)]
pub struct UpdateUser {
    /// The name of the user
    pub name: String,

    /// The email of the user
    pub email: String,

    /// The playlists of the user
    pub playlists: Vec<Uuid>
}

#[derive(Debug, Args)]
pub struct DeleteEntity {
    /// The id of the entity to delete
    pub id: Uuid,
}

#[derive(Debug, Args)]
pub struct SongCommand {
    #[clap(subcommand)]
    pub command: SongSubcommand,
}

#[derive(Debug, Subcommand)]
pub enum SongSubcommand {
    /// Create a new Song
    Create(CreateSong),

    /// Update an existing Song
    Update(UpdateSong),

    /// Delete a Song
    Delete(DeleteEntity),

    /// Show all Songs
    Show,
}

#[derive(Debug, Args)]
pub struct CreateSong {
    /// The name of the Song to create
    pub name: String,

    /// The artist of the Song to create
    pub artist: String,
}

#[derive(Debug, Args)]
pub struct UpdateSong {
    /// The id of the Song
    pub id: Uuid,

    /// The title of the Song
    pub name: String,

    /// The description of the Song
    pub artist: String,
}


#[derive(Debug, Args)]
pub struct PlaylistCommand {
    #[clap(subcommand)]
    pub command: PlaylistSubcommand,
}

#[derive(Debug, Subcommand)]
pub enum PlaylistSubcommand {
    /// Create a new Playlist
    Create(CreatePlaylist),

    /// Update an existing Playlist
    Update(UpdatePlaylist),

    /// Delete a Playlist
    Delete(DeleteEntity),

    /// Show all Playlists
    Show,
}

#[derive(Debug, Args)]
pub struct CreatePlaylist {
    /// The name of the Playlist
    pub name: String,

    /// The status of the Playlist
    pub is_public: bool,

    /// The songs in the playlist
    pub songs: Vec<Uuid>
}

#[derive(Debug, Args)]
pub struct UpdatePlaylist {
    /// The id of the Playlist
    pub id: Uuid,

    /// The name of the Playlist
    pub name: String,

    /// The status of the Playlist
    pub is_public: bool,

    /// The songs of the Playlist
    pub songs: Vec<Uuid>
}


#[derive(Debug, Args)]
pub struct ViewCommand {
    #[clap(subcommand)]
    pub command: ViewSubcommand,
}

#[derive(Debug, Subcommand)]
pub enum ViewSubcommand {
    /// Create a new view
    Create(CreateView),

    /// Show all views with id numbers for users, songs and playlists
    Show,

    /// Show all views with names for users, songs and playlists
    ShowPretty
}

#[derive(Debug, Args)]
pub struct CreateView {
    /// The id of the user who listened to the Song
    pub user_id: i32,

    /// The id of the Song the user listened
    pub Song_id: i32,

    /// How long the user listened the Song for
    pub duration: i32,
}