use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

fn main() {
    // listen on the port
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    
    // for each string that comes in handle the stream
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        println!("Connection established");

        handle_connection(stream);
    }
}

// this is the stream handler function
fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];

    stream.read(&mut buffer).unwrap();
    
    // print the stream to the stdout
    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
}
