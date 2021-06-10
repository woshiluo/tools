//! Systemd-dmenu

/// A Unit
pub struct Unit {
    /// Dbus path
    path: String,
}

#[derive(Debug)]
/// ActiveState of a Service
pub enum ActiveState {
    Active,
    Reloading,
    Inactive,
    Failed,
    Activating,
    Deactivating,
}

impl std::fmt::Display for ActiveState {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Unit {
    pub fn new(conn: &dbus::blocking::Connection, name: &str) -> Result<Unit, dbus::Error> {
        let proxy = conn.with_proxy(
            "org.freedesktop.systemd1",
            "/org/freedesktop/systemd1",
            std::time::Duration::from_millis(5000),
        );
        let (path,): (dbus::strings::Path,) = proxy.method_call(
            "org.freedesktop.systemd1.Manager",
            "GetUnit",
            (format!("{}.service", name),),
        )?;
        Ok(Unit {
            path: path.to_string(),
        })
    }

    fn get_proxy<'a>(
        &self,
        conn: &'a dbus::blocking::Connection,
    ) -> dbus::blocking::Proxy<&'a dbus::blocking::Connection> {
        conn.with_proxy(
            "org.freedesktop.systemd1",
            &self.path,
            std::time::Duration::from_millis(5000),
        )
    }
    pub fn get_status(
        &self,
        conn: &dbus::blocking::Connection,
    ) -> Result<ActiveState, dbus::Error> {
        use dbus::blocking::stdintf::org_freedesktop_dbus::Properties;

        let proxy = self.get_proxy(&conn);

        let state: String = proxy.get("org.freedesktop.systemd1.Unit", "ActiveState")?;
        match state.as_str() {
            "active" => Ok(ActiveState::Active),
            "reloading" => Ok(ActiveState::Reloading),
            "inactive" => Ok(ActiveState::Inactive),
            "failed" => Ok(ActiveState::Failed),
            "activating" => Ok(ActiveState::Activating),
            "deactivating" => Ok(ActiveState::Deactivating),
            _ => Err(dbus::Error::new_failed("Unknow state")),
        }
    }
    pub fn is_active(&self, conn: &dbus::blocking::Connection) -> Result<bool, dbus::Error> {
        let state = self.get_status(&conn)?;
        Ok(match state {
            ActiveState::Active => true,
            ActiveState::Activating => true,
            _ => false,
        })
    }
    pub fn stop(&self, conn: &dbus::blocking::Connection) -> Result<(), dbus::Error> {
        let proxy = self.get_proxy(&conn);

        proxy.method_call("org.freedesktop.systemd1.Unit", "Stop", ("replace",))
    }
    pub fn start(&self, conn: &dbus::blocking::Connection) -> Result<(), dbus::Error> {
        let proxy = self.get_proxy(&conn);

        proxy.method_call("org.freedesktop.systemd1.Unit", "Start", ("replace",))
    }
}
