use std::thread;
use std::collections::HashMap;
use std::process::Command;

pub struct RunCommand {
    command_list: HashMap<&'static str, &'static str>,
}

impl RunCommand {
    pub fn new() -> Self {
        let mut command_list = HashMap::new();
        command_list.insert("calculator", "gnome-calculator");
        command_list.insert("terminal", "gnome-terminal");
        command_list.insert("VScode", "code");
        command_list.insert("home", "xdg-open /home/misar/");
        command_list.insert("firefox", "firefox");
        command_list.insert("camera", "cheese");

        RunCommand { command_list }
    }

    pub fn start(&self, order: &str) {
        // Find which command I'm looking for
        if let Some(command) = self.command_list.get(order).cloned() {
            // Run command in a new thread
            println!("{}",command);
            thread::spawn(move || {
                let status = Command::new("sh")
                    .arg("-c")
                    .arg(command)
                    .status();

                match status {
                    Ok(exit_status) if exit_status.success() => {
                        println!("Command executed successfully");
                    }
                    Ok(exit_status) => {
                        eprintln!("Command failed with exit code: {:?}", exit_status.code());
                    }
                    Err(err) => {
                        eprintln!("Error executing command: {:?}", err);
                    }
                }
            });
        } else {
            eprintln!("Command not found");
        }
    }
}


