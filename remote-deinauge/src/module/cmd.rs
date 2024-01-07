use std::{collections::HashMap, process::Command, thread};
#[warn(dead_code)]
pub struct RunCmd {
    cmd_list: HashMap<&'static str, &'static str>,
}

impl RunCmd {
    pub fn new() -> Self {
        let mut cmd = HashMap::new();

        cmd.insert("calculator", "gnome-calculator");
        cmd.insert("terminal", "gnome-terminal");
        cmd.insert("VScode", "code");
        cmd.insert("home", "xdg-open /home/misar/");
        cmd.insert("firefox", "firefox");
        cmd.insert("camera", "cheese");
        Self { cmd_list: cmd }
    }

    pub fn add_new_cmd(&mut self, cmd_key: &'static str, cmd_value: &'static str) {
        self.cmd_list.insert(cmd_key, cmd_value);
        println!("New cmd added {cmd_key} : {cmd_value}");
    }

    pub fn run_cmd(&self, order: &str) {
        println!("order : {:?}", order.trim());
        println!("cmd :: {:?}", self.cmd_list.get(order.trim()).cloned());
        match self.cmd_list.get(order.trim()).cloned() {
            Some(cmd) => {
                thread::spawn(move || {
                    let status = Command::new("sh").arg("-c").arg(cmd).status();
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
            }
            None => println!("Command Not find"),
        }
    }
}
