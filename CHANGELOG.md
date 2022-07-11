# Changelog

[`sh1106`](https://crates.io/crates/sh1106) is a Rust driver for the SH1106 OLED display. It
supports [embedded-graphics](https://crates.io/crates/embedded-graphics) or raw pixel drawing modes
and works with the [embedded-hal](crates.io/crates/embedded-hal) traits for maximum portability.

<!-- next-header -->

## [Unreleased] - ReleaseDate

### Changed

- **(breaking)** [#28](https://github.com/jamwaffles/sh1106/pull/28) Upgrade MSRV to 1.50.0, add a
  faster implementation of `DrawTarget::fill_solid`.

## [0.4.0] - 2021-07-11

### Changed

- **(breaking)** [#25](https://github.com/jamwaffles/sh1106/pull/25) Upgrade to `embedded-graphics`
  0.7.

## [0.3.4] - 2020-12-28

### Fixed

- [#23](https://github.com/jamwaffles/sh1106/pull/23) Fixed command bytes for `PreChargePeriod` and
  `VcomhDeselect`.

## [0.3.3] - 2020-06-09

### Added

- [#22](https://github.com/jamwaffles/sh1106/pull/22) Add `DisplaySize::Display128x64NoOffset`
  variant for 128x64 displays that don't use a 132x64 buffer internally.

## [0.3.2] - 2020-04-30

### Added

- [#20](https://github.com/jamwaffles/sh1106/pull/20) Add `set_contrast` method to set the display
  contrast/brightness.

## [0.3.1] - 2020-03-21

### Fixed

- Fix docs.rs build config

## [0.3.0] - 2020-03-20

### Added

- Migrate from Travis to CircleCI

### Changed

- **(breaking)** [#18](https://github.com/jamwaffles/sh1106/pull/18) Upgrade to embedded-graphics
  0.6.0

## [0.3.0-alpha.4]

### Fixed

- Pin `embedded-graphics` dependency versio to `0.6.0-alpha.2`

## 0.3.0-alpha.3

### Added

- Added the `NoOutputPin` dummy pin type for SPI cases when no Chip Select pin is required. Use it
  like this:

```rust
let spi = Spi::spi1(
    // ...
);

let mut disp: GraphicsMode<_> = sh1106::Builder::new()
    .connect_spi(spi, dc, sh1106::NoOutputPin::new())
    .into();
```

## 0.3.0-alpha.2

Upgrade to new embedded-graphics `0.6.0-alpha.2` release. Please see the
[embedded-graphics changelog](https://github.com/jamwaffles/embedded-graphics/blob/c0ed1700635f307a4c5114fec1769147878fd584/CHANGELOG.md)
for more information.

### Changed

- **(breaking)** #11 Upgraded to [embedded-graphics](https://crates.io/crates/embedded-graphics)
  0.6.0-alpha.2

## 0.3.0-alpha.1

Upgrade to new embedded-graphics `0.6.0-alpha.1` release. Please see the
[embedded-graphics changelog](https://github.com/jamwaffles/embedded-graphics/blob/embedded-graphics-v0.6.0-alpha.1/CHANGELOG.md)
for more information.

### Changed

- **(breaking)** #9 Upgraded to [embedded-graphics](https://crates.io/crates/embedded-graphics)
  0.6.0-alpha.1

<!-- next-url -->

[unreleased]: https://github.com/jamwaffles/sh1106/compare/v0.4.0...HEAD
[0.4.0]: https://github.com/jamwaffles/sh1106/compare/v0.3.4...v0.4.0
[0.3.4]: https://github.com/jamwaffles/sh1106/compare/v0.3.3...v0.3.4
[0.3.3]: https://github.com/jamwaffles/sh1106/compare/v0.3.2...v0.3.3
[0.3.2]: https://github.com/jamwaffles/sh1106/compare/v0.3.1...v0.3.2
[0.3.1]: https://github.com/jamwaffles/sh1106/compare/v0.3.0...v0.3.1
[0.3.0]: https://github.com/jamwaffles/sh1106/compare/v0.3.0-alpha.4...v0.3.0
[0.3.0-alpha.4]: https://github.com/jamwaffles/sh1106/compare/v0.3.0-alpha.3...v0.3.0-alpha.4
[0.3.0-alpha.3]: https://github.com/jamwaffles/sh1106/compare/v0.3.0-alpha.2...v0.3.0-alpha.3
