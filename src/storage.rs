use crate::config::Config;
use anyhow::{Context, Result};
use serde_yaml_ok as serde_yaml;
use std::fs;
use std::path::PathBuf;

/// Get config directory path
///
/// Returns different config directories based on operating system:
/// - Windows: `%APPDATA%\hostctl`
/// - Linux/macOS: `~/.config/hostctl`
#[cfg(target_os = "windows")]
fn get_config_dir() -> PathBuf {
    dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("C:\\ProgramData"))
        .join("hostctl")
}

/// Get config directory path
///
/// Returns different config directories based on operating system:
/// - Windows: `%APPDATA%\hostctl`
/// - Linux/macOS: `~/.config/hostctl`
#[cfg(any(target_os = "linux", target_os = "macos"))]
fn get_config_dir() -> PathBuf {
    dirs::home_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join(".config")
        .join("hostctl")
}

/// Configuration storage manager
///
/// Responsible for reading, writing, and managing configuration files.
pub struct ConfigStorage;

impl ConfigStorage {
    /// Get the full path to the config file
    ///
    /// # Returns
    /// Returns the `PathBuf` of the config file
    #[must_use]
    pub fn get_config_path() -> PathBuf {
        get_config_dir().join("config.yaml")
    }

    /// Get the path to the config directory
    ///
    /// # Returns
    /// Returns the `PathBuf` of the config directory
    #[must_use]
    pub fn get_config_dir_path() -> PathBuf {
        get_config_dir()
    }

    /// Load configuration from file
    ///
    /// If the config file does not exist, returns a new empty configuration.
    ///
    /// # Errors
    /// Returns an error if the file exists but cannot be read or parsed.
    ///
    /// # Returns
    /// Returns the loaded configuration or a newly created empty configuration
    pub fn load_config() -> Result<Config> {
        let config_path = Self::get_config_path();

        if !config_path.exists() {
            return Ok(Config::new());
        }

        let content = fs::read_to_string(&config_path)
            .with_context(|| format!("Failed to read config file: {}", config_path.display()))?;

        let config: Config =
            serde_yaml::from_str(&content).with_context(|| "Failed to parse config file")?;

        Ok(config)
    }

    /// Save configuration to file
    ///
    /// If the config directory does not exist, it will be created automatically.
    ///
    /// # Arguments
    /// * `config` - The configuration to save
    ///
    /// # Errors
    /// Returns an error if the directory cannot be created or the file cannot be written.
    pub fn save_config(config: &Config) -> Result<()> {
        let config_dir = get_config_dir();
        let config_path = Self::get_config_path();

        // Create config directory (if it doesn't exist)
        fs::create_dir_all(&config_dir).with_context(|| {
            format!(
                "Failed to create config directory: {}",
                config_dir.display()
            )
        })?;

        let content =
            serde_yaml::to_string(config).with_context(|| "Failed to serialize config")?;

        fs::write(&config_path, content)
            .with_context(|| format!("Failed to write config file: {}", config_path.display()))?;

        Ok(())
    }

    /// Ensure config directory exists
    ///
    /// If the config directory does not exist, it will be created automatically.
    ///
    /// # Errors
    /// Returns an error if the directory cannot be created.
    pub fn ensure_config_dir() -> Result<()> {
        let config_dir = get_config_dir();
        if !config_dir.exists() {
            fs::create_dir_all(&config_dir).with_context(|| {
                format!(
                    "Failed to create config directory: {}",
                    config_dir.display()
                )
            })?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{Environment, HostEntry};
    use std::net::Ipv4Addr;

    #[test]
    fn test_config_dir_paths() {
        let config_dir = get_config_dir();
        let config_path = ConfigStorage::get_config_path();

        // Verify path structure is correct
        assert!(config_path.ends_with("config.yaml"));
        assert_eq!(config_path.parent().unwrap(), config_dir);
    }

    #[test]
    fn test_save_and_load_config() {
        // Create a test configuration
        let mut config = Config::new();
        let mut env = Environment::new("test".to_string());

        env.add_entry(HostEntry::new(
            std::net::IpAddr::V4(Ipv4Addr::new(10, 0, 0, 1)),
            "api.test".to_string(),
        ));

        config.add_environment(env);
        config.current_environment = Some("test".to_string());

        // Test serialization and deserialization logic
        let serialized = serde_yaml::to_string(&config).unwrap();
        let deserialized: Config = serde_yaml::from_str(&serialized).unwrap();

        // Verify data integrity
        assert_eq!(deserialized.current_environment, Some("test".to_string()));
        assert!(deserialized.get_environment("test").is_some());
        assert_eq!(
            deserialized.get_environment("test").unwrap().entries.len(),
            1
        );
    }

    #[test]
    fn test_empty_config_creation() {
        // Test creation and loading of empty configuration
        let config = Config::new();

        assert!(config.environments.is_empty());
        assert_eq!(config.current_environment, None);

        // Test serialization of empty configuration
        let serialized = serde_yaml::to_string(&config).unwrap();
        let deserialized: Config = serde_yaml::from_str(&serialized).unwrap();

        assert!(deserialized.environments.is_empty());
        assert_eq!(deserialized.current_environment, None);
    }

    #[test]
    fn test_config_with_multiple_environments() {
        let mut config = Config::new();

        // Add multiple environments
        config.add_environment(Environment::new("dev".to_string()));
        config.add_environment(Environment::new("staging".to_string()));
        config.add_environment(Environment::new("prod".to_string()));

        config.current_environment = Some("staging".to_string());

        // Serialization and deserialization
        let serialized = serde_yaml::to_string(&config).unwrap();
        let deserialized: Config = serde_yaml::from_str(&serialized).unwrap();

        // Verify environment count
        assert_eq!(deserialized.environments.len(), 3);
        assert_eq!(
            deserialized.current_environment,
            Some("staging".to_string())
        );

        // Verify each environment exists
        assert!(deserialized.get_environment("dev").is_some());
        assert!(deserialized.get_environment("staging").is_some());
        assert!(deserialized.get_environment("prod").is_some());
    }

    #[test]
    fn test_config_environment_operations() {
        let mut config = Config::new();

        // Add environment
        config.add_environment(Environment::new("qa".to_string()));
        assert!(config.get_environment("qa").is_some());

        // Get mutable reference and modify
        if let Some(env) = config.get_environment_mut("qa") {
            env.description = Some("Quality Assurance".to_string());
        }

        assert_eq!(
            config.get_environment("qa").unwrap().description,
            Some("Quality Assurance".to_string())
        );

        // Remove environment
        assert!(config.remove_environment("qa"));
        assert!(config.get_environment("qa").is_none());
    }

    #[test]
    fn test_yaml_serialization_format() {
        let mut config = Config::new();
        let mut env =
            Environment::new("demo".to_string()).with_description("Demo environment".to_string());

        env.add_entry(
            HostEntry::new(
                std::net::IpAddr::V4(Ipv4Addr::new(192, 168, 1, 100)),
                "app.demo".to_string(),
            )
            .with_comment("Application server".to_string()),
        );

        config.add_environment(env);

        let yaml = serde_yaml::to_string(&config).unwrap();

        // Verify YAML format contains expected fields
        assert!(yaml.contains("current_environment"));
        assert!(yaml.contains("environments"));
        assert!(yaml.contains("demo"));
        assert!(yaml.contains("Demo environment"));
        assert!(yaml.contains("192.168.1.100"));
        assert!(yaml.contains("app.demo"));
        assert!(yaml.contains("Application server"));
    }
}
