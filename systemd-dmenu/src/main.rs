use dbus::blocking::Connection;
use serde::{Deserialize, Serialize};
use systemd_dmenu::Unit;

const START: &str = "Start";
const STOP: &str = "Stop";
const TOGGLE: &str = "Toggle";

#[derive(Serialize, Deserialize)]
struct Config {
    start: Option<Vec<String>>,
    stop: Option<Vec<String>>,
    toggle: Option<Vec<String>>,
}

pub fn gen_list() -> Result<(), Box<dyn std::error::Error>> {
    // Start a new session
    let conn = Connection::new_session()?;

    use std::io::Read;
    let mut raw_config = String::new();

    std::fs::File::open(format!(
        "{}/.config/systemd-dmenu/config.toml",
        env!("HOME")
    ))?
    .read_to_string(&mut raw_config)?;

    let config: Config = toml::from_str(&raw_config)?;

    let gen_list_from_data =
        |raw_data: Option<Vec<String>>, kind: &str| -> Result<(), dbus::Error> {
            if let Some(data) = raw_data {
                for service_name in data {
                    println!(
                        "{:<16}{:<16}{:<16}",
                        service_name,
                        Unit::new(&conn, &service_name)?
                            .get_status(&conn)?
                            .to_string(),
                        kind,
                    );
                }
            }
            Ok(())
        };

    gen_list_from_data(config.start, START)?;
    gen_list_from_data(config.stop, STOP)?;
    gen_list_from_data(config.toggle, TOGGLE)?;

    Ok(())
}

fn setting() -> Result<(), Box<dyn std::error::Error>> {
    // Start a new session
    let conn = Connection::new_session()?;

    let mut readin = String::new();
    std::io::stdin().read_line(&mut readin)?;

    let args = readin
        .split_whitespace()
        .map(String::from)
        .collect::<Vec<String>>();

    if args.len() != 3 {
        return Err(Box::new(dbus::Error::new_failed("Wrong Readin")));
    }

    let unit = Unit::new(&conn, &args[0])?;
    match args[2].as_str() {
        START => {
            unit.start(&conn)?;
        }
        STOP => {
            unit.stop(&conn)?;
        }
        TOGGLE => {
            if unit.is_active(&conn)? {
                unit.stop(&conn)?;
            } else {
                unit.start(&conn)?;
            }
        }
        _ => (),
    };

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() != 2 {
        return Err(Box::new(dbus::Error::new_failed("Wrong Readin")));
    }
    if args[1] == "list" {
        gen_list()?;
    } else if args[1] == "set" {
        setting()?;
    } else {
        return Err(Box::new(dbus::Error::new_failed("Wrong Readin")));
    };

    Ok(())
}
