use std::{error::Error, fmt::Display, net::IpAddr};

use trust_dns_resolver::Resolver;

#[derive(Debug, Clone)]
struct InvalidConfigError;

#[derive(Debug, Clone)]
struct DnsError {
    host: String,
}

impl Error for DnsError {}

impl Display for DnsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "couldn't resolve host \"{}\"", self.host)
    }
}

pub fn resolve(host: &String) -> Result<IpAddr, Box<dyn Error>> {
    let response = Resolver::from_system_conf()?.lookup_ip(host)?;
    match response.iter().next() {
        Some(ip) => Ok(ip),
        None => Err(DnsError { host: host.clone() }.into()),
    }
}
