//! Test utilities - Provide helper functions for testing

use std::fs;
use std::path::PathBuf;
use tempfile::{TempDir, tempdir};

/// Create temporary config directory for testing
///
/// # Panics
/// Panics if the temporary directory cannot be created.
#[must_use]
pub fn create_temp_config_dir() -> (TempDir, PathBuf) {
    let temp_dir = tempdir().expect("Failed to create temp directory");
    let config_dir = temp_dir.path().join("hostctl");

    fs::create_dir_all(&config_dir).expect("Failed to create config directory");

    (temp_dir, config_dir)
}

/// Create test hosts file content
#[must_use]
pub fn create_test_hosts_content() -> String {
    r"# Test hosts file
127.0.0.1 localhost
127.0.0.1 localhost.localdomain
::1 ip6-localhost ip6-loopback
fe00::0 ip6-localnet
ff00::0 ip6-mcastprefix
ff02::1 ip6-allnodes
ff02::2 ip6-allrouters

# Development entries
127.0.0.1 api.dev.local
127.0.0.1 db.dev.local

# Production entries
10.0.0.1 api.prod.com
10.0.0.2 db.prod.com
"
    .to_string()
}

/// Create simple test environment configuration
#[must_use]
pub fn create_test_config_yaml() -> String {
    r"current_environment: dev
environments:
  dev:
    name: dev
    description: Development environment
    entries:
      - ip: 127.0.0.1
        hostname: api.dev.local
        comment: null
      - ip: 127.0.0.1
        hostname: db.dev.local
        comment: Database server
  prod:
    name: prod
    description: Production environment
    entries:
      - ip: 10.0.0.1
        hostname: api.prod.com
        comment: null
      - ip: 10.0.0.2
        hostname: db.prod.com
        comment: Production database
"
    .to_string()
}

/// Validate hosts file format is correct
#[must_use]
pub fn validate_hosts_format(content: &str) -> bool {
    let lines: Vec<&str> = content.lines().collect();

    // Basic validation: each line should be a comment, empty line, or valid hosts entry
    for line in lines {
        let trimmed = line.trim();

        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }

        // Validate hosts entry format: IP address + hostname
        let parts: Vec<&str> = trimmed.split_whitespace().collect();
        if parts.len() < 2 {
            return false; // Invalid hosts entry
        }

        // Validate IP address format (simple validation)
        let ip_part = parts[0];
        if !ip_part.contains('.') && !ip_part.contains(':') {
            return false; // Not a valid IP address format
        }
    }

    true
}

/// Count valid entries in hosts file
#[must_use]
pub fn count_host_entries(content: &str) -> usize {
    content
        .lines()
        .filter(|line| {
            let trimmed = line.trim();
            !trimmed.is_empty() && !trimmed.starts_with('#')
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_temp_config_dir() {
        let (_temp_dir, config_dir) = create_temp_config_dir();

        assert!(config_dir.exists());
        assert!(config_dir.ends_with("hostctl"));
    }

    #[test]
    fn test_validate_hosts_format() {
        let valid_content = "127.0.0.1 localhost\n::1 ip6-localhost";
        assert!(validate_hosts_format(valid_content));

        let invalid_content = "invalid-format";
        assert!(!validate_hosts_format(invalid_content));
    }

    #[test]
    fn test_count_host_entries() {
        let content = r"# Comment line
127.0.0.1 localhost

# Another comment
::1 ip6-localhost
";

        assert_eq!(count_host_entries(content), 2);
    }
}
