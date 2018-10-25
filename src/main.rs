//extern crate lettre;
extern crate notify;
extern crate reqwest;

#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;

use notify::{Watcher, RawEvent, PollWatcher};
use std::sync::mpsc::channel;
use std::time::Duration;
use std::fs::File;
use std::io::prelude::*;
use notify::RecursiveMode;


#[derive(DeSerialize)]
struct IButtonTranslatorResponse {
    username: String,
}


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

fn runScan() {
    let mut f = File::open("/sys/devices/w1_bus_master1/w1_master_slaves").unwrap();
    let mut contents = String::new();
    f.read_to_string(&mut contents).unwrap();
    if contents.trim() != "not found."{    
	getUsername(contents);
    }
}

fn getUsername(username: String) -> reqwest::Result<()> {
    println!("Sending get for ibutton {}", username);
    let mut res = reqwest::get(&format!("http://ibutton-translator-ibutton-translator.a.csh.rit.edu/?ibutton={}", username))?;
    let mut body = String::new();
    res.read_to_string(&mut body).expect("should read response string");
    let deseralized: IButtonTranslatorResponse = serde_json::from_str(&body).unwrap();
    println!("Username: {}", body.username);

    Ok(())
}

fn main() {
    if let Err(e) = watch() {
        println!("error: {:?}", e)
    }
}
