use std::process::Command;

fn main() {
    Command::new("auto-editor")
        .arg("--player")
        .arg("/bin/true")
        .arg("/home/brenekh/Videos/CS321-Intro-Video.mkv")
        .status()
        .expect("auto-editor process failed to execute");
}
