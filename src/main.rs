mod options;
mod run_command;
mod server;
use crate::run_command::RunCommand;
use crate::server::ServerHandler;

fn main() {
    let mut buffer = String::new();
    let mut server = ServerHandler::init(54000);
    server.accept_connection();
    loop {
        server.read_data(&mut buffer);
        if buffer.trim() == "exit" {
            break;
        }
        let run_command = RunCommand::new();
        run_command.start(&buffer);

        buffer.clear();
    }
    // server.read_data(&mut buffer);
    // let run_command = RunCommand::new();
    // run_command.start(&buffer);
    // // Sleep to allow threads to finish before exiting (optional)
    // thread::sleep(std::time::Duration::from_secs(2));
}
