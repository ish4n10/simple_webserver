use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};


fn main() {

    const HOST: &str = "127.0.0.1";
    const PORT: &str = "6969";

    let endpoint: String = HOST.to_owned() + ":" + PORT;
    
    let listener = 
        TcpListener::bind(endpoint).unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        println!("Connection established");
        handle_connections(stream);
    }
    println!("Hello, world!");
}

fn handle_connections(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
}