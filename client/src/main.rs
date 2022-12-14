use std::io::{Write, Read};

fn main() {
    // socket connect to 127.0.0.1:8000 all at once
    let mut stream = std::net::TcpStream::connect("127.0.0.1:8000").unwrap();
    println!("{:?}", stream);

    let mut line = String::new();
    let once_chat = std::io::stdin().read_line(&mut line).unwrap();

    stream.write(line.as_bytes());


        let mut buf = [0; 1024];
        match stream.read(&mut buf) {
            Ok(_) => {
                let s = std::str::from_utf8(&buf).unwrap();
                println!("response, {}", s);
            }
            Err(e) => println!("failed to read from socket; err = {:?}", e),
        }



}
