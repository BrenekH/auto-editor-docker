use std::env;
use std::path::Path;
use std::process::{Command, ExitCode};

use notify::RecursiveMode;
use notify::Watcher;

use path_clean::PathClean;

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

    // This unwrap is obscuring a potential error source
    let watch_dir = env::var("AUTO_EDITOR_WATCH_DIR").unwrap_or("./".into());
    let watch_dir = Path::new(&watch_dir).clean();

    if !watch_dir.exists() {
        if let Err(e) = std::fs::create_dir_all(&watch_dir) {
            println!("{e}");
            return ExitCode::FAILURE;
        }
    }

    let watcher = notify::recommended_watcher(|res| match res {
        Ok(event) => {
            println!("{event:?}");
        }
        Err(e) => {
            println!("{e}");
        }
    });

    let mut watcher = match watcher {
        Ok(w) => w,
        Err(e) => {
            println!("{e}");
            return ExitCode::FAILURE;
        }
    };

    match watcher.watch(&watch_dir, RecursiveMode::Recursive) {
        Err(e) => {
            println!("{e}");
            return ExitCode::FAILURE;
        }
        _ => {}
    }

    loop {
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
