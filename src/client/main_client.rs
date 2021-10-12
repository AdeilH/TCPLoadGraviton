use std::net::{TcpStream};
use std::io::{Read, Write};
use std::str::from_utf8;
use std::{thread, time};
use std::env;
use rand::Rng;

fn client(address: String){
    let mut rng = rand::thread_rng();
    let message_string = String::from("Hello from ") + &rng.gen_range(0..1000).to_string();
    let msg = message_string.as_bytes();
    match TcpStream::connect(address.clone()) {
       
        Ok(mut stream) => {
            println!("Successfully connected to server in port {}", address);
            loop{
            stream.write(msg).unwrap();
            println!("Sent Hello, awaiting reply...");
            let mut data = [0 as u8; 20]; // using 6 byte buffer
            match stream.read(&mut data) {
                Ok(_) => {
                        
                        let text = from_utf8(&data).unwrap();
                        println!("Reply Received: {} from: {}", text, stream.peer_addr().unwrap());
                        thread::sleep(time::Duration::from_secs(1));
                        // println!();

                },
                Err(e) => {
                    println!("Failed to receive data: {}", e);
                }
            }
        }
        },
        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }
    println!("Terminated.");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    loop{
    let port: String = args[1].to_string().clone();
    thread::spawn(|| {
        client(port);
    });
    thread::sleep(time::Duration::from_millis(300));
}
}