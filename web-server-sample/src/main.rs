use std::{net::{TcpListener, TcpStream}, io::{BufReader, BufRead, Write}};

fn main() {
    // 127.0.0.1:7878でbindできなかったら、その時点でunwrapでプログラム終了する
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for _stream in listener.incoming() {
        let stream = _stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    println!("Request: {:#?}", http_request);

    let response = "HTTP/1.1 200 OK¥r¥n¥r¥n";
    stream.write_all(response.as_bytes()).unwrap();
}