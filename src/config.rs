use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::net::IpAddr;

/// Represents an entry in the hosts file
///
/// Contains IP address, hostname, and optional comment information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HostEntry {
    /// IP address (`IPv4` or `IPv6`)
    pub ip: IpAddr,
    /// Hostname
    pub hostname: String,
    /// Optional comment information
    pub comment: Option<String>,
}

impl HostEntry {
    /// Create a new hosts entry
    ///
    /// # Arguments
    /// * `ip` - IP address
    /// * `hostname` - Hostname
    ///
    /// # Example
    /// ```
    /// use std::net::Ipv4Addr;
    /// use hostctl::config::HostEntry;
    ///
    /// let entry = HostEntry::new(
    ///     std::net::IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
    ///     "localhost".to_string()
    /// );
    /// ```
    #[must_use]
    pub fn new(ip: IpAddr, hostname: String) -> Self {
        Self {
            ip,
            hostname,
            comment: None,
        }
    }

    /// Add comment to entry
    ///
    /// # Arguments
    /// * `comment` - Comment text
    ///
    /// # Returns
    /// Returns a new entry with the comment
    #[must_use]
    pub fn with_comment(mut self, comment: String) -> Self {
        self.comment = Some(comment);
        self
    }

    /// Convert entry to hosts file format string
    ///
    /// # Returns
    /// Returns a string in the format "IP hostname # comment"
    #[must_use]
    pub fn to_line(&self) -> String {
        match &self.comment {
            Some(comment) => format!("{} {} # {}", self.ip, self.hostname, comment),
            None => format!("{} {}", self.ip, self.hostname),
        }
    }
}

/// Represents an environment configuration
///
/// An environment contains a set of hosts entries, which can be used for different development or production scenarios.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Environment {
    /// Environment name
    pub name: String,
    /// Environment description
    pub description: Option<String>,
    /// List of hosts entries in this environment
    pub entries: Vec<HostEntry>,
}

impl Environment {
    /// Create a new empty environment
    ///
    /// # Arguments
    /// * `name` - Environment name
    ///
    /// # Example
    /// ```
    /// use hostctl::config::Environment;
    ///
    /// let env = Environment::new("development".to_string());
    /// ```
    #[must_use]
    pub fn new(name: String) -> Self {
        Self {
            name,
            description: None,
            entries: Vec::new(),
        }
    }

    /// Add description to environment
    ///
    /// # Arguments
    /// * `description` - Environment description text
    ///
    /// # Returns
    /// Returns a new environment with the description
    #[must_use]
    pub fn with_description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }

    /// Add a hosts entry to the environment
    ///
    /// # Arguments
    /// * `entry` - The hosts entry to add
    pub fn add_entry(&mut self, entry: HostEntry) {
        self.entries.push(entry);
    }

    /// Remove entry with specified hostname from the environment
    ///
    /// # Arguments
    /// * `hostname` - The hostname to remove
    ///
    /// # Returns
    /// Returns `true` if an entry was found and removed; otherwise returns `false`
    pub fn remove_entry(&mut self, hostname: &str) -> bool {
        self.entries
            .iter()
            .position(|e| e.hostname == hostname)
            .map(|pos| self.entries.remove(pos))
            .is_some()
    }

    /// Find entry with specified hostname in the environment
    ///
    /// # Arguments
    /// * `hostname` - The hostname to find
    ///
    /// # Returns
    /// Returns a reference to the entry if found; otherwise returns `None`
    #[must_use]
    pub fn find_entry(&self, hostname: &str) -> Option<&HostEntry> {
        self.entries.iter().find(|e| e.hostname == hostname)
    }
}

/// Main configuration structure
///
/// Contains all environment configurations and the currently active environment.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// Name of the currently active environment
    pub current_environment: Option<String>,
    /// Map of all environments, with environment names as keys
    pub environments: HashMap<String, Environment>,
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}

impl Config {
    /// Create a new empty configuration
    ///
    /// # Example
    /// ```
    /// use hostctl::config::Config;
    ///
    /// let config = Config::new();
    /// assert!(config.environments.is_empty());
    /// ```
    #[must_use]
    pub fn new() -> Self {
        Self {
            current_environment: None,
            environments: HashMap::new(),
        }
    }

    /// Add an environment to the configuration
    ///
    /// # Arguments
    /// * `env` - The environment to add
    ///
    /// # Note
    /// If an environment with the same name already exists, it will be overwritten
    pub fn add_environment(&mut self, env: Environment) {
        self.environments.insert(env.name.clone(), env);
    }

    /// Remove an environment from the configuration
    ///
    /// # Arguments
    /// * `name` - The name of the environment to remove
    ///
    /// # Returns
    /// Returns `true` if an environment was found and removed; otherwise returns `false`
    ///
    /// # Note
    /// If the removed environment is the currently active one, the current environment setting will be automatically cleared
    pub fn remove_environment(&mut self, name: &str) -> bool {
        let removed = self.environments.remove(name).is_some();

        // If the removed environment is the current one, clear the current environment setting
        if removed && self.current_environment.as_ref() == Some(&name.to_string()) {
            self.current_environment = None;
        }

        removed
    }

