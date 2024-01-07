mod module;
use crate::module::RunCmd;
use crate::module::Server;
#[warn(unused_imports, unused_variables)]
#[tokio::main]
async fn main() {
    let mut buffer = String::new();
    let mut server = Server::start_server(8888).await;
    let mut cmd = RunCmd::new();
    cmd.add_new_cmd("carla", "/home/misar/Desktop/carla/CarlaUE4.sh");
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

