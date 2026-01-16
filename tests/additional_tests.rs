//! Additional tests - Test additional functionality of hostctl

use hostctl::config::{Config, Environment, HostEntry};
use hostctl::hosts::HostsManager;
use serde_yaml_ok as serde_yaml;
use std::net::{Ipv4Addr, Ipv6Addr};

#[cfg(test)]
mod tests {
    use super::*;

    /// Test applying an empty environment
    #[test]
    fn test_apply_empty_environment() {
        let env = Environment::new("empty".to_string());

        // Empty environment should be created successfully
        assert_eq!(env.name, "empty");
        assert!(env.entries.is_empty());
        assert_eq!(env.description, None);
    }

    /// Test environment with large number of entries
    #[test]
    fn test_apply_large_environment() {
        let mut env = Environment::new("large".to_string());

        // Add 1000 entries
        for i in 0..1000 {
            let ip = Ipv4Addr::new(10, 0, (i / 256) as u8, (i % 256) as u8);
            env.add_entry(HostEntry::new(std::net::IpAddr::V4(ip), format!("host{i}")));
        }

        // Verify all entries were added
        assert_eq!(env.entries.len(), 1000);

        // Verify we can find specific entries
        let entry = env.find_entry("host500");
        assert!(entry.is_some());
        assert_eq!(entry.unwrap().hostname, "host500");
    }

    /// Test handling of duplicate hostnames
    #[test]
    fn test_duplicate_hostnames() {
        let mut env = Environment::new("test".to_string());

        let entry1 = HostEntry::new(
            std::net::IpAddr::V4(Ipv4Addr::new(192, 168, 1, 1)),
            "duplicate".to_string(),
        );

        let entry2 = HostEntry::new(
            std::net::IpAddr::V4(Ipv4Addr::new(192, 168, 1, 2)),
            "duplicate".to_string(),
        );

        env.add_entry(entry1);
        env.add_entry(entry2);

        // Both entries should be added
        assert_eq!(env.entries.len(), 2);

        // find_entry only returns the first match
        let found = env.find_entry("duplicate");
        assert!(found.is_some());
        assert_eq!(
            found.unwrap().ip,
            std::net::IpAddr::V4(Ipv4Addr::new(192, 168, 1, 1))
        );
    }

    /// Test complete `IPv6` address handling
    #[test]
    fn test_ipv6_full_address() {
        let ip = std::net::IpAddr::V6(Ipv6Addr::new(
            0x2001, 0x0db8, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0001,
        ));
        let entry = HostEntry::new(ip, "ipv6-full.example.com".to_string());

        assert_eq!(entry.to_line(), "2001:db8::1 ipv6-full.example.com");
    }

    /// Test hostname edge cases
    #[test]
    fn test_hostname_edge_cases() {
        // Minimum valid hostname (1 character)
        assert!(HostsManager::is_valid_hostname("a"));

        // Maximum valid hostname (253 characters) - needs to be split into multiple labels
        let long_name = format!("{}.{}", "a".repeat(63), "b".repeat(63));
        assert_eq!(long_name.len(), 127); // 63 + 1 + 63
        assert!(HostsManager::is_valid_hostname(&long_name));

        // Exceeds maximum length (254 characters)
        let too_long = "a".repeat(254);
        assert!(!HostsManager::is_valid_hostname(&too_long));

        // Maximum length for a single label (63 characters)
        let max_label = "a".repeat(63);
        assert!(HostsManager::is_valid_hostname(&max_label));

        // Single label exceeds maximum length (64 characters)
        let too_long_label = "a".repeat(64);
        assert!(!HostsManager::is_valid_hostname(&too_long_label));

        // Starts with hyphen
        assert!(!HostsManager::is_valid_hostname("-invalid"));

        // Ends with hyphen
        assert!(!HostsManager::is_valid_hostname("invalid-"));

        // Consecutive dots
        assert!(!HostsManager::is_valid_hostname("invalid..com"));

        // Starts with dot
        assert!(!HostsManager::is_valid_hostname(".invalid.com"));

        // Ends with dot
        assert!(!HostsManager::is_valid_hostname("invalid.com."));

        // Contains space
        assert!(!HostsManager::is_valid_hostname("invalid host.com"));

        // Contains special characters
        assert!(!HostsManager::is_valid_hostname("invalid@host.com"));
        assert!(!HostsManager::is_valid_hostname("invalid_host.com"));

        // Underscore is invalid in hostnames
        assert!(!HostsManager::is_valid_hostname("test_environment"));
    }

