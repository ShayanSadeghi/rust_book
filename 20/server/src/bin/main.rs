use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;
use std::time::Duration;

use server::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap(); // binding (connecting) to a port to listen
                                                                 // "TcpListener::bind" returns a "Result<T, E>", so we use "unwrap" to stop program on error
    let pool = ThreadPool::new(4); // create a new thread pool with 4 threads
    for stream in listener.incoming().take(2) {
        // we add ".take(2)" just for taking two job and then shutdown the server
        let stream = stream.unwrap();

        pool.execute(|| {
            // create new thread for each request. WARNING: It's a bad idea. This may cause DoS
            handle_connection(stream);
        });
    }

    println!("Shutting down.")
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024]; // size 1024 size
    stream.read(&mut buffer).unwrap(); // read bytes from TcpStream and put them in the buffer

    // the header will be like this if it request "/" URI
    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", "index.html")
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5)); // a simulated time consuming request, which make all request to wait for its response for a while
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
