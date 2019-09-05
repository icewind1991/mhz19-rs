# MH-Z19-rs

Accessing MH-Z19 CO₂ sensor over serial bus using rust

## Usage

```rust
use mhz19::MHZ19;

fn main() {
    let mut mhz19 = MHZ19::open("/dev/ttyUSB0").unwrap();
    println!("CO₂ readout: {} ppm", mhz19.read().unwrap());
}
```