    /// Get environment with specified name
    ///
    /// # Arguments
    /// * `name` - Environment name
    ///
    /// # Returns
    /// Returns a reference to the environment if found; otherwise returns `None`
    #[must_use]
    pub fn get_environment(&self, name: &str) -> Option<&Environment> {
        self.environments.get(name)
    }

    /// Get mutable reference to environment with specified name
    ///
    /// # Arguments
    /// * `name` - Environment name
    ///
    /// # Returns
    /// Returns a mutable reference to the environment if found; otherwise returns `None`
    pub fn get_environment_mut(&mut self, name: &str) -> Option<&mut Environment> {
        self.environments.get_mut(name)
    }

    /// Get iterator of all environment names
    ///
    /// # Returns
    /// Returns an iterator of environment names
    pub fn environment_names(&self) -> impl Iterator<Item = &String> {
        self.environments.keys()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::{Ipv4Addr, Ipv6Addr};

    #[test]
    fn test_host_entry_creation() {
        let ip = IpAddr::V4(Ipv4Addr::LOCALHOST);
        let entry = HostEntry::new(ip, "localhost".to_string());

        assert_eq!(entry.ip, ip);
        assert_eq!(entry.hostname, "localhost");
        assert_eq!(entry.comment, None);
    }

    #[test]
    fn test_host_entry_with_comment() {
        let ip = IpAddr::V4(Ipv4Addr::new(192, 168, 1, 1));
        let entry =
            HostEntry::new(ip, "router".to_string()).with_comment("Local router".to_string());

        assert_eq!(entry.comment, Some("Local router".to_string()));
    }

    #[test]
    fn test_host_entry_to_line() {
        let ip = IpAddr::V4(Ipv4Addr::LOCALHOST);
        let entry = HostEntry::new(ip, "localhost".to_string());

        assert_eq!(entry.to_line(), "127.0.0.1 localhost");

        let entry_with_comment = entry.with_comment("Local host".to_string());
        assert_eq!(
            entry_with_comment.to_line(),
            "127.0.0.1 localhost # Local host"
        );
    }

    #[test]
    fn test_ipv6_host_entry() {
        let ip = IpAddr::V6(Ipv6Addr::LOCALHOST);
        let entry = HostEntry::new(ip, "ipv6-localhost".to_string());

        assert_eq!(entry.to_line(), "::1 ipv6-localhost");
    }

    #[test]
    fn test_environment_creation() {
        let env = Environment::new("dev".to_string());

        assert_eq!(env.name, "dev");
        assert_eq!(env.description, None);
        assert!(env.entries.is_empty());
    }

    #[test]
    fn test_environment_with_description() {
        let env = Environment::new("prod".to_string())
            .with_description("Production environment".to_string());

        assert_eq!(env.description, Some("Production environment".to_string()));
    }

    #[test]
    fn test_environment_add_remove_entries() {
        let mut env = Environment::new("test".to_string());
        let entry = HostEntry::new(
            IpAddr::V4(Ipv4Addr::new(10, 0, 0, 1)),
            "api.internal".to_string(),
        );

        // Add entry
        env.add_entry(entry.clone());
        assert_eq!(env.entries.len(), 1);
        assert!(env.find_entry("api.internal").is_some());

        // Remove entry
        assert!(env.remove_entry("api.internal"));
        assert!(env.entries.is_empty());
        assert!(env.find_entry("api.internal").is_none());

        // Remove non-existent entry
        assert!(!env.remove_entry("nonexistent"));
    }

    #[test]
    fn test_config_creation() {
        let config = Config::new();

        assert!(config.environments.is_empty());
        assert_eq!(config.current_environment, None);
    }

    #[test]
    fn test_config_add_environment() {
        let mut config = Config::new();
        let env = Environment::new("staging".to_string());

        config.add_environment(env);
        assert_eq!(config.environments.len(), 1);
        assert!(config.get_environment("staging").is_some());
    }

    #[test]
    fn test_config_remove_environment() {
        let mut config = Config::new();
        let env = Environment::new("qa".to_string());

        config.add_environment(env);
        assert!(config.remove_environment("qa"));
        assert!(config.environments.is_empty());
        assert!(config.get_environment("qa").is_none());

        // Remove non-existent environment
        assert!(!config.remove_environment("nonexistent"));
    }

    #[test]
    fn test_config_environment_names() {
        let mut config = Config::new();

        config.add_environment(Environment::new("dev".to_string()));
        config.add_environment(Environment::new("prod".to_string()));

        let names: Vec<&String> = config.environment_names().collect();
        assert_eq!(names.len(), 2);
        assert!(names.contains(&&"dev".to_string()));
        assert!(names.contains(&&"prod".to_string()));
    }

    #[test]
    fn test_config_current_environment_removal() {
        let mut config = Config::new();
        let env = Environment::new("current".to_string());

        config.add_environment(env);
        config.current_environment = Some("current".to_string());

        config.remove_environment("current");
        assert_eq!(config.current_environment, None);
    }
}
