//extern crate lettre;
extern crate notify;

use notify::{Watcher, RawEvent, PollWatcher};
use std::sync::mpsc::channel;
use std::time::Duration;
use std::fs::File;
use std::io::prelude::*;
use notify::RecursiveMode;

fn watch() -> notify::Result<()> {
    let (tx, rx) = channel();

    let mut watcher = PollWatcher::with_delay_ms(tx, 500).unwrap();

    println!("Started!");

    watcher.watch("/sys/devices/w1_bus_master1", RecursiveMode::NonRecursive).unwrap();

    //try!(watcher.watch("/root", RecursiveMode::Recursive));

    loop {
        match rx.recv() {
            Ok(event) => {
			     runScan()
			 },
	    Err(e) => println!("watch error: {:?}", e),
        }
    }
}

fn runScan(){
    let mut f = File::open("/sys/devices/w1_bus_master1/w1_master_slaves").unwrap();
    let mut contents = String::new();
    f.read_to_string(&mut contents).unwrap();
    if contents.trim() != "not found."{    
	getUsername(contents);
    }
}

fn getUsername(username: String){
    println!("Sending get for ibutton {}", username);    
//send get request
}

fn main() {
    if let Err(e) = watch() {
        println!("error: {:?}", e)
    }
}
