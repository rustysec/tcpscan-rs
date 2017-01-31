extern crate tokio_core;
extern crate futures;
extern crate mio;
extern crate tokio_timer;
extern crate serde_json;
extern crate tcpscan_rs;

use std::env;
use std::vec::Vec;
use tokio_core::reactor::{Core};
use tcpscan_rs::Scanner;

fn usage() {
    println!("Simple Async Tcp Scanner");
    println!("Usage:");
    println!("\t{} PORT1,PORT2,PORT3 IP1,IP2,IP3", env::args().nth(0).unwrap());
    println!("\nThis will scan all three ports across all three of the IPs");
    ()
}

fn main() {
    let mut core = Core::new().unwrap();
    let mut scanner = Scanner::new_scanner();

    if env::args().count() != 3 {
        return usage();
    }

    let addresses = env::args().nth(2).unwrap();
    let ports = env::args().nth(1).unwrap();
    let address_list = addresses.split(",").collect::<Vec<&str>>();
    let port_list = ports.split(",").collect::<Vec<&str>>();

    scanner.set_ports(port_list);

    for address in address_list {
        scanner.add_host(address.to_string(), core.handle());
    }

    match core.run(&mut scanner) {
        Ok(true) => {
            let encoded = serde_json::to_string_pretty(&scanner.results).unwrap();
            println!("{}", encoded);

        },
        Ok(false) => {
        },
        Err(_) => {
        }
    }
}
