# Installing Dory

## Linux

### Tarball (recommended)

```bash
# Install to /usr/local (requires sudo)
curl -fsSL https://raw.githubusercontent.com/vbasky/dory/main/scripts/install.sh | sudo bash

# Install to ~/.local (no sudo required)
curl -fsSL https://raw.githubusercontent.com/vbasky/dory/main/scripts/install.sh | bash -s -- --prefix ~/.local
```

### AppImage (portable)

```bash
# Download from releases (replace amd64 with arm64 for ARM)
wget https://github.com/vbasky/dory/releases/latest/download/dory-linux-amd64.AppImage
chmod +x dory-linux-amd64.AppImage
./dory-linux-amd64.AppImage
```

### Arch Linux

Available in the AUR:

```bash
# Using an AUR helper
paru -S dory
# or
yay -S dory
```

### Debian / Ubuntu

Download the `.deb` package from [Releases](https://github.com/vbasky/dory/releases):

```bash
# Replace amd64 with arm64 for ARM
wget https://github.com/vbasky/dory/releases/latest/download/dory-linux-amd64.deb
sudo dpkg -i dory-linux-amd64.deb
```

### Fedora / RHEL / CentOS

Download the `.rpm` package from [Releases](https://github.com/vbasky/dory/releases):

```bash
# Replace amd64 with arm64 for ARM
sudo dnf install https://github.com/vbasky/dory/releases/latest/download/dory-linux-amd64.rpm
```

### Nix

Using flakes (the default package is a **prebuilt binary** for Linux x86_64 / aarch64, no compilation):

```bash
# Run directly (prebuilt)
nix run github:vbasky/dory

# Install to profile (prebuilt)
nix profile install github:vbasky/dory

# Development shell
nix develop github:vbasky/dory
```

Build from source instead of using the prebuilt binary:

```bash
nix run    github:vbasky/dory#dory-source
nix build  github:vbasky/dory#dory-source
```

Nightly builds track `main` and install side by side with stable (distinct app id, icon, and `dory-nightly.db` database). Consume them from the `nightly` ref:

```bash
nix run github:vbasky/dory/nightly#dory-nightly
nix profile install github:vbasky/dory/nightly#dory-nightly
```

See [docs/RELEASE.md](RELEASE.md) for the channel model.

NixOS / nix-darwin via overlay:

```nix
{
  inputs.dory.url = "github:vbasky/dory";

  outputs = { nixpkgs, dory, ... }: {
    nixosConfigurations.myhost = nixpkgs.lib.nixosSystem {
      modules = [
        ({ pkgs, ... }: {
          nixpkgs.overlays = [ dory.overlays.default ];
          environment.systemPackages = [
            pkgs.dory         # prebuilt binary, no local compile
            # pkgs.dory-source  # alternative: build from source
          ];
        })
      ];
    };
  };
}
```

## macOS

Dory for macOS is not signed with an Apple developer certificate. When opening for the first time, you'll see a warning about an "unidentified developer".

### Installation

1. Download the DMG for your architecture from [Releases](https://github.com/vbasky/dory/releases):
   - **Intel Macs**: `dory-macos-amd64.dmg`
   - **Apple Silicon (M1/M2/M3/M4)**: `dory-macos-arm64.dmg`
2. Open the DMG and drag Dory to Applications
3. When you see the "unidentified developer" warning:
   - Go to **System Settings → Privacy & Security**
   - Click **Open Anyway** next to the security warning
   - Confirm you want to open the application

### Bypass Gatekeeper from Terminal

```bash
# Remove quarantine attribute (allows opening without GUI confirmation)
xattr -cr /Applications/Dory.app

# Now you can open it normally
open /Applications/Dory.app
```

### Requirements

- macOS 11.0 (Big Sur) or later

## Windows

### Installer

1. Download `dory-windows-amd64-setup.exe` from [Releases](https://github.com/vbasky/dory/releases)
2. Run the installer and follow the wizard

### Portable

1. Download `dory-windows-amd64.zip` from [Releases](https://github.com/vbasky/dory/releases)
2. Extract to any folder
3. Run `dory.exe`

> **Note**: The executable is not signed with a Windows code signing certificate. Windows SmartScreen may show a warning. Click "More info" → "Run anyway" to proceed.

### Requirements

- Windows 10 or later
- x86_64 (ARM64 not yet supported)

## Build from Source

```bash
# Via install script (Linux)
curl -fsSL https://raw.githubusercontent.com/vbasky/dory/main/scripts/install.sh | bash -s -- --build

# Or manually
git clone https://github.com/vbasky/dory.git
cd dory

# Recommended: build with the full default feature set
cargo build --release --features sqlite,postgres,mysql,mssql,mongodb,redis,dynamodb,cloudwatch,influxdb,clickhouse,lua,aws,mcp

# Minimal build (relational drivers only, no AI/MCP, no Lua)
cargo build --release --no-default-features --features sqlite,postgres,mysql

./target/release/dory
```

## Uninstall (Linux)

```bash
# If installed with install.sh
curl -fsSL https://raw.githubusercontent.com/vbasky/dory/main/scripts/uninstall.sh | sudo bash

# From ~/.local
curl -fsSL https://raw.githubusercontent.com/vbasky/dory/main/scripts/uninstall.sh | bash -s -- --prefix ~/.local

# Remove user config and data too
./scripts/uninstall.sh --remove-config
```

## Next steps

- [Usage Guide](USAGE.md) — first launch, creating a connection, and running your first query
- [Connecting — Advanced Setup](CONNECTIONS.md) — SSH tunnels, proxies, AWS SSO and value sources
