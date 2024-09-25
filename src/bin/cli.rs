use clap::{Arg, Command};

extern crate enigma_api;

#[tokio::main]
async fn main() {
    let matches = Command::new("cli")
        .about("Training Management System")
        .arg_required_else_help(true)
        .subcommand(
            Command::new("users")
            .about("User Management")
            .arg_required_else_help(true)
            .subcommand(
                Command::new("create")
                .about("Create a new user")
                .arg_required_else_help(true)
                .arg(Arg::new("email").required(true))
                .arg(Arg::new("password").required(true))
                .arg(Arg::new("role_codes").required(true).num_args(1..).value_delimiter(','))
            )
            .subcommand(
                Command::new("index")
                .about("List of users")
            )
            .subcommand(
                Command::new("delete")
                .about("Delete user by id")
                .arg_required_else_help(true)
                .arg(Arg::new("id").required(true))
            )
        )
        .subcommand(
            Command::new("roles")
            .about("Role Management")
            .arg_required_else_help(true)
            .subcommand(
                Command::new("create")
                .about("Create a new role")
                .arg_required_else_help(true)
                .arg(Arg::new("code").required(true))
                .arg(Arg::new("name").required(true))
            )
            .subcommand(
                Command::new("index")
                .about("List of roles")
            )
            .subcommand(
                Command::new("delete")
                .about("Delete role by id")
                .arg_required_else_help(true)
                .arg(Arg::new("id").required(true))
            )
        )
        .get_matches();

    match matches.subcommand() {
        Some(("users", sub_matches)) => match sub_matches.subcommand() {
            Some(("create", create_matches)) => {
                enigma_api::commands::create_user(
                    create_matches.get_one::<String>("email").unwrap().to_owned(), 
                    create_matches.get_one::<String>("password").unwrap().to_owned(),
                    create_matches.get_many::<String>("role_codes").unwrap().map(|v| v.to_owned()).collect(),  
                ).await
            },
            Some(("index", _)) => {
                enigma_api::commands::index_users().await;
            },
            Some(("delete", _)) => {
                enigma_api::commands::delete_user(
                    sub_matches.get_one::<i32>("id")
                        .unwrap().to_owned()
                ).await;
            },
            _ => {},
        },
        Some(("roles", sub_matches)) => match sub_matches.subcommand() {
            Some(("create", create_matches)) => {
                enigma_api::commands::create_role(
                    create_matches.get_one::<String>("code").unwrap().to_owned(), 
                    create_matches.get_one::<String>("name").unwrap().to_owned(), 
                ).await
            },
            Some(("list", _)) => {
                enigma_api::commands::index_roles().await;
            },
            Some(("delete", _)) => {
                enigma_api::commands::delete_role(
                    sub_matches.get_one::<i32>("id")
                        .unwrap().to_owned()
                ).await;
            },
            _ => {},
        },
        _ => {},
    }
}