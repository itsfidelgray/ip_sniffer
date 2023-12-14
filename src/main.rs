use std::io::{self, Write};
use std::net::{IpAddr, TcpStream};
use std::str::FromStr;
use std::sync::mpsc::{channel, Sender};
use std::thread;
use std::u16::MAX;
use std::{env, process};

// Define a struct to hold command line arguments
struct Arguments {
    ipaddr: IpAddr,
    threads: u16,
}

impl Arguments {
    // Define a function to parse command line arguments
    fn new(args: &[String]) -> Result<Arguments, &'static str> {
        if args.len() < 2 {
            return Err("not enough arguments");
        } else if args.len() > 4 {
            return Err("too many arguments");
        }

        let f = args[1].clone();
        if let Ok(ipaddr) = IpAddr::from_str(&f) {
            return Ok(Arguments {
                ipaddr,
                threads: 4,
            });
        } else {
            let flag = args[1].clone();
            if flag.contains("-h") || flag.contains("-help") && args.len() == 2 {
                println!(
                    "Usage: -j to select how many threads you want
                \r\n -h or -help to show this help message"
                );
                return Err("help");
            } else if flag.contains("-h") || flag.contains("-help") {
                return Err("too many arguments");
            } else if flag.contains("-j") {
                let ipaddr = match IpAddr::from_str(&args[3]) {
                    Ok(s) => s,
                    Err(_) => return Err("not a valid IPADDR; must be IPv4 or IPv6"),
                };
                let threads = match args[2].parse::<u16>() {
                    Ok(s) => s,
                    Err(_) => return Err("failed to parse thread number"),
                };
                return Ok(Arguments {
                    threads,
                    ipaddr,
                });
            } else {
                return Err("invalid syntax");
            }
        }
    }
}

// Define a function to scan ports
fn scan(tx: Sender<u16>, start: u16, addr: IpAddr, num_threads: u16) {
    let mut port: u16 = start + 1;
    loop {
        match TcpStream::connect((addr, port)) {
            Ok(_) => {
                print!(".");
                io::stdout().flush().unwrap();
                tx.send(port).unwrap();
            }
            Err(_) => {}
        }

        if (MAX - port) <= num_threads {
            break;
        }
        port += num_threads;
    }
}

fn main() {
    // Collect command line arguments
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    // Parse command line arguments
    let arguments = Arguments::new(&args).unwrap_or_else(|err| {
        if err.contains("help") {
            process::exit(0);
        } else {
            eprintln!("{} problem parsing arguments: {}", program, err);
            process::exit(0);
        }
    });

    let num_threads = arguments.threads;
    let addr = arguments.ipaddr;

    // Create a channel for communication between threads
    let (tx, rx) = channel();

    // Spawn threads to scan ports
    for i in 0..num_threads {
        let tx = tx.clone();
        thread::spawn(move || {
            scan(tx, i, addr, num_threads);
        });
    }

    // Collect results from threads
    let mut out = vec![];
    drop(tx);

    // Collect all results from the receiver into the out vector
    out.extend(rx);

    // Sort and print results
    out.sort();
    for v in out {
        println!("{} is open", v);
    }
}