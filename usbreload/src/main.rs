extern crate pretty_env_logger;

fn main() {
    pretty_env_logger::init();
    for device in rusb::devices().unwrap().iter() {
        let device_name = format!("{:03}-{:03}", device.bus_number(), device.address());
        log::info!("Reseting {}", device_name);
        let mut handle = match device.open() {
            Ok(content) => content,
            Err(err) => {
                log::error!("Failed to open {}: {}", device_name, err);
                continue;
            }
        };
        match handle.reset() {
            Ok(content) => content,
            Err(err) => {
                log::error!("Failed to reset {}: {}", device_name, err);
                continue;
            }
        };
    }
}
