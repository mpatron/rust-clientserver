use chrono::{DateTime, Local, Utc};
use std::io::{Read, Write};
use std::net::TcpStream;
use std::str::from_utf8;
use std::time::Instant;

fn main() {
    let start = Instant::now();

    let local_time = Local::now();
    let utc_time = DateTime::<Utc>::from_naive_utc_and_offset(local_time.naive_utc(), Utc);
    println!("Local time now is {}", local_time);
    println!("UTC time now is {}", utc_time);

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
                }
                Err(e) => {
                    println!("Failed to receive data: {}", e);
                }
            }
        }
        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }
    let duration = start.elapsed();
    println!("Terminated. Time elapsed is: {:?}", duration);
}
