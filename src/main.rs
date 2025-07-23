use std::io::{BufRead, Write};
use web_server::ThreadPool;

fn main() {
    let listener = std::net::TcpListener::bind("127.0.0.1:5424").unwrap();
    for stream in listener.incoming() {
        let mut stream = stream.unwrap();

        let reader = std::io::BufReader::new(&stream);

        let http_request: Vec<_> = reader
            .lines()
            .map(|result| result.unwrap())
            .take_while(|line| !line.is_empty())
            .collect();

        println!("Request: {http_request:#?}");
        let request_line = http_request.get(0).unwrap();
        println!("{request_line}");

        let (filename, status) = if request_line == "GET / HTTP/1.1" {
            ("index.html", "200 OK")
        } else {
            ("error.html", "404 NOT FOUND")
        };

        let contents = std::fs::read_to_string(filename).unwrap();
        let len = contents.len();

        let response = format!("HTTP/1.1 {status}\r\nContent-Length: {len}\r\n\r\n{contents}");

        stream.write_all(response.as_bytes()).unwrap();
    }
}
