use std::vec::Vec;
use subprocess::{Exec, ExitStatus};
use std::convert::TryFrom;

fn main() {
    let port = portpicker::pick_unused_port().expect("No ports free");
    let args: Vec<_> = std::env::args().skip(1).collect();
    let result = Exec::cmd("overmind")
        .args(&args)
        .env("WEBPACKER_DEV_SERVER_PORT", port.to_string())
        .env("WEBPACKER_DEV_SERVER_PUBLIC", format!("localhost:{}", port))
        .join();
    match result {
        Ok(ExitStatus::Exited(code)) => std::process::exit(TryFrom::try_from(code).expect("Exit code is too large")),
        _ => std::process::exit(1),
    }
}