    /// Test IP address edge cases
    #[test]
    fn test_ip_edge_cases() {
        // Valid IPv4 addresses
        assert!(HostsManager::is_valid_ip("0.0.0.0"));
        assert!(HostsManager::is_valid_ip("255.255.255.255"));
        assert!(HostsManager::is_valid_ip("127.0.0.1"));

        // Invalid IPv4 addresses
        assert!(!HostsManager::is_valid_ip("256.0.0.0"));
        assert!(!HostsManager::is_valid_ip("192.168.1"));
        assert!(!HostsManager::is_valid_ip("192.168.1.1.1"));

        // Valid IPv6 addresses
        assert!(HostsManager::is_valid_ip("::1"));
        assert!(HostsManager::is_valid_ip("2001:db8::1"));
        assert!(HostsManager::is_valid_ip("fe80::1"));

        // Invalid IPv6 addresses
        assert!(!HostsManager::is_valid_ip(":::1"));
        assert!(!HostsManager::is_valid_ip("2001:db8:::1"));

        // Empty string
        assert!(!HostsManager::is_valid_ip(""));

        // Random text
        assert!(!HostsManager::is_valid_ip("not an ip"));
    }

    /// Test configuration serialization and deserialization
    #[test]
    fn test_config_serialization_roundtrip() {
        let mut config = Config::new();

        // Add multiple environments
        let mut dev_env = Environment::new("dev".to_string())
            .with_description("Development environment".to_string());
        dev_env.add_entry(HostEntry::new(
            std::net::IpAddr::V4(Ipv4Addr::new(10, 0, 0, 1)),
            "api.dev".to_string(),
        ));

        let mut prod_env = Environment::new("prod".to_string())
            .with_description("Production environment".to_string());
        prod_env.add_entry(HostEntry::new(
            std::net::IpAddr::V4(Ipv4Addr::new(10, 0, 1, 1)),
            "api.prod".to_string(),
        ));

        config.add_environment(dev_env);
        config.add_environment(prod_env);
        config.current_environment = Some("dev".to_string());

        // Serialize
        let yaml = serde_yaml::to_string(&config).unwrap();

        // Deserialize
        let deserialized: Config = serde_yaml::from_str(&yaml).unwrap();

        // Verify data integrity
        assert_eq!(deserialized.environments.len(), 2);
        assert_eq!(deserialized.current_environment, Some("dev".to_string()));

        let dev = deserialized.get_environment("dev").unwrap();
        assert_eq!(dev.description, Some("Development environment".to_string()));
        assert_eq!(dev.entries.len(), 1);
        assert_eq!(dev.entries[0].hostname, "api.dev");

        let prod = deserialized.get_environment("prod").unwrap();
        assert_eq!(prod.description, Some("Production environment".to_string()));
        assert_eq!(prod.entries.len(), 1);
        assert_eq!(prod.entries[0].hostname, "api.prod");
    }

    /// Test environment description handling
    #[test]
    fn test_environment_description() {
        let env1 = Environment::new("test1".to_string());
        assert_eq!(env1.description, None);

        let env2 =
            Environment::new("test2".to_string()).with_description("Test environment".to_string());
        assert_eq!(env2.description, Some("Test environment".to_string()));

        // Empty description
        let env3 = Environment::new("test3".to_string()).with_description(String::new());
        assert_eq!(env3.description, Some(String::new()));
    }

