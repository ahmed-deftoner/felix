use uuid::{Uuid, uuid};

use crate::args::{
    UserSubcommand, 
    UserCommand, 
    CreateUser, 
    UpdateUser, 
    DeleteEntity
};
use crate::models::{User};

pub fn handle_user_command(user: UserCommand) {
    let command = user.command;
    match command {
        UserSubcommand::Create(user) => {
            create_user(user);
        }
        UserSubcommand::Update(user) => {
            update_user(user);
        }
        UserSubcommand::Delete(delete_entity) => {
            delete_user(delete_entity);
        }
        UserSubcommand::Show => {
            show_users();
        }
    }
}

fn create_user(user: CreateUser) {
    println!("Creating user: {:?}", user);

    let new_user = User {
        id: Uuid::new_v4(),
        name: user.name,
        email: user.email,
        playlists: [].to_vec(),
    };
}

fn update_user(user: UpdateUser) {
    println!("Updating user: {:?}", user);

    let db_user = DBUser {
        id: user.id,
        name: user.name,
        email: user.email,
        removed: false,
    };
    
    diesel::update(users.find(user.id))
        .set(&db_user)
        .execute(&connection)
        .expect("Error updating user");
}

fn delete_user(user: DeleteEntity) {
    println!("Deleting user: {:?}", user);
    use crate::schema::users::dsl::*;

    let connection = establish_connection();
    diesel::delete(users.find(user.id))
        .execute(&connection)
        .expect("Error deleting user");
}

fn show_users() {
    use crate::schema::users::dsl::*;

    let connection = establish_connection();
    let results = users
        .load::<DBUser>(&connection)
        .unwrap();

    println!("Displaying {} users", results.len());
    for user in results {
        println!("{:?}", user);
    }
}