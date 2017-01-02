use std::net::{TcpListener, TcpStream};
use std::thread;

// traits
use std::io::Read;
use std::io::Write;

fn handle_client(mut stream: TcpStream) {
    let mut buf;
    loop {
        // clear out the buffer so we don't send garbage
        buf = [0; 512];
        let _ = match stream.read(&mut buf) {
            Err(e) => panic!("Got an error: {}", e),
            Ok(m) => {
                if m == 0 {
                    // we've got an EOF
                    break;
                }
                m
            },
        };

        match stream.write(b"HTTP/1.0 200 OK\r\nContent-Type: text/plain\r\nContent-Length: 5\r\n\r\nPong!\r\n") {
            Err(_) => break,
            Ok(_) => continue,
        }
    }
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
