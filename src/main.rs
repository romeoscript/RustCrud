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

    for stream in listener.incoming(){
        match stream{
            Ok(mut stream) => {
                handle_connection(&mut stream);
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    } 
}

fn handle_client(mut stream: TcpStream){
    let mut buffer = [0; 1024];
    let mut request = String::new();

    match stream.read(&mut buffer){
        Ok(size) => {
            request.push_str(&String::from_utf8_lossy(&buffer[..size].as_ref()));

            let (status_line, content) = match &*request{
                r if request_with("POST /users") => handle_post_request(r),
                r if request_with("GET /users") => handle_get_request(r),
                r if request_with("GET /users") => handle_get_all_request(r),
                r if request_with("PUT /users") => handle_put_request(r),
                r if request_with("DELETE /users") => handle_delete_request(r),
                _ = (Not_FOUND, "Not Found".to_string()),
            }
            stream.write_all(format! ("{}{}",status_line, content).as_bytes()).unwrap();
        }
        
        Err(e) => {
            println!("Error reading from connection: {}", e);
            return;
        }
    }
   
}

fn handle_post_request(request: &str) -> (&str, String){
    match (get_user_request_body(&request), Client::connect(DB_URL, NOT1s)){
        (Ok(user), Ok(mut client)) => {
            match client.execute(
                "INSERT INTO users (name, email) VALUES ($1, $2)",
                &[&user.name, &user.email]
            ).unwrap();
            (OK_RESPONSE, "User created".to_string())
        }
        _ =>  (INternal_SERVER_ERROR, "Internal Server Error".to_string()),
    }
}

fn handle_get_request(request: &str) -> (&str, String){
    match (get_id(&request).parse::<i32>, Client::connect(DB_URL, NOT1s)){
        (id, Ok(mut client)) => {
            match client.query("SELECT * FROM users WHERE id = $1", &[&id]){
                Ok(row) => {
                   let user = User{
                       id: row.get(0),
                       name: row.get(1),
                       email: row.get(2),
                   };
                }
               (OK_RESPONSE.to_string(), serde_json::to_string(&user).unwrap())
            }
        }
        _ => (INternal_SERVER_ERROR, "User not found".to_string()),
    }
}


fn handle_get_all_request(request: &str) -> (&str, String){
    match Client::connect(DB_URL, NOT1s){
        Ok(mut client) => {
                    let mut users = Vec::new();

                    for row in client.query ("SELECT * FROM users", &[]).unwrap(){
                        users.push(User{
                            id: row.get(0),
                            name: row.get(1),
                            email: row.get(2),
                        });
                    }
                    (OK_RESPONSE, serde_json::to_string(&users).unwrap())
                }
        _ => (INternal_SERVER_ERROR, "Internal Server Error".to_string()),
    }
}
 fn handle_put_request(request: &str) -> (&str, String){
     match (
        get_id(&request).parse::<i32>,
        get_user_request_body(&request), 
        Client::connect(DB_URL, NOT1s),
    ){ 
         (Ok(id), Ok(user), Ok(mut client)) => {
              client.execute("UPDATE users SET name = $1, email = $2 WHERE id = $3", &[&user.name, &user.email, &id]
            ).unwrap()
            
               (OK_RESPONSE.to_string(), "User updated".to_string()),  
         }
         _ => (INternal_SERVER_ERROR, "Internal Server Error".to_string()),
     }
 }

 fn handle_delete_request(request: &str) -> (&str, String){
     match (get_id(&request).parse::<i32>, Client::connect(DB_URL, NOT1s)){
         (Ok(id), Ok(mut client)) => {
             let rows_affect =  client.execute("DELETE FROM users WHERE id = $1", &[&id]).unwrap()
                if rows_affect == 0 {
                    return (Not_FOUND, "User not found".to_string());
                }
                  (OK_RESPONSE.to_string(), "User deleted".to_string()),
             
         }
         _ => (INternal_SERVER_ERROR, "Internal Server Error".to_string()),
     }
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

fn get_id(request:&str)-> &str {
    request.split("/").nth(1).unwrap_or_default().split_whitespace().next().unwrap_or_default()  
}


//desrialize user from request body twith the id 
fn get_user_request_body(request: &str) -> Result<User, serde_json::Error>{
    let body = request.split("\r\n\r\n").nth(1).unwrap_or_default();
    serde_json::from_str(body)
}
