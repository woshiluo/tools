extern crate pretty_env_logger;

enum Status {
    Bind,
    Unbind,
}

fn set_device(device: rusb::Device<rusb::GlobalContext>, status: Status) -> std::io::Result<()> {
    let bus_number = device.bus_number();
    let address = device.address();
    let device_name = format!("{:03}-{:03}", bus_number, address);
    log::info!(
        "{} {}",
        match status {
            Status::Bind => "binding",
            Status::Unbind => "Unbinding",
        },
        device_name
    );

    std::fs::write(
        match status {
            Status::Bind => "/sys/bus/usb/drivers/usb/bind",
            Status::Unbind => "/sys/bus/usb/drivers/usb/unbind",
        },
        format!("{}-{}", bus_number, address),
    )?;

    Ok(())
}

fn main() {
    pretty_env_logger::init();
    for device in rusb::devices().unwrap().iter() {
        match set_device(device, Status::Unbind) {
            Err(err) => log::error!("{}", err),
            _ => (),
        }
    }

    std::thread::sleep(std::time::Duration::from_secs(1));

    for device in rusb::devices().unwrap().iter() {
        match set_device(device, Status::Bind) {
            Err(err) => log::error!("{}", err),
            _ => (),
        }
    }
}
