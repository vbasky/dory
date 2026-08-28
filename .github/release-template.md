## Installation

### Linux

### Tarball (recommended)
```bash
# Download and extract (replace amd64 with arm64 for ARM)
wget https://github.com/__REPO__/releases/download/__VERSION__/dory-linux-amd64.tar.gz
tar -xzf dory-linux-amd64.tar.gz

# Run installer
sudo ./scripts/install.sh
```

### AppImage (portable)
```bash
# Download
wget https://github.com/__REPO__/releases/download/__VERSION__/dory-linux-amd64.AppImage
chmod +x dory-linux-amd64.AppImage
./dory-linux-amd64.AppImage
```

### macOS

Dory for macOS is not signed with an Apple developer certificate. When opening for the first time:

1. Download the DMG for your architecture:
   - **Intel Macs**: `dory-macos-amd64.dmg`
   - **Apple Silicon (M1/M2/M3)**: `dory-macos-arm64.dmg`
2. Open the DMG and drag Dory to Applications
3. When you see "unidentified developer", go to **System Preferences → Privacy & Security**
4. Click **Open Anyway** next to the warning
5. Confirm you want to open the application

Alternatively, from Terminal:
```bash
xattr -cr /Applications/Dory.app
```

### Windows

#### Installer
1. Download `dory-windows-amd64-setup.exe`
2. Run the installer and follow the wizard

#### Portable
1. Download `dory-windows-amd64.zip`
2. Extract and run `dory.exe`

> Note: The executable is not signed. Windows SmartScreen may show a warning. Click "More info" → "Run anyway".

---

## Verify Downloads

All release artifacts are signed with GPG key `A614B7D25134987A`.

### Import the public key (one time)

```bash
gpg --keyserver keyserver.ubuntu.com --recv-keys A614B7D25134987A
```

### Verify a detached GPG signature

```bash
# Linux tarball
gpg --verify dory-linux-amd64.tar.gz.asc dory-linux-amd64.tar.gz

# Linux AppImage
gpg --verify dory-linux-amd64.AppImage.asc dory-linux-amd64.AppImage

# macOS DMG
gpg --verify dory-macos-arm64.dmg.asc dory-macos-arm64.dmg

# Windows ZIP
gpg --verify dory-windows-amd64.zip.asc dory-windows-amd64.zip

# Windows installer
gpg --verify dory-windows-amd64-setup.exe.asc dory-windows-amd64-setup.exe
```

### Verify a native .deb package

```bash
dpkg-sig --verify dory_*.deb
```

### Verify a native .rpm package

```bash
rpm --checksig dory-*.rpm
```

### Verify a SHA256 checksum

```bash
sha256sum -c dory-linux-amd64.tar.gz.sha256
```

## System Requirements

| Platform | Requirements |
|----------|-------------|
| Linux | x86_64 or ARM64, Vulkan-capable GPU (recommended) |
| macOS | macOS 11.0 (Big Sur) or later |
| Windows | Windows 10 or later, x86_64 |

---
