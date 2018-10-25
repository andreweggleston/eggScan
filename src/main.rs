//extern crate lettre;
extern crate notify;
extern crate reqwest;

#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;

extern crate lettre_email;
extern crate lettre;

use notify::{Watcher, RawEvent, PollWatcher};
use std::sync::mpsc::channel;
use std::time::{Duration, SystemTime};
use std::fs::File;
use std::io::prelude::*;
use notify::RecursiveMode;
use lettre_email::EmailBuilder;
use lettre::SendmailTransport;
use std::thread::sleep;
use std::process::Command;
use std::path::Path;
use mime;
use lettre::SendmailTransport;
use lettre::SendableEmail;

#[derive(Deserialize)]
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
                run_scan()
            }
            Err(e) => println!("watch error: {:?}", e),
        }
    }
}

fn run_scan() {
    let mut f = File::open("/sys/devices/w1_bus_master1/w1_master_slaves").unwrap();
    let mut contents = String::new();
    f.read_to_string(&mut contents).unwrap();
    if contents.trim() != "not found." {
        get_username(contents);
    }
}

fn get_username(username: String) -> reqwest::Result<()> {
    println!("Sending get for ibutton {}", username);
    let mut res = reqwest::get(&format!("http://ibutton-translator-ibutton-translator.a.csh.rit.edu/?ibutton={}", username))?;
    let mut body = String::new();
    res.read_to_string(&mut body).expect("should read response string");
    let deseralized: IButtonTranslatorResponse = serde_json::from_str(&body).unwrap();
    println!("Username: {}", deseralized.username);
    Command::new("scanimage").arg("--resolution").arg("300").arg("-x").arg("215")
        .arg("-y").arg("279").arg(">").arg("/scans/scan.jpg");//scanimage --resolution 300 -x 215 -y 279 > /scans/TMP/
    sleep(Duration::new(45, 0));
    send_email(deserialized.username);
    Ok(())
}

fn send_email(username: String) {
    let mut emailbuilder = EmailBuilder::new();
    emailbuilder.add_to(&format!("{}@csh.rit.edu", username));
    emailbuilder.add_from(&format!("eggScan@csh.rit.edu"));
    emailbuilder.body(&"Your scanned file is attached!");
    emailbuilder.sender(&format!("eggScan@csh.rit.edu"));
    emailbuilder.subject(&format!("Scan from {}", SystemTime::now));
    let mime = mime::JPEG;
    emailbuilder.attachment(&Path::new("/scans/scan.jpg"), None, mime);
    let mut email = emailbuilder.build().expect("Should be a valid email");
    let mut email: SendableEmail() = email.into();
    let mut mailer: SendmailTransport = SendmailTransport::new;
    mailer.send(email);

}

fn main() {
    if let Err(e) = watch() {
        println!("error: {:?}", e)
    }
}
