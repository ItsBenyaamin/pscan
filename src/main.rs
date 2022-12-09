use std::net::{SocketAddr, TcpStream};
use std::str::FromStr;
use std::time::Duration;
use clap::Parser;

#[derive(Parser)]
struct Cli {
    /// address
    #[arg(short, long)]
    addr: String,
    /// pscan -a address -s 443
    #[arg(short, long)]
    single: Option<u16>,
    /// pscan -a address -r 80-443
    #[arg(short, long)]
    range: Option<String>,
    /// pscan -a address -l 80,443,1080,445
    #[arg(short, long)]
    list: Option<String>
}

fn main() {
    let args = Cli::parse();
    
    if let Some(single) = args.single {
        scan_ports(&args.addr, vec![single]);
    }
    
    if let Some(range) = args.range {
        let mut split = range.split("-");
        let start: u16 = match split.next() {
            Some(v) => v.parse().unwrap(),
            None => panic!("range value is not valid, see pscan --help for more info")
        };
        let end: u16 = match split.next() {
            Some(v) => v.parse().unwrap(),
            None => panic!("range value is not valid, see pscan --help for more info")
        };
        let mut list = vec![];
        (start..=end).into_iter().for_each(|x| list.push(x));
        scan_ports(&args.addr, list);
    }
    
    if let Some(ports) = args.list {
        let mut list = vec![];
        let mut split = ports.split(",");
        loop {
            match split.next() {
                Some(v) => {
                    list.push(v.parse().unwrap());
                }
                None => break
            }
        }
        scan_ports(&args.addr, list);
    }
}

fn scan_ports(addr: &String, list: Vec<u16>) {
    let mut handles = vec![];
    for port in list {
        let host = format!("{}:{}", addr, port);
        let handle = std::thread::spawn(move || {
            let socket_addr = SocketAddr::from_str(host.as_str()).unwrap();
            match TcpStream::connect_timeout(&socket_addr, Duration::from_secs(3)) {
                Ok(_) => println!("port {} is OPEN.", port),
                Err(_) => println!("port {} is NOT OPEN", port)
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}