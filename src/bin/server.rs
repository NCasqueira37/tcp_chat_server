use std::{io, thread};
use std::{
    io::Read,
    net::{TcpListener, TcpStream},
};

fn main() {
    create_listener();
}

fn get_addr() -> String {
    println!("Server Address:");
    let mut buf = String::new();
    let _ = io::stdin().read_line(&mut buf);
    buf.trim().to_string()
}

fn create_listener() {
    let listener = loop {
        let addr = get_addr();
        let listener = TcpListener::bind(&addr);
        match listener {
            Ok(listener) => break listener,
            Err(_e) => {
                println!("Address not valid, try again.");
                get_addr();
            }
        }
    };

    // Show address in console
    println!("Server started on: {:?}", listener.local_addr().unwrap());

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                // Handle each stream in a different thread
                println!("{} CONNECTED", stream.local_addr().unwrap());
                thread::spawn(move || {
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
