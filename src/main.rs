//extern crate lettre;
extern crate notify;
extern crate reqwest;

#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;

extern crate lettre_email;
extern crate lettre;

extern crate mime;

use notify::{Watcher, RawEvent, PollWatcher};
use std::sync::mpsc::channel;
use std::time::{Duration, SystemTime};
use std::fs::File;
use std::io::prelude::*;
use notify::RecursiveMode;
use lettre_email::{Mailbox, EmailBuilder, Email};
use lettre::sendmail::SendmailTransport;
use lettre::SendableEmail;
use std::thread::sleep;
use std::process::Command;
use std::path::Path;
use lettre::sendmail::error::SendmailResult;
use lettre::EmailTransport;

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
    let mut res = reqwest::get(&format!("localhost:5000/?ibutton={}", username))?;
    let mut body = String::new();
    res.read_to_string(&mut body).expect("should read response string");
    let deserialized: IButtonTranslatorResponse = serde_json::from_str(&body).unwrap();
    println!("Username: {}", deserialized.username);
    Command::new("scanimage").arg("--resolution").arg("300").arg("-x").arg("215")
        .arg("-y").arg("279").arg(">").arg("/scans/scan.jpg");//scanimage --resolution 300 -x 215 -y 279 > /scans/TMP/
    sleep(Duration::new(45, 0));
    send_email(deserialized.username);
    Ok(())
}

fn send_email(username: String) {
    let mut emailbuilder = EmailBuilder::new();
    emailbuilder.to(Mailbox::new(format!("{}@csh.rit.edu", username)));
    emailbuilder.from(Mailbox::new(format!("eggScan@csh.rit.edu")));
    emailbuilder.body("Your scanned file is attached!");
    emailbuilder.sender(Mailbox::new(format!("eggScan@csh.rit.edu")));
    emailbuilder.subject(format!("Scan from {:?}", SystemTime::now()));
    let mime = "image/jpeg".parse::<mime::Mime>().unwrap();
    emailbuilder.attachment(&Path::new("/scans/scan.jpg"), None, &mime);
    let mut email = &emailbuilder.build().expect("Should be a valid email");
    let mut sendableemail: SendableEmail<Email> = email.into();
    let mut mailer = &SendmailTransport::new();
    mailer.send(&sendableemail);
}

fn main() {
    if let Err(e) = watch() {
        println!("error: {:?}", e)
    }
}
