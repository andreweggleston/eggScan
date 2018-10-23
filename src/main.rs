//extern crate lettre;
extern crate notify;

use notify::{Watcher, RawEvent, PollWatcher};
use std::sync::mpsc::channel;
use std::time::Duration;
use notify::RecursiveMode;

fn watch() -> notify::Result<()> {
    let (tx, rx) = channel();

    let mut watcher = PollWatcher::with_delay_ms(tx, 500).unwrap();

    println!("Started!");

    watcher.watch("/sys/devices/w1_bus_master1", RecursiveMode::Recursive).unwrap();

    //try!(watcher.watch("/root", RecursiveMode::Recursive));

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
