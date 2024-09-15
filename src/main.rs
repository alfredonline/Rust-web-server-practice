use std::fs;
use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();

    let get_root = b"GET / HTTP/1.1\r\n";
    let get_root_html = b"GET /index.html HTTP/1.1\r\n";
    let get_about = b"GET /about HTTP/1.1\r\n";
    let get_about_html = b"GET /about.html HTTP/1.1\r\n";

    // Serve "index.html" for both "/" and "/index.html" paths
    if buffer.starts_with(get_root) || buffer.starts_with(get_root_html) {
        let contents = fs::read_to_string("index.html").unwrap();
        let response = format!("HTTP/1.1 200 OK\r\n\r\n{}", contents);
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();

    // Serve "about.html" for both "/about" and "/about.html" paths
    } else if buffer.starts_with(get_about) || buffer.starts_with(get_about_html) {
        let contents = fs::read_to_string("about.html").unwrap();
        let response = format!("HTTP/1.1 200 OK\r\n\r\n{}", contents);
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();

    // If the requested path is not found, return a 404 error
    } else {
        let status_line = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
        let contents = fs::read_to_string("404.html").unwrap();
        let response = format!("{}\r\n\r\n{}", status_line, contents);
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }


    


}
