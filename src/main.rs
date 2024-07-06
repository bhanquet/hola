use hola::ThreadPool;
use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    path::Path,
    thread,
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    let available_parallelism = thread::available_parallelism().unwrap();
    println!("Starting with {available_parallelism} thread");
    let pool = ThreadPool::new(available_parallelism);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });

    }

    println!("Shutting down.");
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();


    let (status_line, filename) = if request_line == "GET / HTTP/1.1" {
        ("HTTP/1.1 200 OK", Path::new("html").join("index.html"))
    } else {
        ("HTTP/1.1 404 NOT FOUND", Path::new("html").join("404.html"))
    };

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();

}