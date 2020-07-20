use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;
use std::time::Duration;

use hello::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(5);

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let response = if buffer.starts_with(get) {
        let content = fs::read_to_string("hello.html").unwrap();

        format!("HTTP/1.1 200 OK\r\n\r\n{}", content)
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        format!("HTTP/1.1 200 OK\r\n\r\n")
    } else {
        format!("HTTP/1.1 404 NOT FOUND\r\n\r\n")
    };

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
