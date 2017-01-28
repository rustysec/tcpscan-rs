# tcpscan-rs
Need a tcp scanner template/proof-of-concept in Rust? Here you go!

## Usage
To fire off a scan, just provide comma separated lists of IPs and ports. This will become more robust over time...
bear with me!
```rust
./tcpscnanner 445,135,22 127.0.0.1,192.168.0.1,192.168.0.2
```

## Under The Hood
This is a single threaded TCP scanner using [futures-rs](https://github.com/alexcrichton/futures-rs), [metal io](https://github.com/carllerche/mio), and [tokio](https://github.com/tokio-rs/tokio) to provide socket asynchronicity. The goal is to acheive similar functionality to a scanner written in C++ with Boost's ASIO. Obviously, there is a lot of work to be done here, and hopefully it will simply grow into a helper library (Crate) for easy inclusion into other projects.
