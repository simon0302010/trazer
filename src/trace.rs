use std::thread;
use std::{io::Error, net::IpAddr, time::Duration};
use std::sync::mpsc;

use traceroute::TraceHop;

pub fn trace_route(host: &str, timeout: Duration) -> Result<Vec<TraceHop>, String> {
    let ip = sync_resolve::resolve_host(host)
        .and_then(|r| {
            r.collect::<Vec<IpAddr>>()
                .first()
                .ok_or(Error::new(
                    std::io::ErrorKind::NotFound,
                    format!("Couldn't resolve {}", host),
                ))
                .map(|a| *a)
        });

    if let Err(e) = ip { return Err(e.to_string()) }
    let ip = ip.unwrap();

    println!("tracing route to {}", ip);

    let res = traceroute::execute(format!("{}:0", ip));
    if let Err(e) = res {
        eprintln!("error while tracing: {}", e);
        return Err(format!("Failed to trace route of {}: {}", host, e));
    }

    let (trace_tx, trace_rx) = mpsc::channel();

    let _ = thread::spawn(move || {
        for hop in res.unwrap() {
            let _ = trace_tx.send(hop);
        }
    });

    let mut hops = Vec::new();
    loop {
        match trace_rx.recv_timeout(timeout) {
            Ok(item) => if let Ok(hop) = item {
                hops.push(hop);
            },
            Err(_) => break
        }
    }

    return Ok(hops);
}