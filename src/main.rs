use std::env;
use std::process::{Command, ExitCode};

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();

    if args.len() != 1 {
        let mut cmd = Command::new("auto-editor");
        cmd.arg("--player").arg("/bin/true");

        for arg in args {
            cmd.arg(arg);
        }

        let status = cmd.status().expect("auto-editor process failed to execute");
        return (status.code().unwrap_or(127) as u8).into();
    }

    println!("TODO: Run as watcher daemon");

    ExitCode::SUCCESS
}
