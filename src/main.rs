use crate::lib::Book;

use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::{fs, io};
use tabled::{Table, Tabled};
extern crate xdg;

mod lib;

fn main() -> Result<(), io::Error> {
    // TODO Genre?
    // TODO ISBN leading 0?
    // TODO Colors

    // Set DB to XDG Config
    let xdg = xdg::BaseDirectories::with_prefix("rustary").unwrap();
    let config = xdg.place_config_file("books.json")?;
    let check = Path::new(&config).exists();

    // Check if DB exists
    if !check {
        let mut config_file = File::create(&config)?;
        write!(&mut config_file, "[]")?;
    }

    // Set config path of DB
    let config: PathBuf = xdg
        .find_config_file("books.json")
        .expect("Book Database note found!");

    // Read DB file
    let mut data = read_file(&config)?;

    println!("(n) New Entry | (l) List Books | (q) Quit");
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        match input.trim() {
            "n" => {
                let book_new: Book = lib::Book::new();
                data.push(book_new);
                marshal_matters(&data, &config)?;
                println!("(n) New Entry | (l) List Books | (q) Quit");
            }
            "l" => {
                // Print DB in Table
                let table = Table::new(&data).to_string();
                println!("{}", table);
                println!("(n) New Entry | (l) List Books | (q) Quit");
            }
            "q" => break,
            _ => {
                println!("Wrong Input!");
                println!("(n) New Entry | (l) List Books | (q) Quit");
                continue;
            }
        }
    }

    // TODO push data on the json stack
    // write data back
    // marshal_matters(data, &config)?;

    // Print DB in Table
    // let table = Table::new(data).to_string();
    // println!("{}", table);

    // let book_new: Book = lib::Book::new();

    // let b2: Book = Book {
    //     title: String::from("The Way of Kings"),
    //     series: String::from("Stormlight Archive"),
    //     author: String::from("Brandon Sanderson"),
    //     genre: lib::Genres::Fantasy,
    //     tags: vec!["adventure".to_owned(), "slow".to_owned()],
    //     isbn: 3452938492,
    //     edition: lib::Editions::Hardcover,
    //     owned: true,
    //     read: true,
    //     rating: 4.5,
    // };

    // println!("{:?}", config);
    // println!("{:?}", check);
    // println!("{:?}");

    // let mut config_file = File::open(config)?;
    // println!("{:?}", config);

    // Nice json print
    // println!("{:#?}", data);

    // Search
    // for i in 0..data.len() {
    //     // Title search
    //     println!("{} {:?}", i, data[i].title.contains("The Way of Kings"));
    //     // Tag search
    //     println!("{} {:?}", i, data[i].tags.iter().any(|x| x=="slow"));
    // }
    // data.pop();

    // for i in 0..data.len() {
    //     println!("{} {:?}", i, data[i]);
    // }

    // WOW remove inside the vector fking amazing, FIXME Check for out of bounds
    // data.remove(1);
    //
    // for i in 0..data.len() {
    //     println!("{} {:?}", i, data[i]);
    // }

    // TODO check data
    // let show = read_file(&file).unwrap();
    // println!("{:?}", show);

    Ok(())
}

fn marshal_matters(obj: &Vec<Book>, file: &PathBuf) -> Result<(), io::Error> {
    fs::write(file, serde_json::to_string_pretty(&obj).unwrap())?;
    Ok(())
}

fn read_file(file: &PathBuf) -> Result<Vec<Book>, io::Error> {
    let get_data = {
        let get_data = fs::read_to_string(file)?;

        serde_json::from_str::<Vec<Book>>(&get_data).unwrap()
    };
    Ok(get_data)
}

// #[derive(Tabled)]
// struct Menu {
//     value: String,
//     name: String,
// }

// fn menu() {
//     let menu = vec![
//         Menu {
//             value: String::from("n"),
//             name: String::from("New Entry"),
//         },
//         Menu {
//             value: String::from("l"),
//             name: String::from("List Books"),
//         },
//         Menu {
//             value: String::from("q"),
//             name: String::from("Exit"),
//         },
//     ];
//
//     let table = Table::new(menu);
//     println!("{}", table);
//
//     // println!("{}", "-----------------------------------------------------------".blue());
//     // println!("1: Circle");
//     // println!("2: Triangle");
//     // println!("0: Quit");
//     // println!("{}", "-----------------------------------------------------------".blue());
// }
