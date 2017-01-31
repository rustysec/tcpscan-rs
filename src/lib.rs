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
use tokio_core::reactor::{Core, Handle};
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
    skips: Vec<String>,
    ports: Vec<String>,
    timer: Timer,
    pub results: Vec<HostResult>
}

impl Scanner {
    pub fn new_scanner() -> Scanner {
        Scanner {
            hosts: Vec::new(),
            skips: Vec::new(),
            ports: Vec::new(),
            timer: Timer::default(),
            results: Vec::new() }
    }

    pub fn add_host(&mut self, address: String, handle: Handle) {
        let mut host = Host { ports: Vec::new(), address: address.to_string() };
        for port in self.ports.clone() {
            let addr = address.to_string() + &":".to_string() + &port;
            let addr = addr.parse().unwrap();
            let sock = TcpStream::connect(&addr, &handle);
            let port_object = Port { port: port, timer: self.timer.sleep(Duration::from_millis(5000)), socket: Box::new(sock), result: "Closed".to_string() };
            host.ports.push(port_object);
        }
        self.hosts.push(host);
    }

    pub fn set_ports(&mut self, ports: Vec<&str>) {
        for port in ports {
            self.ports.push(port.to_string());
        }
    }
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
            for host in self.hosts.as_slice() {
                let mut host_result = HostResult { address: host.address.clone(), ports: HashMap::new() };
                for port in host.ports.as_slice() {
                    host_result.ports.insert(port.port.clone(), port.result.clone());
                }
                self.results.push(host_result);
            }
            Ok(Async::Ready(true))
        }
        else {
            Ok(Async::NotReady)
        }
    }
}
