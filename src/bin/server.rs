use std::{io::Read, net::{TcpListener, TcpStream}};
use std::thread;

fn main(){
    create_listener(String::from("127.0.0.1:80"));
}

pub fn create_listener(addr: String) {
    let listener = TcpListener::bind(addr).unwrap();
    
    loop {
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    thread::spawn(|| {
                        // Handle each stream in a different thread
                        handle_stream(stream);
                    });
                }
                Err(e) => {
                    println!("ERROR: {e}");
                }
            }
        }
    }
    
}

fn handle_stream(mut stream: TcpStream) {
    let mut buffer = [0u8; 1024];
    let bytes_read = stream.read(&mut buffer);
    println!("{:?}: {}", stream.local_addr(), String::from_utf8_lossy(&buffer[..bytes_read.unwrap()]));
}