use std::net::TcpListener;
use clap::Parser;
use std::process;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// Interface/IP address to bind to
    #[arg(short, long, default_value = "0.0.0.0")]
    interface: String,

    /// Number of times to bind and print a port
    #[arg(short, long, default_value_t = 1)]
    count: u32,
}

fn main() {
    let args = Args::parse();

    for _ in 0..args.count {
        // bind to <IP>:0 to let kernel pick an open ephemeral port
        let addr = format!("{}:{}", args.interface, 0);

        match TcpListener::bind(&addr) {
            Ok(listener) => {
                match listener.local_addr() {
                    Ok(local_addr) => {
                        // bound, print port
                        println!("{}", local_addr.port());
                    }
                    Err(e) => {
                        eprintln!("Failed to get local address: {e}");
                        process::exit(1);
                    }
                }
                // listener dropped here at end of loop iteration
            }
            Err(e) => {
                eprintln!("Failed to bind to {addr}: {e}");
                process::exit(1);
            }
        }
    }
}