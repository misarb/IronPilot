
use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
// use libc::{self};
use crate::options::OPTION;


pub struct ServerHandler {
    listener: TcpListener,
    client_socket: Option<TcpStream>, 
    // host: [u8; libc::NI_MAXHOST as usize],
    // server: [u8; libc::NI_MAXHOST as usize], 
    buf: [u8; 1024],
    bytes_recv: isize,
}

impl ServerHandler {
    pub fn init(port: u16) -> Self {
        let listener = TcpListener::bind(format!("192.168.32.84:{}", port)).expect("Can't Create a Socket!");

        Self {
            listener,
            client_socket: None, 
            // host: [0; libc::NI_MAXHOST as usize],
            // server: [0; libc::NI_MAXHOST as usize],
            buf: [0; 1024],
            bytes_recv: 0,
        }
    }

    pub fn accept_connection(&mut self) {
        if let Ok((stream,_)) = self.listener.accept(){

        self.client_socket = Some(stream);

        self.send_data(OPTION);
      }else{
            eprintln!("Error in accepting connection");
      }
    }

    fn send_data(&mut self, data_to_send: &str) {

        match self.client_socket.take() {
            Some(mut stream) => {
                if let Err(err) = stream.write_all(data_to_send.as_bytes()) {
                    eprintln!("Failed to send data: {}", err);
                }
                self.client_socket = Some(stream);
            },
            None => eprintln!("No client connection available to send data"),
        }
    }

    pub fn read_data(&mut self,buffer:&mut String) -> i32{
        //clear the buffer
        self.buf = [0; 1024];
        match self.client_socket.as_ref().map(|mut s| s.read(&mut self.buf)) {
            Some(Ok(bytes_recv)) => {
                // Store the recieved data to the buffer 
                *buffer = String::from_utf8_lossy(&self.buf[..bytes_recv]).trim().to_string();
                println!("Data Recieved: {}", buffer);
                // resend the data to the client
                self.client_socket.as_ref().map(|mut s| s.write_all(&self.buf[0..bytes_recv]));

                bytes_recv as i32
            }
            Some(Err(err)) => {
                eprintln!("Error reading data: {}", err);
                return -1; 
            }
            None => {
                eprintln!("No client connection available to read data");
                return 0;
            },
        }
    }
}