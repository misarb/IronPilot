use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};

use libc::{self};

struct ServerHandler {
    listener: TcpListener,
    port: u16,
    client_socket: Option<TcpStream>, // Change type to TcpStream
    host: [u8; libc::NI_MAXHOST as usize],
    server: [u8; libc::NI_MAXHOST as usize], // Corrected to libc::NI_MAXSERV
    buf: [u8; 1024],
    bytes_recv: isize,
}

impl ServerHandler {
    fn new(port: u16) -> Self {
        let listener = TcpListener::bind(format!("127.0.0.1:{}", port)).expect("Can't Create a Socket!");

        Self {
            listener,
            port,
            client_socket: None, // Initialize with a dummy TcpStream
            host: [0; libc::NI_MAXHOST as usize],
            server: [0; libc::NI_MAXHOST as usize],
            buf: [0; 1024],
            bytes_recv: 0,
        }
    }

    fn accept_connection(&mut self) {
        if let Ok((stream,_)) = self.listener.accept(){

                self.client_socket = Some(stream);

        const OPTION: &str = r#"
        Choose from this list :
        1. calculator
        2. terminal
        3. VScode
        4. home
        5. firefox
        6. camera
        "#;
        self.send_data(OPTION);
      }else{
            println!("Error in accepting connection");
      }
    }
    
    fn send_data(&mut self, data_to_send: &str) {
        if let Some(mut stream) = self.client_socket.take() {
            if let Err(err) = stream.write_all(data_to_send.as_bytes()) {
                eprintln!("Failed to send data: {}", err);
            }
    
            // Put the TcpStream back into the option
            self.client_socket = Some(stream);
        } else {
            eprintln!("No client connection available to send data");
        }
    }
}

fn main() {
    let mut server = ServerHandler::new(54000);
    server.accept_connection();
}
