use std::net::{TcpListener, TcpStream};
use std::thread;

// traits
use std::io::Read;
use std::io::Write;

// Got to look after that client yo
fn handle_client(mut stream: TcpStream) {
    let mut buf;
    loop {
        // clear out the buffer so we don't send garbage
        buf = [0; 512];
        // First of all we read from the socket
        // this method will block until all bytes are read
        // and we recieve an EOF signal of 0
        // (could also be content length was 0 of bytes recieved through TCP)
        let _ = match stream.read(&mut buf) {
            Err(e) => panic!("Got an error: {}", e), // lets build a 500 error response here
            Ok(m) => {
                if m == 0 {
                    // we've got an EOF, stop blocking the program
                    break;
                }
                m // append m to the reference buf
            },
        };
        // this sends a static HTTP reponse with 'Pong!' as the payload
        // after this write completes, we again loop around
        // match does a comparison to check the result returned from stream.write
        // if OK, continue, if Error, break
        match stream.write(b"HTTP/1.0 200 OK\r\nContent-Type: text/plain\r\nContent-Length: 5\r\n\r\nPong!\r\n") {
            Err(_) => break, // break, handle with 500 error reponse
            Ok(_) => continue, // continue the loop
        }
    }
}

// Our main program
fn main() {
    let listener = TcpListener::bind("127.0.0.1:8888").unwrap(); // bind to the TCP socket
    for stream in listener.incoming() { // listen for connections
        match stream {
            Err(e) => { println!("failed: {}", e) } // don't panic! otherwise we bring the server down
            Ok(stream) => {
                // spawn a new thread for each connection.
                // later we will move to a more event driven model
                // possibly using mio library (Metal IO)
                thread::spawn(move || {
                    handle_client(stream)
                });
            }
        }
    }
}
