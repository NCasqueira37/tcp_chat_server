use std::io;
use std::io::Write;
use std::net::TcpStream;

fn main() {
    let addr = String::from("127.0.0.1:80");
    let mut stream = TcpStream::connect(addr);
    let mut buf: String = String::from("");

    loop {
        // Clear buffer
        buf.clear();
        match io::stdin().read_line(&mut buf) {
            Ok(_n) => {
                println!("Message: {}", buf);
            }
            Err(err) => {
                println!("{}", err.kind());
            }
        }

        match &mut stream {
            Ok(stream) => {
                stream.write_all(buf.as_bytes()).unwrap_or_else(|err| {
                    println!("{}", err.kind());
                });
            }
            Err(e) => {
                println!("{}", e.kind());
            }
        }
    }
}
