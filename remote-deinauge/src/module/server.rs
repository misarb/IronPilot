use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};

use crate::module::OPTION;

pub struct Server {
    client_socket: TcpStream,
    pub buffer: [u8; 1023],
    pub recived_bytes: usize,
}

impl Server {
    pub async fn start_server(port: u16) -> Self {
        let listener = TcpListener::bind(format!("127.0.0.1:{}", port))
            .await
            .unwrap();
        let (socket, _) = listener.accept().await.unwrap();

        Self {
            client_socket: socket,
            buffer: [0; 1023],
            recived_bytes: 0,
        }
    }

    pub async fn read_data(&mut self) {
        let (mut reader, mut writer) = self.client_socket.split();
        writer
            .write_all(OPTION.to_string().as_bytes())
            .await
            .unwrap();
        self.recived_bytes = reader.read(&mut self.buffer).await.unwrap();
    }
}
