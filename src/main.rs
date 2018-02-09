extern crate notify;

use notify::{Watcher, RecursiveMode, watcher};
use std::process::{Command, Stdio};
use std::sync::mpsc::channel;
use std::time::Duration;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let project_path = &args[1];
    let project_src = format!("{}/src", &project_path);
    let executable = format!("{}/vendor/bin/phpunit", &project_path);

    let (tx, rx) = channel();
    let mut watcher = watcher(tx, Duration::from_secs(2)).unwrap();

    watcher.watch(&project_src, RecursiveMode::Recursive).unwrap();

    println!("watching {:?}", &
        project_src);
    loop {
        match rx.recv() {
            Ok(event) => {
                println!("File changed: running executable: '{:?}'", &executable);
                println!();

                let mut command = Command::new(&executable)
                    .args(&[format!("{}/tests", &project_path)])
                    .stdout(Stdio::inherit())
                    .stderr(Stdio::inherit())
                    .spawn()
                    .unwrap();

                let status = command.wait();
            },
            Err(e) => println!("watch error:{:?}", e),
        }
    }
}

fn execute() {

}
