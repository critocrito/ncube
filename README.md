# Ncube

> Enter the hypercube.

<p align="center">
  <img src="/logo.png" alt="Ncube - Data exploration and verification for human rights">
</p>

*This project is in an early stage and probably not working right now.*

## Installation

No packages are available at this point. Ncube needs to be build from source.
Make sure to have a recent version of [Rust](https://www.rust-lang.org/)
installed. [Rustup](https://rustup.rs/) is a great way to do so.

``` sh
$ rustc --version
rustc 1.42.0 (b8cedc004 2020-03-09)
```

To start the Ncube daemon run in a terminal:

``` sh
cargo run --bin ncubed
```

In another terminal start the UI:

``` sh
cargo run --bin ncube
```

## Documentation

All documentation can be [found in the `doc`](doc) directory.

The HTTP endpoints of `ncubed` are described in the [HTTP API
documentation](doc/http-api.md).

The architecture choices for Ncube are described as a series of [architecture
decision
records](https://www.thoughtworks.com/de/radar/techniques/lightweight-architecture-decision-records).
They are supported with a series of [diagrams](doc/diagrams/ncube). To
re-generate the architecture diagrams install
[`fc4`](https://fundingcircle.github.io/fc4-framework/docs/get-started) and
regenerate the images:

``` sh
fc4 -fsr doc/diagrams
```

## Building App

At the current stage only MacOS DMG images can be build. It depends on
[`create-dmg`](https://github.com/andreyvit/create-dmg).

``` sh
rm -rf target && ./scripts/build_macos.sh
```

## License

All code is copyrighted by *christo@cryptodrunks.net* and licensed under the [GPL3](https://www.gnu.org/licenses/gpl-3.0.html). 
