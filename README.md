[![License](https://img.shields.io/badge/License-BSD_3--Clause-yellow.svg)](https://opensource.org/licenses/BSD-3-Clause)
[![Release](https://github.com/mbrav/nuttertools/actions/workflows/release.yml/badge.svg)](https://github.com/mbrav/nuttertools/actions/workflows/release.yml)
[![wakatime](https://wakatime.com/badge/user/54ad05ce-f39b-4fa3-9f2a-6fe4b1c53ba4/project/65b04d94-061a-4c48-a268-584ff8ab9bbd.svg)](https://wakatime.com/badge/user/54ad05ce-f39b-4fa3-9f2a-6fe4b1c53ba4/project/65b04d94-061a-4c48-a268-584ff8ab9bbd)
[![tokei](https://tokei.rs/b1/github/mbrav/nuttertools?category=lines)](https://tokei.rs/b1/github/mbrav/nuttertools)

<p align="center">
    <a href="https://github.com/mbrav/nuttertools" target="_blank" rel="noopener noreferrer">
        <img width="400" src="logo.png" title="nuttertools">
    </a>
</p>

<h1 align="center">nuttertools</h1>

<p align="center">A collection of crazy CLI tools in Rust.
<br>Inspired by the "nuttertools" cheat code from <i>Grand Theft Auto: ViceCity</i>.
</p>

## Run

First, build release binary

```bash
cargo build --release 
```

The run the binary

```bash
./target/release/nuttertools --help
```

You will get the following output:

```text
A collection of crazy CLI tools in Rust

Usage: nuttertools [OPTIONS] <COMMAND>

Commands:
  phone-gen         Brute force all possible phone numbers
  prosecho          The drunk pro's alternative to echo
  proxy-police      Proxy tool for spoofing red light activity
  rat               A program that will rat on all files you pass to it
  russian-roulette  Famous Russian gun game that blows brains
  help              Print this message or the help of the given subcommand(s)

Options:
  -n, --no-time  Specify whether to print elapsed time of the program
  -h, --help     Print help
  -V, --version  Print version
```
