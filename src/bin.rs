use mhz19::MHZ19;
use std::time::Duration;
use std::thread::sleep;
use std::ffi::OsStr;
use std::env;

const DELAY: Duration = Duration::from_secs(1);

fn main() {
    match env::args().skip(1).next() {
        Some(path) => loop {
            listen(&path);
            sleep(DELAY);
        },
        None => eprintln!("Path required")
    }
}

fn listen<T: AsRef<OsStr> + ?Sized>(port: &T) {
    let mut mhz19 = MHZ19::open(port).unwrap();
    loop {
        match { mhz19.read() } {
            Ok(value) => println!("{}", value),
            Err(err) => {
                eprintln!("Error while reading value: {}, reconnecting", err);
                return;
            }
        };
        sleep(DELAY);
    }
}