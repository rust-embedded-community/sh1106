# SH1106 driver

[![Build Status](https://travis-ci.org/jamwaffles/sh1106.svg?branch=master)](https://travis-ci.org/jamwaffles/sh1106)

[![SH116 display module showing the Rust logo](readme_banner.jpg?raw=true)](examples/image.rs)

I2C driver for the SH1106 OLED display written in 100% Rust

## [Documentation](https://docs.rs/sh1106)

From [`examples/text.rs`](examples/text.rs):

```rust
// ...snip, see examples/text.rs for runnable code ...

let i2c = /* ... */;

let mut disp: GraphicsMode<_> = Builder::new().connect_i2c(i2c).into();

disp.init().unwrap();
disp.flush().unwrap();

disp.draw(
    Font6x8::render_str("Hello world!")
        .stroke_width(1)
        .into_iter(),
);
disp.draw(
    Font6x8::render_str("Hello Rust!")
        .stroke_width(1)
        .translate(Point::new(0, 16))
        .into_iter(),
);

disp.flush().unwrap();
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the
work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
