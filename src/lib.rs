use std::net::{Shutdown,SocketAddr,TcpStream,IpAddr,Ipv4Addr};

pub struct Config {
    pub addr: std::net::Ipv4Addr,
    pub start_port: u16,
    pub end_port: u16,
}

pub struct ScanResult {
    pub socket_addr: SocketAddr,
    pub result: bool
}

impl Config {
    pub fn new(addr_str: &str, ports_str: &str) -> Result<Config, &'static str> {
        let addr: Ipv4Addr = addr_str.parse().unwrap();
        let ports = parse_ports(ports_str).unwrap();
        Ok(
            Config {
                addr: addr,
                start_port: ports.0,
                end_port: ports.1
            }
        )
    }
}

pub fn run(config: Config){
    scan_range(config.addr, config.start_port, config.end_port);
    /*
    for s in scan {
        println!("{}: {}", s.socket_addr, s.result);
    }
    */
}

pub fn parse_ports(ports_str: &str) -> Result<(u16, u16), &'static str> {
    let mut v: Vec<&str> = ports_str.split("-").collect();
    let err_msg = "Could not parse ports. Must be in the format n-n where n is\
a u16.";
    v.sort();
    if v.len() > 2{
        return Err(err_msg)
    }
    if v.len() == 1 {
        let port: u16 = v[0].trim().parse().expect(err_msg);
        return Ok((port,port))
    }
    let start: u16 = v[0].trim().parse().expect(err_msg);
    let end: u16 = v[1].trim().parse().expect(err_msg);
    Ok((start,end))
}

pub fn scan_range(addr: std::net::Ipv4Addr, start: u16, end: u16) -> Vec <ScanResult> {
    let mut results = Vec::new();
    for port in start..(end+1){
        let addr = SocketAddr::new(IpAddr::V4(addr), port);
        let this_result = ScanResult {
            socket_addr: addr,
            result: connect(addr)
        };
        println!("{}: {}", this_result.socket_addr, this_result.result);
        results.push(this_result);
    }
    results
}

pub fn connect(addr: SocketAddr) -> bool {
    if let Ok(stream) = TcpStream::connect(addr) {
        stream.shutdown(Shutdown::Both).expect("Couldn't close connection");
        true
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn good_connection(){
        // Should definitely set up a listener or smth for this...
        let addr = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
        let port: u16 = 22;
        assert_eq!(connect(addr, port), true);
    }

    #[test]
    fn bad_connection(){
        let addr = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
        let port: u16 = 1234;
        assert_eq!(connect(addr, port), false);
    }

}
