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

const OK_RESPONSE: &str = "HTTP/1.1 200 OK\rContent-Type: application/json\r\n\r\n";
const Not_FOUND: &str = "HTTP/1.1 404 NOT FOUND\r\n\r\n";

