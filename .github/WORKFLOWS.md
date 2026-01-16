# GitHub Actions Workflows

This repository uses GitHub Actions for continuous integration and automated releases.

## Workflows

### 1. CI Workflow (`.github/workflows/ci.yml`)

Runs on every push and pull request to `main` or `master` branches.

**Jobs:**
- **Test Suite**: Runs formatting checks, clippy, builds the project, and runs all tests
- **Build Cross-Platform**: Builds release binaries for multiple platforms
- **Security Audit**: Runs `cargo audit` to check for security vulnerabilities
- **Check Documentation**: Builds and validates documentation

**Supported Platforms:**
- Linux x86_64
- macOS x86_64 (Intel)
- macOS aarch64 (Apple Silicon)
- Windows x86_64
- Linux aarch64 (ARM64)

### 2. Release Workflow (`.github/workflows/release.yml`)

Automatically triggered when a tag matching `v*` is pushed.

**Jobs:**
- **Create Release**: Creates a GitHub release with release notes
- **Build and Upload**: Builds release binaries for all platforms and uploads them
- **Create Checksums**: Generates and uploads a combined checksums file

**Generated Assets:**
- `hostctl-linux-x86_64.tar.gz` + SHA256 checksum
- `hostctl-macos-x86_64.tar.gz` + SHA256 checksum
- `hostctl-macos-aarch64.tar.gz` + SHA256 checksum
- `hostctl-windows-x86_64.zip` + SHA256 checksum
- `checksums.txt` (combined checksums for all platforms)

### 3. Publish Workflow (`.github/workflows/publish.yml`)

Manually triggered workflow to create a new release.

**Usage:**
1. Go to Actions tab in GitHub
2. Select "Publish Release" workflow
3. Click "Run workflow"
4. Enter version (e.g., `v1.0.0`)
5. Choose if it's a prerelease or draft
6. Click "Run workflow"

This will:
- Create a Git tag
- Push the tag to the repository
- Trigger the Release workflow automatically

## Creating a Release

### Option 1: Using Publish Workflow (Recommended)

1. Go to Actions → Publish Release
2. Run workflow with version `v1.0.0`
3. Wait for the Release workflow to complete
4. Check the Releases page for the new release

### Option 2: Manual Tag Push

```bash
# Create and push a tag
git tag -a v1.0.0 -m "Release v1.0.0"
git push origin v1.0.0
```

The Release workflow will automatically trigger and build all binaries.

### Option 3: Using GitHub UI

1. Go to Releases page
2. Click "Create a new release"
3. Choose a tag (or create a new one)
4. Fill in release title and description
5. Click "Publish release"

## Release Assets

Each release includes:

### Binaries
- **Linux**: `hostctl-linux-x86_64.tar.gz` - Precompiled binary for Linux x86_64
- **macOS Intel**: `hostctl-macos-x86_64.tar.gz` - Precompiled binary for macOS Intel
- **macOS Apple Silicon**: `hostctl-macos-aarch64.tar.gz` - Precompiled binary for macOS Apple Silicon
- **Windows**: `hostctl-windows-x86_64.zip` - Precompiled binary for Windows

### Checksums
- Individual SHA256 checksums for each binary
- Combined `checksums.txt` file for easy verification

### Installation Instructions
Each release includes platform-specific installation instructions in the release notes.

## Verification

After downloading a binary, verify its integrity:

```bash
# Download the checksums file
wget https://github.com/yourusername/hostctl/releases/download/v1.0.0/checksums.txt

# Verify the downloaded binary
sha256sum -c checksums.txt
```

## Troubleshooting

### Build Failures

If a build fails:
1. Check the Actions tab for detailed logs
2. Ensure all dependencies are up to date
3. Verify the code compiles locally with `cargo build --release`

### Release Not Triggering

If the Release workflow doesn't trigger after pushing a tag:
1. Verify the tag format matches `v*` (e.g., `v1.0.0`)
2. Check that the tag was pushed to the repository
3. Ensure the workflow file is in `.github/workflows/`

### Cross-Compilation Issues

If cross-compilation fails for ARM64:
1. The workflow uses `cross` for cross-compilation
2. Check the cross-compilation logs for specific errors
3. Some dependencies may not support all targets

## Security

- All workflows use the latest versions of actions
- Secrets are managed through GitHub Secrets
- Dependencies are audited for security vulnerabilities
- Binaries are stripped to reduce size and remove debug symbols

## Maintenance

To update the workflows:
1. Edit the workflow files in `.github/workflows/`
2. Commit and push changes
3. Test the workflows by pushing to a branch or creating a PR

To add new platforms:
1. Add the platform to the matrix in `ci.yml` and `release.yml`
2. Update the documentation accordingly
3. Test the build locally if possible
