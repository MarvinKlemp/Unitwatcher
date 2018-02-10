extern crate notify;

use notify::{Watcher, RecursiveMode, RawEvent, raw_watcher};
use std::process::{Command, Stdio};
use std::sync::mpsc::channel;
use std::time::Duration;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let project_path = &args[1];
    let watch_path = format!("{}/tests", &project_path);
    let executable_path = format!("{}/vendor/bin/phpunit", &project_path);

    let (tx, rx) = channel();
    let mut watcher = raw_watcher(tx).unwrap();

    watcher.watch(&watch_path, RecursiveMode::Recursive).unwrap();

    println!("watching {:?}", &watch_path);
    loop {
        match rx.recv() {
            Ok(RawEvent{path: Some(path), op: Ok(op), cookie}) => {
                let mut command = Command::new(&executable_path)
                    .args(&[
                        &watch_path,
                        &format!("--filter={:?}", &path.file_stem().unwrap())
                    ])
                    .stdout(Stdio::inherit())
                    .stderr(Stdio::inherit())
                    .spawn()
                    .unwrap();

                let status = command.wait();
            },
            Ok(event) => println!("broken event: {:?}", event),
            Err(e) => println!("watch error:{:?}", e),
        }
    }
}