    /// Test entry comment handling
    #[test]
    fn test_entry_comment() {
        let entry1 = HostEntry::new(
            std::net::IpAddr::V4(Ipv4Addr::LOCALHOST),
            "localhost".to_string(),
        );
        assert_eq!(entry1.comment, None);

        let entry2 = HostEntry::new(
            std::net::IpAddr::V4(Ipv4Addr::LOCALHOST),
            "localhost".to_string(),
        )
        .with_comment("Local host".to_string());
        assert_eq!(entry2.comment, Some("Local host".to_string()));

        // Empty comment
        let entry3 = HostEntry::new(
            std::net::IpAddr::V4(Ipv4Addr::LOCALHOST),
            "localhost".to_string(),
        )
        .with_comment(String::new());
        assert_eq!(entry3.comment, Some(String::new()));
    }

    /// Test configuration default implementation
    #[test]
    fn test_config_default() {
        let config = Config::default();

        assert!(config.environments.is_empty());
        assert_eq!(config.current_environment, None);
    }

    /// Test behavior when removing current environment
    #[test]
    fn test_remove_current_environment() {
        let mut config = Config::new();

        // Add environments
        config.add_environment(Environment::new("env1".to_string()));
        config.add_environment(Environment::new("env2".to_string()));

        // Set current environment
        config.current_environment = Some("env1".to_string());

        // Remove current environment
        assert!(config.remove_environment("env1"));

        // Verify current environment is cleared
        assert_eq!(config.current_environment, None);

        // Verify environment is removed
        assert!(config.get_environment("env1").is_none());
        assert!(config.get_environment("env2").is_some());
    }

    /// Test parsing hosts file lines (various formats)
    #[test]
    fn test_parse_hosts_line_various_formats() {
        // Standard format
        let entry = HostsManager::parse_hosts_line("127.0.0.1 localhost").unwrap();
        assert_eq!(entry.hostname, "localhost");
        assert_eq!(entry.comment, None);

        // With comment
        let entry = HostsManager::parse_hosts_line("127.0.0.1 localhost # comment").unwrap();
        assert_eq!(entry.hostname, "localhost");
        assert_eq!(entry.comment, Some("comment".to_string()));

        // Multiple spaces
        let entry = HostsManager::parse_hosts_line("127.0.0.1    localhost").unwrap();
        assert_eq!(entry.hostname, "localhost");

        // Tab
        let entry = HostsManager::parse_hosts_line("127.0.0.1\tlocalhost").unwrap();
        assert_eq!(entry.hostname, "localhost");

        // Mixed whitespace
        let entry = HostsManager::parse_hosts_line("127.0.0.1 \t localhost").unwrap();
        assert_eq!(entry.hostname, "localhost");

        // Leading spaces
        let entry = HostsManager::parse_hosts_line("  127.0.0.1 localhost").unwrap();
        assert_eq!(entry.hostname, "localhost");

        // Trailing spaces
        let entry = HostsManager::parse_hosts_line("127.0.0.1 localhost  ").unwrap();
        assert_eq!(entry.hostname, "localhost");
    }

    /// Test environment name validation
    #[test]
    fn test_environment_name_validation() {
        // Valid environment names
        assert!(HostsManager::is_valid_hostname("dev"));
        assert!(HostsManager::is_valid_hostname("production"));
        assert!(HostsManager::is_valid_hostname("staging-1"));
        assert!(HostsManager::is_valid_hostname("test-environment"));

        // Invalid environment names
        assert!(!HostsManager::is_valid_hostname(""));
        assert!(!HostsManager::is_valid_hostname("invalid name"));
        assert!(!HostsManager::is_valid_hostname("invalid@name"));
        assert!(!HostsManager::is_valid_hostname("-invalid"));
        assert!(!HostsManager::is_valid_hostname("invalid-"));
        assert!(!HostsManager::is_valid_hostname("test_environment"));
    }
}
