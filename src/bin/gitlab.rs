#[macro_use]
extern crate clap;
extern crate gitlab;
extern crate reqwest;

use std::env;
use reqwest::Client;
use gitlab::{Gitlab, Credentials};
use gitlab::projects::{SingleProjectOptions, GetProjectUsersOptions, ProjectParams};

fn main() {
    let matches = clap_app!(app =>
        (version: crate_version!())
        (author: crate_authors!())
        (about: "Gitlab CLI")
        (@arg host: -h --host +takes_value "Set Gitlab host")
        (@subcommand getproject =>
            (about: "Get a project by ID, name, or namespace path")
            (@arg name: "ID or name of project to retrieve")
        )
        (@subcommand gitignore =>
            (about: "List or retrieve gitignore templates")
            (@arg list: -l --list "Fetch all available gitignore template names")
            (@arg template: "Name of template to retrieve")
        )
        (@subcommand listusers =>
            (about: "List users of a project")
            (@arg name: "ID or name of project to retrieve")
        )
        (@subcommand createproject =>
            (about: "Create a new project")
            (@arg name: "Name of new project")
        )
        (@subcommand deleteproject =>
            (about: "Delete a project")
            (@arg name: "Name of project to delete")
        )
    ).get_matches();

    let credentials = match env::var("GITLAB_ACCESS_TOKEN") {
        Ok(token) => Credentials::AccessToken(token),
        _ => Credentials::Anonymous,
    };

    let host = matches.value_of("host").unwrap_or("https://gitlab.com");

    println!("Gitlab host: {}", host);
    println!("Credentials used: {:?}", credentials);

    let gitlab = Gitlab::new(
        host,
        Client::new().unwrap(),
        credentials);

    if let Some(matches) = matches.subcommand_matches("gitignore") {
        let gi = gitlab.gitignores();
        let template = matches.value_of("template").unwrap();

        let gitignore_template = gi.single_template(template).unwrap();

        println!("{:?}", gitignore_template);
    }

    if let Some(matches) = matches.subcommand_matches("getproject") {
        let projects = gitlab.projects();

        let name = matches.value_of("name").unwrap();

        let spo = SingleProjectOptions::builder(name).statistics(true).build();
        println!("{:?}", projects.project(&spo));
    }

    if let Some(matches) = matches.subcommand_matches("listusers") {
        let projects = gitlab.projects();
        let name = matches.value_of("name").unwrap();
        let b = GetProjectUsersOptions::builder(name).build();
        println!("{:?}", projects.users(&b));
    }

    if let Some(matches) = matches.subcommand_matches("createproject") {
        let projects = gitlab.projects();
        let name = matches.value_of("name").unwrap();
        let p = ProjectParams::builder(name).build();
        println!("{:?}", projects.create(&p));
    }

    if let Some(matches) = matches.subcommand_matches("deleteproject") {
        let projects = gitlab.projects();
        let name = matches.value_of("name").unwrap();
        println!("{:?}", projects.delete(name));
    }
}