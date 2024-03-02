use std::net::TcpStream;
use std::io::{Read, Write};
use std::str::from_utf8;
use std::time::{Duration, Instant};

fn main() {
    let start = Instant::now();
    match TcpStream::connect("rserver:3333") {
        Ok(mut stream) => {
            println!("Successfully connected to server in port 3333");

            let msg = b"Hello!";

            stream.write(msg).unwrap();
            println!("Sent Hello, awaiting reply...");

            let mut data = [0 as u8; 6]; // using 6 byte buffer
            match stream.read_exact(&mut data) {
                Ok(_) => {
                    if &data == msg {
                        println!("Reply is ok!");
                    } else {
                        let text = from_utf8(&data).unwrap();
                        println!("Unexpected reply: {}", text);
                    }
                },
                Err(e) => {
                    println!("Failed to receive data: {}", e);
                }
            }
        },
        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }
    let duration = start.elapsed();
    println!("Terminated. Time elapsed is: {:?}", duration);
}