use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

fn main() {
    const SERVADDR: &str = "127.0.0.1:7878";
    let listener: TcpListener = TcpListener::bind(&SERVADDR).unwrap();
    println!("The server has been started on {}", &SERVADDR);

    for stream in listener.incoming() {
        let stream: TcpStream = stream.unwrap();
        handle_connection(stream);
        println!("Connection established!");
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    let response: &str = "HTTP/1.1 200 OK\r\n\r\n";

    stream.write_all(response.as_bytes()).unwrap();
    println!("Response sent: {}", response);
    println!("Request: {:#?}", http_request)
}
