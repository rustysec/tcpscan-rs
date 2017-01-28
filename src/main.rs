extern crate tokio_core;
extern crate futures;
extern crate mio;
extern crate tokio_timer;
extern crate serde_json;

use std::env;
use std::time::Duration;
use std::vec::Vec;
use std::collections::HashMap;
use futures::{Future, Poll, Async};
use tokio_core::reactor::Core;
use tokio_core::net::{TcpStream};
use tokio_core::io::{IoFuture};
use tokio_timer::{Timer,Sleep};

include!(concat!(env!("OUT_DIR"), "/serde_types.rs"));

pub struct Port {
    port: String,
    socket: IoFuture<TcpStream>,
    timer: Sleep,
    result: String
}

pub struct Host {
    address: String,
    ports: Vec<Port>
}

pub struct Scanner {
    hosts: Vec<Host>,
    skips: Vec<String>
}

impl Future for Scanner {
    type Item = bool;
    type Error = std::io::Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        let mut complete: bool = true;

        for i in 0..self.hosts.len() {
            match self.hosts.get_mut(i) {
                Some(h) => {
                    for j in 0..h.ports.len() {
                        let k = i.to_string() + &":".to_string() + &j.to_string();
                        if self.skips.contains(&k) { continue; }
                        match h.ports.get_mut(j) {
                            Some(p) => {
                                match p.socket.poll() {
                                    Ok(Async::NotReady) => {
                                        match p.timer.poll() {
                                            Ok(Async::NotReady) => {
                                                complete = false;
                                            },
                                            Ok(Async::Ready(_)) => {
                                                p.result = "Closed".to_string();
                                                self.skips.push(k);
                                            },
                                            Err(_) => {
                                            }
                                        }
                                    },
                                    Ok(Async::Ready(_)) => {
                                        p.result = "Open".to_string();
                                        self.skips.push(k);
                                    },
                                    Err(_) => {
                                        p.result = "Closed".to_string();
                                        self.skips.push(k);
                                    }
                                }
                            },
                            None => {
                            }
                        }
                    }
                },
                None => {
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
    println!("\t{} PORT1,PORT2,PORT3 IP1,IP2,IP3", env::args().nth(0).unwrap());
    println!("\nThis will scan all three ports across all three of the IPs");
    ()
}

fn main() {
    let mut core = Core::new().unwrap();
    let mut scanner = Scanner { hosts: Vec::new(), skips: Vec::new() };

    let timer = Timer::default();

    if env::args().count() != 3 {
        return usage();
    }

    let addresses = env::args().nth(2).unwrap();
    let ports = env::args().nth(1).unwrap();
    let address_list = addresses.split(",").collect::<Vec<&str>>();
    let port_list = ports.split(",").collect::<Vec<&str>>();

    for address in address_list {
        let mut host = Host { ports: Vec::new(), address: address.to_string() };
        for port in port_list.clone() {
            let addr = address.to_string() + &":".to_string() + port;
            let addr = addr.parse().unwrap();
            let sock = TcpStream::connect(&addr, &core.handle());
            let port_object = Port { port: port.to_string(), timer: timer.sleep(Duration::from_millis(5000)), socket: Box::new(sock), result: "Closed".to_string() };
            host.ports.push(port_object);
        }
        scanner.hosts.push(host);
    }

    // let mut thang = timeout.select(sock).map(|(win, _)| win);
    match core.run(&mut scanner) {
        Ok(true) => {
            let mut results = Vec::new();
            for host in scanner.hosts {
                let mut host_result = HostResult { address: host.address, ports: HashMap::new() };
                for port in host.ports {
                    host_result.ports.insert(port.port, port.result);
                }
                results.push(host_result);
            }

            let encoded = serde_json::to_string_pretty(&results).unwrap();
            println!("{}", encoded);

        },
        Ok(false) => {
        },
        Err(_) => {
        }
    }
}
