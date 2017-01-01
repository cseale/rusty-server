use std::net::{TcpListener, TcpStream};
use std::thread;

// traits
use std::io::Write;

fn handle_client(mut stream: TcpStream) {
    stream.write(b"Hello World\n");
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8888").unwrap();
    for stream in listener.incoming() {
        match stream {
            Err(e) => { println!("failed: {}", e) }
            Ok(stream) => {
                thread::spawn(move || {
                    handle_client(stream)
                });
            }
        }
    }
}
