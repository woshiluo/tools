extern crate i3ipc;
use i3ipc::I3Connection;

extern crate pretty_env_logger;

fn set_outputs(
    connection: &mut i3ipc::I3Connection,
    laptop_screen: &str,
) -> Result<(), i3ipc::MessageError> {
    let outputs = connection.get_outputs()?.outputs;
    let laptop_screen_status = {
        let mut result = false;
        for output in &outputs {
            if output.name == laptop_screen {
                result = output.active;
            }
        }
        result
    };

    if outputs.len() >= 2 {
        log::trace!("Have more than one display.");
        if laptop_screen_status {
            connection.run_command(&format!("output {} disable", laptop_screen))?;
        }
    } else {
        log::trace!("Only laptop screen");
        if !laptop_screen_status {
            connection.run_command(&format!("output {} enable", laptop_screen))?;
        }
    };

    Ok(())
}

fn main() {
    pretty_env_logger::init();
    let laptop_screen = std::env::args().nth(1).unwrap();
    log::info!("Get laptop Screen {}", laptop_screen);
    // Start connection
    let mut connection = I3Connection::connect().unwrap();

    loop {
        if let Err(err) = set_outputs(&mut connection, &laptop_screen) {
            log::error!("Failed to set output: {}", err);
        }
        std::thread::sleep(std::time::Duration::from_secs(5));
    }
}
