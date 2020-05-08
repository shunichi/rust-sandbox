use std::io::prelude::*;
use std::fs::OpenOptions;
use std::{thread, time};

// See https://wiki.archlinux.jp/index.php/Lenovo_ThinkPad_X1_Carbon_(Gen_6)

// echo -n "none" > /sys/bus/serio/devices/serio1/drvctl
// sleep 1
// echo -n "reconnect" > /sys/bus/serio/devices/serio1/drvctl

fn main() {
    let mut devctl = match OpenOptions::new().write(true).open("/sys/bus/serio/devices/serio1/drvctl") {
        Ok(file) => file,
        Err(e) => {
            if e.kind() == std::io::ErrorKind::PermissionDenied {
                eprintln!("Error: Permission denied. Use sudo.")
            } else {
                eprintln!("Error: {}", e);
            }
            std::process::exit(1);
        }
    };
    devctl.write(b"none").unwrap();

    let sleep_duration = time::Duration::from_millis(1000);
    thread::sleep(sleep_duration);

    devctl.write(b"reconnect").unwrap();
}
