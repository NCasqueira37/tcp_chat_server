use std::thread;
use std::{
    io::Read,
    net::{TcpListener, TcpStream},
};

fn main() {
    create_listener(String::from("127.0.0.1:80"));
}

pub fn create_listener(addr: String) {
    let listener = TcpListener::bind(addr).unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move || {
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

fn handle_stream(mut stream: TcpStream) {
    let mut buffer = [0u8; 1024];
    loop {
        let bytes_read = stream.read(&mut buffer);
        match bytes_read {
            Ok(r) => {
                // Print data from user to console
                println!(
                    "{:?}(size: {} Bytes): {}",
                    stream.local_addr(),
                    r,
                    String::from_utf8_lossy(&buffer[..r])
                );
            }
            Err(e) => {
                println!("{:?} DISCONNECTED ({})", stream.local_addr(), e);
                break;
            }
        }
    }
}
