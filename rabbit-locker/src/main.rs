/// heavily inspired from the code here: https://github.com/geoffreycopin/http_server/blob/master/src/main.rs
/// https://dev.to/geoffreycopin/-build-a-web-server-with-rust-and-tokio-part-0-the-simplest-possible-get-handler-1lhi
/// Thank you, for providing that example

use clap::Parser;

/// Simple webserver that calls iptables to block for some time
/// a specific port
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Port to listen at
    #[arg(short, long)]
    port: usize,

    /// Port to block
    #[arg(short, long)]
    blocked_port: usize,
}

#[tokio::main]
async fn main() {
    println!("Hello, world!");
}
