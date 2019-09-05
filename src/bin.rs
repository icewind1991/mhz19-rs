use mhz19::MHZ19;
use std::time::Duration;
use std::thread::sleep;

fn main() {
    let mut mhz19 = MHZ19::open("/dev/ttyUSB0").unwrap();
    let delay = Duration::from_secs(1);
    loop {
        let value = mhz19.read().unwrap();
        println!("{}", value);
        sleep(delay);
    }
}