use serde::{Deserialize, Serialize};
use std::io::{self, Write};
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter};
use tabled::Tabled;

#[derive(Deserialize, Serialize, Debug, EnumIter, Display)]
pub enum Genres {
    Adventure,
    Code,
    Fantasy,
    NonFiction,
    ScienceFiction,
}

#[derive(Deserialize, Serialize, Debug, EnumIter, Display)]
pub enum Editions {
    Audio,
    Digital,
    Hardcover,
    Paperback,
}

#[derive(Deserialize, Serialize, Debug, Tabled)]
pub struct Tags {
    string: String
}

#[derive(Deserialize, Serialize, Debug, Tabled)]
pub struct Book {
    pub title: String,
    pub series: String,
    pub author: String,
    pub genre: Genres,
    #[tabled(skip)]
    pub tags: Vec<String>,
    #[tabled(skip)]
    pub isbn: u64,
    pub edition: Editions,
    pub owned: bool,
    pub read: bool,
    pub rating: f32,
}

impl Book {
    pub fn new() -> Book {
        Book {
            title: title(),
            series: series(),
            author: author(),
            genre: genre(),
            tags: tags(),
            isbn: isbn(),
            edition: edition(),
            owned: owned(),
            read: read(),
            rating: rating(),
        }
    }
}

fn title() -> String {
    let mut buf: String = String::new();
    print!("Title: ");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut buf)
        .expect("Error reading Title");
    let buf: String = buf.trim().to_owned();
    buf
}

fn series() -> String {
    let mut buf: String = String::new();
    print!("Series: ");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut buf)
        .expect("Error reading Series");
    let buf: String = buf.trim().to_owned();
    buf
}

fn author() -> String {
    let mut buf: String = String::new();
    print!("Author: ");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut buf)
        .expect("Error reading Series");
    let buf: String = buf.trim().to_owned();
    buf
}

fn genre() -> Genres {
    let mut buf: String = String::new();
    let mut x = 1;
    for genre in Genres::iter() {
        println!("{} => {:?}", x, genre);
        x += 1;
    }

    let buf: Genres = loop {
        print!("Genre: ");
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut buf)
            .expect("Error reading Genre");
        match buf.trim() {
            "1" => break Genres::Adventure,
            "2" => break Genres::Code,
            "3" => break Genres::Fantasy,
            "4" => break Genres::NonFiction,
            "5" => break Genres::ScienceFiction,
            _ => {
                println!("Wrong Input!");
                buf.clear();
                continue;
            }
        }
    };
    buf
}

fn tags() -> Vec<String> {
    let mut buf: String = String::new();
    print!("Tags: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut buf).expect("Error reading Tags");
    let buf: Vec<String> = buf
        .trim()
        .split_whitespace()
        .map(|x| x.to_owned())
        .collect();
    buf
}

fn isbn() -> u64 {
    let mut buf: String = String::new();
    let buf: u64 = loop {
        print!("ISBN: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut buf).expect("Error reading ISBN");
        match buf.trim().parse() {
            Ok(v) => break v,
            Err(_) => {
                println!("Not a ISBN!");
                buf.clear();
                continue;
            }
        }
    };
    buf
}

fn edition() -> Editions {
    let mut buf: String = String::new();
    let mut x = 1;
    for edition in Editions::iter() {
        println!("{} => {:?}", x, edition);
        x += 1;
    }

    let buf: Editions = loop {
        print!("Edition: ");
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut buf)
            .expect("Error reading Genre");
        match buf.trim() {
            "1" => break Editions::Audio,
            "2" => break Editions::Digital,
            "3" => break Editions::Hardcover,
            "4" => break Editions::Paperback,
            _ => {
                println!("Wrong Input!");
                buf.clear();
                continue;
            }
        }
    };
    buf
}

fn owned() -> bool {
    let mut buf: String = String::new();
    println!("1 => true");
    println!("2 => false");
    let buf: bool = loop {
        print!("Owned: ");
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut buf)
            .expect("Error reading Status");
        if buf.trim() == "1" {
            break true;
        } else if buf == "2" {
            break false;
        } else {
            println!("Wrong Input!");
            buf.clear();
            continue;
        }
    };
    buf
}

fn read() -> bool {
    let mut buf: String = String::new();
    println!("1 => true");
    println!("2 => false");
    let buf: bool = loop {
        print!("Read: ");
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut buf)
            .expect("Error reading Status");
        if buf.trim() == "1" {
            break true;
        } else if buf == "2" {
            break false;
        } else {
            println!("Wrong Input!");
            buf.clear();
            continue;
        }
    };
    buf
}

fn rating() -> f32 {
    let mut buf: String = String::new();
    print!("Rating: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut buf).expect("Error reading ISBN");
    let buf: f32 = loop {
        match buf.trim().parse() {
            Ok(v) => break v,
            Err(_) => {
                println!("Not an ISBN!");
                continue;
            }
        }
    };
    buf
}
