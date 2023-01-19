use std::fs::{OpenOptions, self};
use std::io::Write;

use uuid::{Uuid};

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
    
    fileRef.write_all(new_user.id.as_bytes()).expect("write failed");
    fileRef.write_all(" ".as_bytes()).expect("write failed");
    fileRef.write_all(new_user.name.as_bytes()).expect("write failed");
    fileRef.write_all(" ".as_bytes()).expect("write failed");
    fileRef.write_all(new_user.email.as_bytes()).expect("write failed");
    fileRef.write_all(" ".as_bytes()).expect("write failed");
    fileRef.write_all("[]".as_bytes()).expect("write failed");
    fileRef.write_all("\n".as_bytes()).expect("write failed");
    println!("User created successfully");
}


fn show_users() {
    let users = include_str!("../data/users.txt")
        .lines()
        .map(|line| {
            let mut x = line.split_whitespace();
            let user = User {
                id: x.next().unwrap().parse::<Uuid>().unwrap(),
                name: x.next().unwrap().to_owned(),
                email: x.next().unwrap().to_owned(),
                playlists: [].to_vec()
            };
            println!("{:?}", user);
        });
}

fn update_user(user: UpdateUser) {
    todo!()
}

fn delete_user(user: DeleteEntity) {
    todo!()
}