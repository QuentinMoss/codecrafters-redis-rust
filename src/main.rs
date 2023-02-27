use std::net::TcpListener;
use std::io::{Write, Read};

// Client will only send PING for now, if we receive a PING, respond with PONG
// Takes zero or one arguments
// If there are more than two arguments return an error 
// If there are zero arguments return PONG 
// If there is one argument return the argument the client sent
//
//
// First pass is to simply return PONG if client sends PING

// For Simple Strings, the first byte of the reply is "+"
// For Errors, the first byte of the reply is "-"
// For Integers, the first byte of the reply is ":"
// For Bulk Strings, the first byte of the reply is "$"
// For Arrays, the first byte of the reply is "*"

fn handle_client(mut stream: std::net::TcpStream) -> std::io::Result<()> {
    let mut buf = [0; 512];

    loop {
        let bytes_read = match stream.read(&mut buf) {
            Ok(bytes) => bytes,
            Err(e) => return Err(e)
        };

        if bytes_read == 0 {
            println!("Client closed connection?");
            break;
        } 

        stream.write(b"+PONG\r\n").unwrap();
    }
    Ok(())

}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();
    
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_client(stream);   
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
    Ok(())
}
