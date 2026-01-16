# Hostctl - Hosts File Management Tool

A command-line tool for managing hosts file with environment support, written in
Rust.

## Features

- ✅ **Hosts Entry Management**: Add, remove, and list host entries
- ✅ **Environment Support**: Create custom environments (e.g., development,
  testing, production)
- ✅ **Quick Switching**: Switch between environments with a single command
- ✅ **Cross-Platform**: Supports Windows, macOS, and Linux
- ✅ **Backup Functionality**: Backup current hosts file content
- ✅ **Configuration Persistence**: Automatically saves configurations

## Installation

### Pre-built Binaries (Recommended)

Download pre-built binaries from the [Releases](https://github.com/antediluvian/hostctl/releases) page.

#### Linux
```bash
# Download and extract
wget https://github.com/antediluvian/hostctl/releases/latest/download/hostctl-linux-x86_64.tar.gz
tar -xzf hostctl-linux-x86_64.tar.gz

# Install to /usr/local/bin
sudo mv hostctl /usr/local/bin/

# Verify installation
hostctl --version
```

#### macOS
```bash
# Intel Macs
curl -L https://github.com/antediluvian/hostctl/releases/latest/download/hostctl-macos-x86_64.tar.gz -o hostctl.tar.gz

# Apple Silicon Macs
curl -L https://github.com/antediluvian/hostctl/releases/latest/download/hostctl-macos-aarch64.tar.gz -o hostctl.tar.gz

# Extract and install
tar -xzf hostctl.tar.gz
sudo mv hostctl /usr/local/bin/

# Verify installation
hostctl --version
```

#### Windows
```powershell
# Download using PowerShell
Invoke-WebRequest -Uri "https://github.com/antediluvian/hostctl/releases/latest/download/hostctl-windows-x86_64.zip" -OutFile "hostctl.zip"

# Extract
Expand-Archive -Path hostctl.zip -DestinationPath .

# Add to PATH (optional)
# Move hostctl.exe to a directory in your PATH, e.g., C:\Program Files\hostctl\

# Verify installation
.\hostctl.exe --version
```

#### Verify Checksums
```bash
# Download checksums
wget https://github.com/antediluvian/hostctl/releases/latest/download/checksums.txt

# Verify
sha256sum -c checksums.txt
```

### Build from Source

#### Prerequisites

1. Install Rust toolchain:
   ```bash
   # On macOS/Linux
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

   # On Windows
   # Download and install from https://rustup.rs/
   ```

2. Restart your terminal or run:
   ```bash
   source $HOME/.cargo/env
   ```

#### Build
```bash
# Clone the repository
git clone https://github.com/antediluvian/hostctl.git
cd hostctl

# Build the project
cargo build --release

# Install globally (optional)
cargo install --path .
```

### Using Cargo

```bash
# Install directly from crates.io
cargo install hostctl

# Or install from GitHub repository
cargo install --git https://github.com/antediluvian/hostctl.git
```

## Usage

### Basic Commands

```bash
# List all environments
hostctl list

# Show current environment
hostctl current

# Create a new environment
hostctl add development --description "Development environment"

# Switch to an environment
hostctl switch development

# Add a host entry to an environment
hostctl add-entry development 127.0.0.1 api.local --comment "Local API server"

# Remove a host entry from an environment
hostctl remove-entry development api.local

# Show details of an environment
hostctl show development

# Remove an environment
hostctl remove development
```

### Example Workflow

1. **Create environments for different scenarios:**
   ```bash
   hostctl add dev --description "Development environment"
   hostctl add test --description "Testing environment"
   hostctl add prod --description "Production environment"
   ```

2. **Add entries to development environment:**
   ```bash
   hostctl add-entry dev 127.0.0.1 api.dev.local
   hostctl add-entry dev 127.0.0.1 frontend.dev.local
   hostctl add-entry dev 192.168.1.100 database.dev.local
   ```

3. **Switch to testing environment:**
   ```bash
   hostctl switch test
   hostctl add-entry test 192.168.1.200 api.test.local
   hostctl add-entry test 192.168.1.200 frontend.test.local
   ```

4. **Quickly switch between environments:**
   ```bash
   hostctl switch dev    # Switch to development
   hostctl switch test   # Switch to testing
   ```

## Configuration

The tool stores configuration in platform-specific locations:

- **Windows**: `%APPDATA%\hostctl\config.yaml`
- **macOS/Linux**: `~/.config/hostctl/config.yaml`

## Security Notes

⚠️ **Important**: This tool modifies system hosts files, which requires
administrative privileges on most systems.

- On macOS/Linux, you may need to run commands with `sudo`
- On Windows, run the command prompt as Administrator

## File Locations

### Hosts Files

- **Windows**: `C:\Windows\System32\drivers\etc\hosts`
- **macOS**: `/etc/hosts`
- **Linux**: `/etc/hosts`

### Configuration Files

- **Windows**: `%APPDATA%\hostctl\config.yaml`
- **macOS/Linux**: `~/.config/hostctl/config.yaml`

## Troubleshooting

### Permission Issues

If you encounter permission errors when switching environments:

**macOS/Linux:**

```bash
sudo hostctl switch development
```

**Windows:**
Run Command Prompt as Administrator

### Environment Not Found

Ensure the environment exists by listing all environments:

```bash
hostctl list
```

### Configuration Corruption

If the configuration becomes corrupted, you can reset it by deleting the config
file:

```bash
# Find config location
# macOS/Linux: ~/.config/hostctl/config.yaml
# Windows: C:\ProgramData\hostctl\config.yaml
# Then manually delete the config.yaml file
```

## Development

### Project Structure

```
src/
├── main.rs      # CLI interface and command handling
├── config.rs    # Data structures for environments and host entries
├── hosts.rs     # Hosts file operations
└── storage.rs   # Configuration persistence
```

### Building for Release

```bash
cargo build --release
```

The binary will be available at `target/release/hostctl` (or
`target/release/hostctl.exe` on Windows).

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file
for details.

### Summary of MIT License

- **Freedom to use**: You can use this software for any purpose
- **Freedom to modify**: You can modify the source code
- **Freedom to distribute**: You can distribute original or modified versions
- **Attribution required**: You must include the original copyright notice
- **No warranty**: The software is provided "as is" without warranty

For the full license text, please refer to the [LICENSE](LICENSE) file in the
root directory of this project.
