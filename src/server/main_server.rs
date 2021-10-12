use std::thread;
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Read, Write};
use std::str::from_utf8;
use std::env;

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0 as u8; 64];
    while match stream.read(&mut buffer) {
        Ok(size) => {
            let message = from_utf8(&buffer).unwrap();
            let message_rcvd = message.split(" ");
            let vec: Vec<&str> = message_rcvd.collect();
            // println!("Message Received: {} from: {}", message, stream.peer_addr().unwrap());
            let msg = String::from("Hi to ") + vec.last().unwrap();
            let message = msg.as_bytes();
            stream.write(&message[0..size]).unwrap();
            true
        },
        Err(_) => {
            println!("An error occurred, terminating connection with {}", stream.peer_addr().unwrap());
            stream.shutdown(Shutdown::Both).unwrap();
            false
        }
    } {}
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let listener = TcpListener::bind(&args[1]).unwrap();
    let mut number_of_clients = 0;
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move|| {
                    handle_client(stream)
                });
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
        number_of_clients = number_of_clients + 1; 
        println!("Number of Clients: {}", number_of_clients);
    }
    
     drop(listener);
}