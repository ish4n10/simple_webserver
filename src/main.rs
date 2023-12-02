use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream}, fs::{OpenOptions, File},
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
    init_response();
    let content = std::fs::read_to_string("index.html").unwrap();

    let response = format!(
        "HTTP/1.1 200 OK\r\n\rContent-Length: {}\r\n\r\n{}",
        content.len(),
        content
    );

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
}

fn init_response() {
    let mut file = OpenOptions::new()
                        .read(true)
                        .write(true)
                        .create(true)
                        .open("index.html").unwrap();
    file.write(b"<html><head>THIS IS A RESPONSE</head></html>").unwrap();

}