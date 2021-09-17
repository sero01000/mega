# mega

[![Crates.io](https://img.shields.io/crates/v/mega_api.svg)](https://crates.io/crates/mega_api)
[![Docs.rs](https://docs.rs/mega_api/badge.svg)](https://docs.rs/mega_api)
[![CI](https://github.com/sero01000/mega/workflows/Continuous%20Integration/badge.svg)](https://github.com/sero01000/mega/actions)
[![Coverage Status](https://coveralls.io/repos/github/sero01000/mega/badge.svg?branch=main)](https://coveralls.io/github/sero01000/mega?branch=main)


## Usage


```
use mega_api;

fn main() {
    let email="test@email.com";
    let password="test";
    let account=Mega::login(email,password);
    println!("{:?}", account);
}
```

## Task list
- [x] Authentication api
- [ ] Get all files function
- [ ] Download file function
- [ ] ∞

## Installation

### Cargo

* Install the rust toolchain in order to have cargo installed by following
  [this](https://www.rust-lang.org/tools/install) guide.
* run `cargo install mega`

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

See [CONTRIBUTING.md](CONTRIBUTING.md).
