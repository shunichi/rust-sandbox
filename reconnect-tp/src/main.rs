use std::fs::{self, OpenOptions};
use std::io::prelude::*;
use std::path::PathBuf;
use std::{thread, time};

// See https://wiki.archlinux.jp/index.php/Lenovo_ThinkPad_X1_Carbon_(Gen_6)

// echo -n "none" > /sys/bus/serio/devices/serioX/drvctl
// sleep 1
// echo -n "reconnect" > /sys/bus/serio/devices/serioX/drvctl

struct Options {
    loop_mode: bool,
    force: bool,
    verbose: bool,
}

fn usage() {
    eprint!(
        "Usage: reconnect-tp [OPTIONS]

OPTIONS:
  -l         loop mode
  -f         reconnect even if already connected
  --verbose  verbose output
"
    );
    std::process::exit(1);
}

fn parse_options() -> Options {
    let mut options = Options {
        loop_mode: false,
        force: false,
        verbose: false,
    };
    for argument in std::env::args().skip(1) {
        match argument.as_str() {
            "-l" => {
                options.loop_mode = true;
            }
            "-f" => {
                options.force = true;
            }
            "--verbose" => {
                options.verbose = true;
            }
            _ => {
                usage();
            }
        }
    }
    if options.loop_mode {
        options.force = false;
    }
    return options;
}

fn elan_trackppoint_exists() -> bool {
    let input_device_dir = "/sys/class/input/";
    for entry in fs::read_dir(input_device_dir).unwrap() {
        let path = entry.unwrap().path();
        let os_file_name = path.file_name().unwrap();
        let file_name = os_file_name.to_string_lossy();
        if !file_name.starts_with("mouse") {
            continue;
        }

        let mut device_name_path = path.clone();
        device_name_path.push("device/name");
        if !device_name_path.exists() {
            continue;
        }
        let name = fs::read_to_string(device_name_path).unwrap();
        if name == "TPPS/2 Elan TrackPoint\n" {
            return true;
        }
    }
    return false;
}

fn rmi4_device_path() -> String {
    let rmi4_description = "RMI4 PS/2 pass-through\n";
    let serio_device_dir = "/sys/bus/serio/devices";

    for entry in fs::read_dir(serio_device_dir).unwrap() {
        let entry_path = entry.unwrap().path();
        let mut description_path = entry_path.clone();
        description_path.push("description");
        if description_path.exists() {
            let description = fs::read_to_string(description_path.as_os_str()).unwrap();
            println!("{}: {}", description_path.to_string_lossy(), description);
            if description == rmi4_description {
                return entry_path.into_os_string().into_string().unwrap();
            }
        } else {
            println!("{}: not exists", description_path.to_string_lossy())
        }
    }
    panic!("RMI4 device not found!");
}

fn reconnect_device(device_path: &str) {
    let mut drvctl_path = PathBuf::from(device_path);
    drvctl_path.push("drvctl");
    let mut devctl = match OpenOptions::new().write(true).open(&drvctl_path) {
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

fn do_loop(options: &Options) {
    do_oneshot(&options, true);
    loop {
        std::thread::sleep(time::Duration::from_millis(2000));
        do_oneshot(&options, false);
    }
}

fn reconnect() {
    let drvctl_path = rmi4_device_path();
    println!("Reconnecting RMI4 device: {}", &drvctl_path);
    reconnect_device(&drvctl_path);
}

fn do_oneshot(options: &Options, force_output: bool) {
    if options.force {
        reconnect();
    } else if elan_trackppoint_exists() {
        if force_output {
            println!("TPPS/2 Elan TrackPoint found!");
        }
    } else {
        println!("TPPS/2 Elan TrackPoint not found!");
        reconnect();
    }
}

fn main() {
    let options = parse_options();
    if options.loop_mode {
        do_loop(&options);
    } else {
        do_oneshot(&options, false);
    }
}
