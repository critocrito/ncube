# Ncube

> Enter the hypercube.

<div align="center">

![Ncube Screenshot](https://raw.githubusercontent.com/critocrito/ncube/master/resources/screenshots/ncube.png)

[Installation](#installation) • [Documentation](#documentation) • [Development](#development)

![GitHub Workflow Status](https://img.shields.io/github/workflow/status/critocrito/ncube/Build%20Status?style=flat-square)
![GitHub release (latest by date)](https://img.shields.io/github/v/release/critocrito/ncube?color=orange&style=flat-square)
![GitHub](https://img.shields.io/github/license/critocrito/ncube?color=blue&style=flat-square)

</div>

---

_This project is in an early stage and probably not working right now._

## Installation

### Linux

Currently there is only [a `deb` package for Ubuntu 18.04](https://github.com/critocrito/ncube/releases/latest). For any other Linux distribution build Ncube [from source](#from-source).

Install the provided `deb` package by either double-clicking it in the file browser or using a terminal:

```sh
sudo dpkg -i ncube_<version>_amd64.deb
```

### macOS

Ncube can be installed using the [DMG installer image](https://github.com/critocrito/ncube/releases/latest). You can install Ncube by double-clicking the `Ncube_<version>.dmg` file and dragging the application into your `Applications` folder. Newer versions of macOS might refuse to install the package since Ncube is not verified by Apple. To install Ncube follow the following steps:

1. Try to run Ncube.
2. On your Mac, choose Apple menu > System Preferences, click Security & Privacy, then click General.
3. Click the lock icon to unlock it, then enter an administrator name and password.
4. Click on Open Anyway to allow Ncube on your computer.

![macOS Security & Privacy preferences](https://raw.githubusercontent.com/critocrito/ncube/master/resources/screenshots/mac-preferences.png)

### From Source

The following prerequesites are required to build Ncube from source:

- A recent version of [Rust](https://www.rust-lang.org/).
  [Rustup](https://rustup.rs/) is a great way to do so. The minimum supported
  version is 1.40.0+.
- The UI is developed in ClojureScript and therefore requires Java and
  [Clojure](https://clojure.org/guides/getting_started). Ncube is tested using
  Clojure 1.10.1 and ClojureScript 1.10.597. Not that ClojureScript 1.10.753
  does not work right now.
- The CSS stylesheets are compiled using [PostCSS](https://postcss.org/) which requires NodeJS and  
  `npm`/`yarn`. Run `yarn install` (or `npm install`) to fetch all dependencies.

```sh
yarn install
```

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

Once all the dependencies are in place build Ncube:

```sh
make
```

The build produces a single binary and can be started like this:

```sh
./target/release/ncube
```

## Documentation

All documentation can be [found in the `doc`](doc) directory.

The HTTP endpoints of `ncubed` are described in the [HTTP API
documentation](doc/http-api.md).

They are supported with a series of [diagrams](doc/diagrams/ncube). To
re-generate the architecture diagrams install
[`fc4`](https://fundingcircle.github.io/fc4-framework/docs/get-started) and
regenerate the images:

```sh
fc4 -fsr doc/diagrams
```

The account authorization for remote workspaces is described in [a dedicated
document](doc/auth-workflow.pdf).

## Development

Ncube consists of several parts:

- `ncubed` is the backend of Ncube that exposes all functionality of Ncube.
- `ncubectl` is a CLI tool to manage server installations of Ncube.
- The frontend UI is a single page web app that communicates to `ncubed`.
- `ncube` is the full desktop app including `ncubed` and wrapping the frontend UI in a local browser window.

See the [installation from source](#from-source) section to setup all the build
dependencies.

Since the backend delivers the frontend using it's own HTTP server, the frontend
assets must be available when compiling the backend. there is an [open
issue](https://github.com/critocrito/ncube/issues/39) to resolve this but until
then before compiling the backend you need to run the following once, and
everytime you clean the `target` directory.:

```sh
make ui
make workspace
```

Open a terminal in the project root and start the backend:

```sh
cargo run --bin ncubed
```

In a different terminal start the webpack development server to build the UI:

```sh
yarn start
```

This will open a browser at `http://localhost:8080` that provides the UI.
Further this will start [React
Cosmos](https://github.com/react-cosmos/react-cosmos) on
`http://localhost:5000`.

### Tests

The tests can be run by executing the following command:

```sh
make test
```

This project provides [`devcards`](https://github.com/bhauman/devcards/) to
display its design system. They can be inspected at
`http://localhost:9500/figwheel-extra-main/devcards`.

A standalone version of `devcards` is build when compiling the production
distribution (`make/make build`). The output directory is `resources/dist`.

## License

All code is copyrighted by _christo@cryptodrunks.net_ and licensed under the [GPL3](https://www.gnu.org/licenses/gpl-3.0.html).
