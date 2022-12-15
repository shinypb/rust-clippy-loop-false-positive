use anyhow::{anyhow, Result};
use mdns_sd::{ServiceDaemon, ServiceEvent};

fn demo() -> Result<(String, String)> {
    let mdns = ServiceDaemon::new()?;
    let service_type = "_http._tcp.local.";
    let receiver = mdns.browse(service_type)?;
    while let Ok(event) = receiver.recv() {
        let ServiceEvent::ServiceResolved(info) = event else {
            // We don't care about other events here
            continue;
        };
        let Some(addr) = info.get_addresses().iter().next() else {
            // No one should ever have zero addresses, but just in case...
            continue;
        };
        let port = info.get_port();
        return Ok((
            info.get_fullname().to_string(),
            format!("http://{addr}:{port}"),
        ));
    }

    Err(anyhow!(format!(
        "Failed to discover {service_type} node on the local network"
    )))
}

fn main() -> Result<()> {
    let (name, url) = demo()?;
    println!("Found {name} at {url}");
    Ok(())
}
