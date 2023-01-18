mod models;
mod args;
mod ops;
use args::EntityType;
use args::FelixArgs;
use clap::Parser;

fn main() {
    let args = FelixArgs::parse();

/*     match args.entity_type {
        EntityType::User(user) => handle_user_command(user),
        EntityType::Video(video) => handle_video_command(video),
        EntityType::View(view) => handle_view_command(view),
    };*/
}
