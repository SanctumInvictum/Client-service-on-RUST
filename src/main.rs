use std::thread;
use std::time::Duration;
use std::process::Command;

fn main() {

    let mut line1 = thread::spawn(move || {
        let com = Command::new("cmd")
            .args(&["/C", "cargo", "run", "--bin", "model"])
            .output()
            .expect("failed to execute process");
    });

    let mut line2 = thread::spawn(move || {
        let com = Command::new("cmd")
            .args(&["/C", "cargo", "run", "--bin", "interface"])
            .output()
            .expect("failed to execute process");
    });

    line1.join().unwrap();
    line2.join().unwrap();

}
