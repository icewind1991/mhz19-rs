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

## License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or https://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or https://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.
