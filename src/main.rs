extern crate tokio_core;
extern crate futures;
extern crate mio;

use std::env;
use std::vec::Vec;
use futures::{Future, Poll, Async};
use tokio_core::reactor::Core;
use tokio_core::net::{TcpStream};
use tokio_core::io::{IoFuture};

pub struct Host {
    address: String,
    port: i32
}

pub struct Scanner {
    sockets: Vec<IoFuture<TcpStream>>,
    hosts: Vec<Host>,
    skips: Vec<i32>
}

impl Future for Scanner {
    type Item = bool;
    type Error = std::io::Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        let mut complete: bool = true;

        for i in 0..self.sockets.len() {
            if self.skips.contains(&(i as i32)) { continue; }
            match self.sockets.get_mut(i) {
                Some(s) => {
                    match s.poll() {
                        Ok(Async::NotReady) => {
                            complete = false;
                        },
                        Ok(Async::Ready(_)) => {
                            self.skips.push(i as i32);
                            let host = self.hosts.get(i).unwrap();
                            println!("{}:{} opened!", host.address, host.port);
                        },
                        Err(_) => {
                            self.skips.push(i as i32);
                            let host = self.hosts.get(i).unwrap();
                            println!("{}:{} closed.", host.address, host.port);
                        }
                    }
                },
                None => {
                    complete = false;
                }
            }
        }
        if complete {
            Ok(Async::Ready(true))
        }
        else {
            Ok(Async::NotReady)
        }
    }
}

fn usage() {
    println!("Simple Async Tcp Scanner");
    println!("Usage:");
    println!("\t{} IP1,IP2,IP3 PORT1,PORT2,PORT3", env::args().nth(0).unwrap());
    println!("\nThis will scan all three ports across all three of the IPs");
    ()
}

fn main() {
    let mut core = Core::new().unwrap();

    if env::args().count() != 3 {
        return usage();
    }

    let addresses = env::args().nth(1).unwrap();
    let ports = env::args().nth(2).unwrap();
    let address_list = addresses.split(",").collect::<Vec<&str>>();
    let port_list = ports.split(",").collect::<Vec<&str>>();

    let mut scanner = Scanner { sockets: Vec::new(), hosts: Vec::new(), skips: Vec::new() };
    for address in address_list {
        for port in port_list.clone() {
            let addr = address.to_string() + &":".to_string() + port;
            let addr = addr.parse().unwrap();
            scanner.sockets.push(Box::new(TcpStream::connect(&addr, &core.handle())));
            scanner.hosts.push(Host {
                address: address.clone().to_string(),
                port: port.parse().unwrap()
            });
        }
    }

    core.run(&mut scanner).unwrap();
    println!("All done!");
}
