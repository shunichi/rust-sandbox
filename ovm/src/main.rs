use std::vec::Vec;
use std::process::Command;
use std::os::unix::process::CommandExt;

fn main() {
    let port = portpicker::pick_unused_port().expect("No ports free");
    let args: Vec<_> = std::env::args().skip(1).collect();
    let error = Command::new("overmind")
        .args(&args)
        .env("WEBPACKER_DEV_SERVER_PORT", port.to_string())
        .env("WEBPACKER_DEV_SERVER_PUBLIC", format!("localhost:{}", port))
        .exec();
    eprintln!("{:?}", error);
    std::process::exit(1);
}
