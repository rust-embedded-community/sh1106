# SH1106 driver

[![Build Status](https://travis-ci.org/jamwaffles/sh1106.svg?branch=master)](https://travis-ci.org/jamwaffles/sh1106)

[![SH116 display module showing the Rust logo](readme_banner.jpg?raw=true)](examples/image.rs)

I2C driver for the SH1106 OLED display written in 100% Rust

## [Documentation](https://docs.rs/sh1106)

From [`examples/text.rs`](examples/text.rs):

```rust
// ...snip, see examples/text.rs for runnable code ...

let i2c = BlockingI2c::i2c1(
    dp.I2C1,
    (scl, sda),
    &mut afio.mapr,
    Mode::Fast {
        frequency: 400_000,
        duty_cycle: DutyCycle::Ratio2to1,
    },
    clocks,
    &mut rcc.apb1,
    1000,
    10,
    1000,
    1000,
);

let mut disp: GraphicsMode<_> = Builder::new().connect_i2c(i2c).into();

disp.init().unwrap();
disp.flush().unwrap();

disp.draw(
    Font6x8::render_str("Hello world!")
        .with_stroke(Some(1u8.into()))
        .into_iter(),
);
disp.draw(
    Font6x8::render_str("Hello Rust!")
        .with_stroke(Some(1u8.into()))
        .translate(Coord::new(0, 16))
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
