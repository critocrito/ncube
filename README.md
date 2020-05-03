# Ncube

> Enter the hypercube.

[![License: GPL v3](https://img.shields.io/badge/License-GPL%20v3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0) ![Build Status](https://github.com/critocrito/ncube/workflows/Build%20Status/badge.svg)

<p align="center">
  <img src="/resources/logo.png" width="400" alt="Ncube - Data exploration and verification for human rights">
</p>

_This project is in an early stage and probably not working right now._

## Installation

No packages are published yet, but macOS DMG images can be built manually. To do
so install all required toolchains as described in the development section and
run the following command:

```sh
make clean pkg-dmg
```

To build a release version of Ncube run the following:

```sh
make clean target/release/ncube
```

## Development

Ncube consists of several parts:

- `ncubed` is the backend of Ncube that exposes all functionality of Ncube.
- The frontend UI is a single page web app that communicates to `ncubed`.
- `ncube` is the full desktop app including `ncubed` and wrapping the frontend UI in a local browser window.

The following toolchains have to be available to build Ncube:

- Make sure to have a recent version of [Rust](https://www.rust-lang.org/)
  installed. [Rustup](https://rustup.rs/) is a great way to do so.
- The UI is developed in Clojurescript and therefore requires Java and
  [Clojure](https://clojure.org/guides/getting_started).
- The CSS stylesheets are compiled using [PostCSS](https://postcss.org/) which requires NodeJS and  
  `npm`/`yarn`. Run `yarn install` (or `npm install`) to fetch all dependencies.

```sh
$ cargo --version
rustc 1.42.0 (b8cedc004 2020-03-09)

$ java -version
openjdk version "13.0.2" 2020-01-14

$ clojure -e '(clojure-version)'
"1.10.1"

$ node --version
v12.16.1

$ yarn --version
1.22.0
```

Begin by compiling the stylesheets. I usually leave this command running in a
terminal. It will watch the stylesheets for any changes and recompile if needed.

```sh
node_modules/.bin/postcss -w -o target/public/cljs-out/styles.css src/css/*.css
```

The UI development environment is based on [Figwheel](https://figwheel.org/).
The [`dev.cljs.edn`](./dev.cljs.edn) configuration starts a development REPL for
the UI development. When using Emacs with Cider permit the
[`.dir-locals.el`](./.dir-locals.el) to configure the REPL.

Alternatively start a development REPL manually in another terminal window:

```sh
clj -A:fig-deps:dev-deps:dev
```

This opens the browser at port 9500 on `localhost`.

Since the backend delivers the frontend using it's own HTTP server, the frontend
assets must be available when compiling the backend. This is the case even if
you use Figwheel to load the UI on port 9500 and have it communicate with the
backend on port 40666 since they produce different assets in a different
locations. Open yet another terminal and run the following:

```sh
make ui
cargo run --bin ncubed
```

### Tests

This project provides [`devcards`](https://github.com/bhauman/devcards/) to
display its design system. They can be inspected at
`http://localhost:9500/figwheel-extra-main/devcards`.

A standalone version of `devcards` is build when compiling the production
distribution (`make/make build`). The output directory is `resources/dist`.

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

```sh
fc4 -fsr doc/diagrams
```

## License

All code is copyrighted by _christo@cryptodrunks.net_ and licensed under the [GPL3](https://www.gnu.org/licenses/gpl-3.0.html).
