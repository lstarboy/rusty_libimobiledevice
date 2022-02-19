// jkcoxson

use rusty_libimobiledevice::libimobiledevice;

fn main() {
    const VERSION: &str = "0.1.0";

    let mut udid = "".to_string();
    let mut dmg_path = "".to_string();
    let mut image_type = "Developer".to_string();
    let mut display_xml = false;

    // Parse arguments
    let mut args: Vec<String> = std::env::args().collect();
    args.remove(0);
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "-u" | "--udid" => {
                udid = args[i + 1].clone();
            }
            "-l" | "--list" => {
                todo!();
            }
            "-t" | "--imagetype" => {
                image_type = args[i + 1].clone();
            }
            "-x" | "--xml" => {
                display_xml = true;
            }
            "-h" | "--help" => {
                println!("Usage: ideviceimagemounter [options]");
                println!("");
                println!("Options:");
                println!("  -u, --udid <udid>    : udid of the device to mount");
                println!("  -l, --list           : list all devices");
                println!("  -t, --imagetype <type> : image type to mount (Developer, Distribution, or Recovery)");
                println!("  -x, --xml            : display xml plist");
                println!("  -h, --help           : display this help message");
                println!("  -v, --version        : display version");
                return;
            }
            "-v" | "--version" => {
                println!("v{}", VERSION);
                return;
            }
            _ => {
                if args[i].starts_with("-") {
                    println!("Unknown flag: {}", args[i]);
                    return;
                }
                dmg_path = args[i].clone();
            }
        }
        i += 1;
    }
    if udid == "" {
        println!("Error: No UDID specified");
        return;
    }
    if dmg_path == "" {
        println!("Error: No DMG specified");
        return;
    }

    // Get the device
    let mut device = match libimobiledevice::get_device(udid.to_string()) {
        Some(device) => device,
        None => {
            println!("Error: Could not find device");
            return;
        }
    };

    // Start the lockdownd service
    match device.start_lockdownd_service("ideviceimagemounter".to_string()) {
        Ok(()) => {}
        Err(e) => {
            println!("Error starting lockdown service: {:?}", e);
            return;
        }
    }

    println!("Product Version: {}", device.lockdownd_get_value("ProductVersion".to_string(), "".to_string()).unwrap());


}

