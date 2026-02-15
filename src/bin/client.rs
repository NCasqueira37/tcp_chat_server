use std::net::TcpStream;
use std::io::Write;

fn main() {
    let addr = String::from("127.0.0.1:80");
    let stream = TcpStream::connect(addr);
    match stream {
        Ok(mut stream) => {
            let buf = b"hello world";
            stream.write(buf).unwrap();
        }
        Err(e) => {
            println!("{}", e);
        }
    }
}