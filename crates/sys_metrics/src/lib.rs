pub mod host {
    use std::io;

    /// Returns the local machine hostname on Windows and Unix.
    pub fn get_hostname() -> io::Result<String> {
        #[cfg(windows)]
        {
            if let Ok(name) = std::env::var("COMPUTERNAME") {
                let trimmed = name.trim();
                if !trimmed.is_empty() {
                    return Ok(trimmed.to_string());
                }
            }
        }

        if let Ok(name) = std::env::var("HOSTNAME") {
            let trimmed = name.trim();
            if !trimmed.is_empty() {
                return Ok(trimmed.to_string());
            }
        }

        #[cfg(unix)]
        {
            if let Ok(out) = std::process::Command::new("hostname").output() {
                if out.status.success() {
                    let s = String::from_utf8_lossy(&out.stdout).trim().to_string();
                    if !s.is_empty() {
                        return Ok(s);
                    }
                }
            }
        }

        Ok("DropBridge-PC".to_string())
    }
}
