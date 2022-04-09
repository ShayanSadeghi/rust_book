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

    // the header will be like this if it request "/" URI
    let get = b"GET / HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", "index.html")
    } else {
        ("HTTP/1.1 404 NOT", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();

    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );

    stream.write(response.as_bytes()).unwrap(); // convert response to bytes
    stream.flush().unwrap(); // "flush" will wait and prevent the program from continuing until all the bytes are written to the connection
}
