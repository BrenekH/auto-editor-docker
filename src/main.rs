use std::env;
use std::fs;
use std::io;
use std::path::Path;
use std::process::{Command, ExitCode};
use std::thread;

use notify::Event;
use notify::EventKind;
use notify::RecursiveMode;
use notify::Watcher;

use notify::event::AccessKind;
use notify::event::AccessMode;
use path_clean::PathClean;

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();

    if args.len() != 1 {
        return (run_auto_editor(&args[1..]).expect("auto-editor process failed to execute") as u8)
            .into();
    }

    // This unwrap is obscuring a potential error source
    let watch_dir = env::var("AUTO_EDITOR_WATCH_DIR").unwrap_or("./watch".into());
    let watch_dir = Path::new(&watch_dir).clean();

    let output_dir = env::var("AUTO_EDITOR_OUTPUT_DIR").unwrap_or("./output".into());
    let output_dir = Path::new(&output_dir).clean();

    if !watch_dir.exists() {
        if let Err(e) = fs::create_dir_all(&watch_dir) {
            println!("{e}");
            return ExitCode::FAILURE;
        }
    }

    if !output_dir.exists() {
        if let Err(e) = fs::create_dir_all(&output_dir) {
            println!("{e}");
            return ExitCode::FAILURE;
        }
    }

    let watcher = notify::recommended_watcher(move |res: Result<Event, notify::Error>| match res {
        Ok(event) => {
            if event.kind == EventKind::Access(AccessKind::Close(AccessMode::Write)) {
                for path_buf in event.paths {
                    let out_dir = output_dir.clone();
                    thread::spawn(move || {
                        let path = Path::new(&path_buf);

                        let input_file = path_buf.to_str().unwrap().into();
                        let output_file = out_dir
                            .join(path.file_name().unwrap())
                            .to_str()
                            .unwrap()
                            .into();

                        println!("Starting to process {input_file} -> {output_file}");
                        let proc_result = run_auto_editor(
                            &(vec![input_file, "--output-file".into(), output_file]),
                        );

                        match proc_result {
                            Ok(exit_code) => {
                                if exit_code == 0 {
                                    if let Err(e) = fs::remove_file(path_buf) {
                                        println!("{e}")
                                    }
                                }
                            }
                            Err(e) => println!("{e}"),
                        }
                    });
                }
            }
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

    println!("Waiting for files in {}", watch_dir.to_str().unwrap());
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

fn run_auto_editor(args: &[String]) -> io::Result<i32> {
    let mut cmd = Command::new("auto-editor");
    cmd.arg("--player").arg("/bin/true");

    for arg in args {
        cmd.arg(arg);
    }

    let status = cmd.status()?;
    Ok(status.code().unwrap_or(127))
}
