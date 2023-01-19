use std::fs::OpenOptions;
use std::io::Write;

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

    let mut fileRef = OpenOptions::new()
        .append(true)
        .open("../data/users.txt")
        .expect("Unable to open file");   
    
    fileRef.write_all(new_user).expect("write failed");
    fileRef.write_all("\n".as_bytes()).expect("write failed");
    println!("User created successfully");
}

fn update_user(user: UpdateUser) {
    println!("Updating user: {:?}", user);

    let db_user = User {
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
    let users = include_str!("../data/users.txt")
        .lines()
        .map(|line| {
            let x = line.split_whitespace();
            let user = User {
                id: x.next().unwrap().parse::<Uuid>().unwrap(),
                name: x.next().unwrap().to_owned(),
                email: x.next().unwrap().to_owned(),
                playlists: x.next().unwrap(),
            };
            println!("{:?}", user);
        });
}