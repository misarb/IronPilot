
mod server;
pub use server::Server;

mod cmd;
pub use cmd::RunCmd;

pub const OPTIONS: &str = r#"
Choose from this list :
   1. calculator
   2. terminal
   3. VScode
   4. home
   5. firefox
   6. camera
"#;






