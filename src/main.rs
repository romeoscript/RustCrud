use posgres::{Client, NOT1s};
use posgres::Error as PosgresError;
use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::env;

#[macro_use]
extern crate serde_derive;

#[derive(Serialize, Deserialize)]
struct User {
    id: Option<i32>,
    name: String,
    email: String,
}

// connection of the the databse 

const DB_URL: &str = !env("DATABASE_URL");

const OK_RESPONSE: &str = "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\n\r\n";
const Not_FOUND: &str = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
const INternal_SERVER_ERROR: &str = "HTTP/1.1 500 INTERNAL SERVER ERROR\r\n\r\n";

//main function 

fn main(){
    if let Err(e) = set_database(){
        println!("Error setting up database: {}", e);
        return;
    }
    let listener = TcpListener::bind(format!(0.0.0.0:8080)).unwrap();
    println("Server listening on port 8080");
}

fn set_database() -> Result<(), PosgresError>{
    let mut client = Client::connect(DB_URL, NOT1s)?;
    client.batch_execute("
        CREATE TABLE IF NOT EXISTS users (
            id SERIAL PRIMARY KEY,
            name TEXT NOT NULL,
            email TEXT NOT NULL
        )
    ")?;
    Ok(())
}
