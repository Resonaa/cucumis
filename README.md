<div align="center">

# cucumis

[hack.chat](https://hack.chat) tunnel that resolves DNS pollution

[![GitHub Actions](https://img.shields.io/github/actions/workflow/status/jwcub/cucumis/build.yml?style=flat-square)](https://github.com/jwcub/cucumis/actions)
[![Crates.io](https://img.shields.io/crates/v/cucumis?style=flat-square)](https://crates.io/crates/cucumis)
[![Downloads](https://img.shields.io/crates/d/cucumis?style=flat-square)](https://crates.io/crates/cucumis)
[![License](https://img.shields.io/github/license/jwcub/cucumis?style=flat-square)](https://github.com/jwcub/cucumis/blob/main/LICENSE)
[![GitHub repo size](https://img.shields.io/github/repo-size/jwcub/cucumis?style=flat-square)](https://github.com/jwcub/cucumis)
[![GitHub Repo stars](https://img.shields.io/github/stars/jwcub/cucumis?style=flat-square&color=yellow)](https://github.com/jwcub/cucumis/stargazers)
[![GitHub commit activity](https://img.shields.io/github/commit-activity/y/jwcub/cucumis?style=flat-square)](https://github.com/jwcub/cucumis/commits/main/)
[![GitHub contributors](https://img.shields.io/github/contributors/jwcub/cucumis?style=flat-square)](https://github.com/jwcub/cucumis/graphs/contributors)

</div>

## Overview

`cucumis` is a [hack.chat](https://hack.chat) tunnel that resolves DNS pollution for users in China. It can be used as a proxy server for your bots and work deadly fine with [hackchat++ client](https://github.com/Hiyoteam/hackchat-client-plus).

## Usage

```sh
$ cucumis
```

Just run with no options and a WebSocket tunnel will be launched on `127.0.0.1:9091` by default.

If you prefer some configuration:

```sh
$ cucumis --help
hack.chat tunnel that resolves DNS pollution

Usage: cucumis.exe [OPTIONS]

Options:
      --host <HOST>      Tunnel host [default: 127.0.0.1]
      --port <PORT>      Tunnel port [default: 9091]
      --ip <IP>          hack.chat real IP [default: 104.131.138.176]
      --domain <DOMAIN>  Dummy domain to bypass SNI [default: bake.lyka.pro]
      --ws <WS>          hack.chat WS url [default: wss://hack.chat/chat-ws]
  -v, --verbose...       Increase logging verbosity
  -q, --quiet...         Decrease logging verbosity
  -h, --help             Print help
  -V, --version          Print version
```

## Installation

### Installing from Crates.io (Recommended)

```sh
$ cargo install cucumis
$ cucumis --help
```

### Building from Source

```sh
$ git clone https://github.com/jwcub/cucumis.git
$ cd cucumis
$ cargo run --release -- --help
```

## Development

Before pushing your commits, be sure to run through all the checks:

```sh
$ cargo clippy
$ cargo fmt
$ cargo build
```

## License

This project is licensed under the [Unlicense](https://github.com/jwcub/cucumis/blob/main/LICENSE).
