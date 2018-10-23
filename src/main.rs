//extern crate lettre;
extern crate notify;

use notify::{RecommendedWatcher, Watcher};
use std::sync::mpsc::channel;
use std::time::Duration;
use notify::RecursiveMode;

fn watch() -> notify::Result<()> {
    let (tx, rx) = channel();

    let mut watcher: RecommendedWatcher = try!(Watcher::new(tx, Duration::from_secs(1)));

    println!("Started!");

    try!(watcher.watch("/sys/devices/w1_bus_master1/w1_master_slaves", RecursiveMode::Recursive));

    loop {
        match rx.recv() {
            Ok(event) => println!("{:?}", event),
            Err(e) => println!("watch error: {:?}", e),
        }
    }
}

fn main() {
    if let Err(e) = watch() {
        println!("error: {:?}", e)
    }
}
