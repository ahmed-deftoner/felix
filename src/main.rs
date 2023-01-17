mod models;
mod args;
use args::EntityType;
use args::RustflixArgs;
use clap::Parser;

fn main() {
    let args = RustflixArgs::parse();

/*     match args.entity_type {
        EntityType::User(user) => handle_user_command(user),
        EntityType::Video(video) => handle_video_command(video),
        EntityType::View(view) => handle_view_command(view),
    };*/
}
