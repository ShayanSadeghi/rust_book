use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap(); // binding (connecting) to a port to listen
                                                                 // "TcpListener::bind" returns a "Result<T, E>", so we use "unwrap" to stop program on error
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024]; // size 1024 size

    stream.read(&mut buffer).unwrap(); // read bytes from TcpStream and put them in the buffer
                                       //    Printing Request
    println!("Requset: {}", String::from_utf8_lossy(&buffer[..])); // convert the bytes in the buffer and print that string

    // Make and send a response
    // read the HTML file
    let contents = fs::read_to_string("index.html").unwrap();
    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
        contents.len(),
        contents
    );

    stream.write(response.as_bytes()).unwrap(); // convert response to bytes
    stream.flush().unwrap(); // "flush" will wait and prevent the program from continuing until all the bytes are written to the connection
}