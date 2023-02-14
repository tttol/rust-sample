use std::net::TcpListener;

fn main() {
    // 127.0.0.1:7878でbindできなかったら、その時点でunwrapでプログラム終了する
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for _stream in listener.incoming() {
        let stream = _stream.unwrap();
        println!("Connection established!")
    }
}
