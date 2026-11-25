use local_ip_address::local_ip;
use mdns_sd::{ServiceDaemon, ServiceInfo};
use std::collections::HashMap;
use tracing::{error, info};

pub struct MdnsResponder {
    daemon: ServiceDaemon,
    service_fullname: String,
}

impl MdnsResponder {
    pub const SERVICE_TYPE: &'static str = "_dropbridge._tcp.local.";

    /// Starts advertising the DropBridge service on the local network
    pub fn start(
        device_name: &str,
        port: u16,
    ) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let daemon = ServiceDaemon::new()?;
        let my_ip = local_ip()?;

        let clean_name = sanitize_filename::sanitize(device_name);
        let clean_name = if clean_name.is_empty() {
            "PC".to_string()
        } else {
            clean_name
        };

        let instance_name = format!("DropBridge-{}", clean_name);
        let host_name = format!("{}.local.", clean_name.to_lowercase().replace(' ', "-"));

        let mut properties = HashMap::new();
        properties.insert("version".to_string(), env!("CARGO_PKG_VERSION").to_string());
        properties.insert("device_name".to_string(), device_name.to_string());
        properties.insert("port".to_string(), port.to_string());

        let service_info = ServiceInfo::new(
            Self::SERVICE_TYPE,
            &instance_name,
            &host_name,
            my_ip,
            port,
            properties,
        )?;

        let fullname = service_info.get_fullname().to_string();
        daemon.register(service_info)?;

        info!(
            "mDNS Service registered: {} at {}:{} ({})",
            fullname, my_ip, port, host_name
        );

        Ok(Self {
            daemon,
            service_fullname: fullname,
        })
    }

    /// Stops advertising the service on shutdown
    pub fn stop(&self) {
        if let Err(e) = self.daemon.unregister(&self.service_fullname) {
            error!("Failed to unregister mDNS service: {}", e);
        } else {
            info!("mDNS Service unregistered successfully");
        }
    }
}
