# tcpscan-rs
Need a tcp scanner template/proof-of-concept in Rust? Here you go!

## Usage
To fire off a scan, just provide comma separated lists of IPs and ports. This will become more robust over time... 
bear with me!
```rust
./tcpscnanner 127.0.0.1,192.168.0.1,192.168.0.2 445,135,22
```

## Under The Hood
This is a single threaded TCP scanner using `mio` and `futures-rs` to provide socket asynchronicity. The goal is to acheive
similar functionality to a scanner written with Boost's ASIO. Obviously, there is a lot of work to be done here, and hopefully 
it will simply grow into a helper library (Crate) for easy inclusion into other projects.
