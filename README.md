# tcpscan-rs
Need a tcp scanner template/proof-of-concept in Rust? Here you go!

## Usage
To use tcpscan-rs in your project, add a dependency to your Cargo.toml
```toml
tcpscan-rs = { git = "https://github.com/rustysec/tcpscan-rs.git" }
```
and
```rust
extern crate tcpscan_rs
use tcpscan_rs::Scanner
```
to the top of your source file

## Example
This project is now a Crate with a simple example included. You can build the the example by simple
```shell
cargo build --example simple
```
and invoking the executable as
```shell
./tcpscnanner 445,135,22 127.0.0.1,192.168.0.1,192.168.0.2
```
or simply
```shell
cargo run --example simple 445,135,22 127.0.0.1,192.168.0.1,192.168.0.2
```

## Under The Hood
This is a single threaded TCP scanner using [futures-rs](https://github.com/alexcrichton/futures-rs), [metal io](https://github.com/carllerche/mio), and [tokio](https://github.com/tokio-rs/tokio) to provide socket asynchronicity. The goal is to acheive similar functionality to a scanner written in C++ with Boost's ASIO. Obviously, there is a lot of work to be done here, and hopefully it will simply grow into a helper library (Crate) for easy inclusion into other projects.

## Platform Support
I have tested this under linux x86 and x64 as well as 64bit Windows using `x86_64-pc-windows-gnu` with mingw.
