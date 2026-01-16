use crate::config::{Environment, HostEntry};
use anyhow::{Context, Result};
use std::fs;
use std::io::{BufRead, BufReader};
use std::net::IpAddr;
use std::path::Path;

/// Hosts file manager
///
/// Responsible for reading, parsing, and writing the system hosts file.
pub struct HostsManager;

impl HostsManager {
    /// Get the path to the system hosts file
    ///
    /// Returns different paths based on the operating system:
    /// - Windows: `C:\Windows\System32\drivers\etc\hosts`
    /// - Linux/macOS: `/etc/hosts`
    #[cfg(target_os = "windows")]
    fn get_hosts_path() -> &'static str {
        r"C:\Windows\System32\drivers\etc\hosts"
    }

    /// Get the path to the system hosts file
    ///
    /// Returns different paths based on the operating system:
    /// - Windows: `C:\Windows\System32\drivers\etc\hosts`
    /// - Linux/macOS: `/etc/hosts`
    #[cfg(any(target_os = "linux", target_os = "macos"))]
    fn get_hosts_path() -> &'static str {
        "/etc/hosts"
    }

    /// Read and parse the current hosts file
    ///
    /// # Returns
    /// Returns a vector containing all hosts entries
    ///
    /// # Errors
    /// Returns an error if the hosts file cannot be read.
    pub fn read_current_hosts() -> Result<Vec<HostEntry>> {
        let path = Self::get_hosts_path();
        let file =
            fs::File::open(path).with_context(|| format!("Failed to open hosts file: {path}"))?;

        let reader = BufReader::new(file);
        let mut entries = Vec::new();

        for line in reader.lines().map_while(Result::ok) {
            if let Some(entry) = Self::parse_hosts_line(&line) {
                entries.push(entry);
            }
        }

        Ok(entries)
    }

    /// Parse a line from the hosts file
    ///
    /// # Arguments
    /// * `line` - The line to parse
    ///
    /// # Returns
    /// Returns `Some(HostEntry)` if the line contains a valid hosts entry; otherwise returns `None`
    ///
    /// # Format
    /// Supports the following formats:
    /// - `IP hostname`
    /// - `IP hostname # comment`
    /// - `IP hostname1 hostname2 # comment`
    #[must_use]
    pub fn parse_hosts_line(line: &str) -> Option<HostEntry> {
        let line = line.trim();

        // Skip empty lines and comment lines
        if line.is_empty() || line.starts_with('#') {
            return None;
        }

        // Separate comment
        let (content, comment) = if let Some(pos) = line.find('#') {
            (&line[..pos], Some(line[pos + 1..].trim().to_string()))
        } else {
            (line, None)
        };

        // Parse IP and hostname
        let parts: Vec<&str> = content.split_whitespace().collect();
        if parts.len() < 2 {
            return None;
        }

        let ip: IpAddr = parts[0].parse().ok()?;
        let hostname = parts[1].to_string();

        Some(HostEntry {
            ip,
            hostname,
            comment,
        })
    }

    /// Apply the specified environment configuration to the system hosts file
    ///
    /// This operation backs up the current hosts file, then writes the new configuration.
    ///
    /// # Arguments
    /// * `env` - The environment configuration to apply
    ///
    /// # Errors
    /// Returns an error if the hosts file cannot be read or written.
    pub fn apply_environment(env: &Environment) -> Result<()> {
        // First backup the current hosts file
        Self::backup_hosts_file()?;

        // Read current hosts file content
        let path = Self::get_hosts_path();
        let current_content = fs::read_to_string(path)
            .with_context(|| format!("Failed to read hosts file: {path}"))?;

        // Separate hostctl managed entries and system entries
        let (system_entries, _managed_entries) = Self::separate_entries(&current_content);

        // Build new hosts file content
        let mut new_content = String::new();

        // Add system entries
        for entry in system_entries {
            new_content.push_str(&entry.to_line());
            new_content.push('\n');
        }

        // Add separator
        new_content.push_str("\n# ===== hostctl managed entries =====\n");

        // Add environment entries
        for entry in &env.entries {
            new_content.push_str(&entry.to_line());
            new_content.push('\n');
        }

        // Write new hosts file
        fs::write(path, new_content)
            .with_context(|| format!("Failed to write hosts file: {path}"))?;

        Ok(())
    }

    /// Backup the current hosts file
    ///
    /// Backup file name format is `hosts.backup.YYYYMMDD_HHMMSS`
    ///
    /// # Returns
    /// Returns the path to the backup file
    ///
    /// # Errors
    /// Returns an error if the backup file cannot be created.
    fn backup_hosts_file() -> Result<std::path::PathBuf> {
        let path = Path::new(Self::get_hosts_path());
        let timestamp = chrono::Local::now().format("%Y%m%d_%H%M%S");
        let backup_path = path.with_file_name(format!("hosts.backup.{timestamp}"));

        fs::copy(path, &backup_path)
            .with_context(|| format!("Failed to create backup: {}", backup_path.display()))?;

        Ok(backup_path)
    }

    /// Separate system entries and hostctl managed entries
    ///
    /// # Arguments
    /// * `content` - The hosts file content
    ///
    /// # Returns
    /// Returns a tuple: (system entries, hostctl managed entries)
    fn separate_entries(content: &str) -> (Vec<HostEntry>, Vec<HostEntry>) {
        let mut system_entries = Vec::new();
        let mut managed_entries = Vec::new();
        let mut in_managed_section = false;

        for line in content.lines() {
            // Check if entering or exiting hostctl managed section
            if line.contains("hostctl managed entries") {
                in_managed_section = true;
                continue;
            }

            if let Some(entry) = Self::parse_hosts_line(line) {
                if in_managed_section {
                    managed_entries.push(entry);
                } else {
                    system_entries.push(entry);
                }
            }
        }

        (system_entries, managed_entries)
    }

    /// Validate if hostname format is valid
    ///
    /// # Arguments
    /// * `hostname` - The hostname to validate
    ///
    /// # Returns
    /// Returns `true` if the hostname format is valid; otherwise returns `false`
    ///
    /// # Rules
    /// - Length does not exceed 253 characters
    /// - Contains only letters, numbers, hyphens, and dots
    /// - Does not start or end with a hyphen
    /// - Labels are separated by dots
    #[must_use]
    pub fn is_valid_hostname(hostname: &str) -> bool {
        if hostname.is_empty() || hostname.len() > 253 {
            return false;
        }

        // Check each label
        for label in hostname.split('.') {
            if label.is_empty() || label.len() > 63 {
                return false;
            }

            // Label cannot start or end with hyphen
            if label.starts_with('-') || label.ends_with('-') {
                return false;
            }

            // Label can only contain letters, numbers, and hyphens
            if !label.chars().all(|c| c.is_alphanumeric() || c == '-') {
                return false;
            }
        }

        true
    }

    /// Validate if IP address format is valid
    ///
    /// # Arguments
    /// * `ip_str` - The IP address string to validate
    ///
    /// # Returns
    /// Returns `true` if the IP address format is valid; otherwise returns `false`
    #[must_use]
    pub fn is_valid_ip(ip_str: &str) -> bool {
        ip_str.parse::<IpAddr>().is_ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::{Ipv4Addr, Ipv6Addr};

    #[test]
    fn test_parse_hosts_line_valid() {
        let line = "127.0.0.1 localhost";
        let entry = HostsManager::parse_hosts_line(line).unwrap();

        assert_eq!(entry.ip, IpAddr::V4(Ipv4Addr::LOCALHOST));
        assert_eq!(entry.hostname, "localhost");
        assert_eq!(entry.comment, None);
    }

    #[test]
    fn test_parse_hosts_line_with_comment() {
        let line = "192.168.1.1 router # Local router";
        let entry = HostsManager::parse_hosts_line(line).unwrap();

        assert_eq!(entry.ip, IpAddr::V4(Ipv4Addr::new(192, 168, 1, 1)));
        assert_eq!(entry.hostname, "router");
        assert_eq!(entry.comment, Some("Local router".to_string()));
    }

    #[test]
    fn test_parse_hosts_line_ipv6() {
        let line = "::1 ipv6-localhost";
        let entry = HostsManager::parse_hosts_line(line).unwrap();

        assert_eq!(entry.ip, IpAddr::V6(Ipv6Addr::LOCALHOST));
        assert_eq!(entry.hostname, "ipv6-localhost");
    }

    #[test]
    fn test_parse_hosts_line_empty() {
        assert!(HostsManager::parse_hosts_line("").is_none());
        assert!(HostsManager::parse_hosts_line("   ").is_none());
    }

    #[test]
    fn test_parse_hosts_line_comment_only() {
        assert!(HostsManager::parse_hosts_line("# This is a comment").is_none());
    }

    #[test]
    fn test_parse_hosts_line_invalid() {
        assert!(HostsManager::parse_hosts_line("invalid line").is_none());
        assert!(HostsManager::parse_hosts_line("127.0.0.1").is_none());
    }

    #[test]
    fn test_is_valid_hostname() {
        // Valid hostnames
        assert!(HostsManager::is_valid_hostname("localhost"));
        assert!(HostsManager::is_valid_hostname("example.com"));
        assert!(HostsManager::is_valid_hostname("sub.domain.example.com"));
        assert!(HostsManager::is_valid_hostname("my-server"));
        assert!(HostsManager::is_valid_hostname("server-1"));

        // Invalid hostnames
        assert!(!HostsManager::is_valid_hostname(""));
        assert!(!HostsManager::is_valid_hostname("-invalid"));
        assert!(!HostsManager::is_valid_hostname("invalid-"));
        assert!(!HostsManager::is_valid_hostname("invalid..com"));
        assert!(!HostsManager::is_valid_hostname("invalid space.com"));
        assert!(!HostsManager::is_valid_hostname("a".repeat(254).as_str()));
    }

    #[test]
    fn test_is_valid_ip() {
        // Valid IP addresses
        assert!(HostsManager::is_valid_ip("127.0.0.1"));
        assert!(HostsManager::is_valid_ip("192.168.1.1"));
        assert!(HostsManager::is_valid_ip("::1"));
        assert!(HostsManager::is_valid_ip("2001:db8::1"));

        // Invalid IP addresses
        assert!(!HostsManager::is_valid_ip("256.256.256.256"));
        assert!(!HostsManager::is_valid_ip("invalid"));
        assert!(!HostsManager::is_valid_ip(""));
    }

    #[test]
    fn test_separate_entries() {
        let content = r"127.0.0.1 localhost
192.168.1.1 router

# ===== hostctl managed entries =====
10.0.0.1 api.dev
10.0.0.2 db.dev
";

        let (system, managed) = HostsManager::separate_entries(content);

        assert_eq!(system.len(), 2);
        assert_eq!(managed.len(), 2);

        assert_eq!(system[0].hostname, "localhost");
        assert_eq!(system[1].hostname, "router");
        assert_eq!(managed[0].hostname, "api.dev");
        assert_eq!(managed[1].hostname, "db.dev");
    }

    #[test]
    fn test_separate_entries_no_managed_section() {
        let content = r"127.0.0.1 localhost
192.168.1.1 router
";

        let (system, managed) = HostsManager::separate_entries(content);

        assert_eq!(system.len(), 2);
        assert_eq!(managed.len(), 0);
    }

    #[test]
    fn test_host_entry_to_line() {
        let entry = HostEntry::new(
            IpAddr::V4(Ipv4Addr::new(10, 0, 0, 1)),
            "api.example.com".to_string(),
        );

        assert_eq!(entry.to_line(), "10.0.0.1 api.example.com");

        let entry_with_comment = entry.with_comment("API server".to_string());
        assert_eq!(
            entry_with_comment.to_line(),
            "10.0.0.1 api.example.com # API server"
        );
    }

    #[test]
    fn test_parse_hosts_line_multiple_hostnames() {
        // hosts file can contain multiple hostnames on one line, but we only take the first one
        let line = "127.0.0.1 localhost localhost.localdomain";
        let entry = HostsManager::parse_hosts_line(line).unwrap();

        assert_eq!(entry.hostname, "localhost");
    }

    #[test]
    fn test_parse_hosts_line_with_tabs() {
        let line = "127.0.0.1\tlocalhost\t# Local host";
        let entry = HostsManager::parse_hosts_line(line).unwrap();

        assert_eq!(entry.hostname, "localhost");
        assert_eq!(entry.comment, Some("Local host".to_string()));
    }
}
