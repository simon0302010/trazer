use std::{io::Error, net::IpAddr, sync::mpsc, thread, time::{Duration, Instant}};

use sync_resolve;
use traceroute;

fn main() {
    let ip = sync_resolve::resolve_host("tu.berlin")
        .and_then(|r| {
            r.collect::<Vec<IpAddr>>()
                .first()
                .ok_or(Error::new(
                    std::io::ErrorKind::NotFound,
                    "Couldn't find address",
                ))
                .map(|a| *a)
        })
        .expect("Failed to resolve host");

    println!("target ip: {}", ip);

    let res = traceroute::execute(format!("{}:0", ip));
    if let Err(e) = res {
        eprintln!("error while tracing: {}", e);
        return;
    }

    let (trace_tx, trace_rx) = mpsc::channel();

    let _ = thread::spawn(move || {
        for hop in res.unwrap() {
            let _ = trace_tx.send(hop);
        }
    });

    loop {
        match trace_rx.recv_timeout(Duration::from_secs(5)) {
            Ok(item) => println!("{:?}", item),
            Err(_) => break
        }
    }
}