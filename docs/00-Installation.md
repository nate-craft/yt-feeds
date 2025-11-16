# Installation

___

## Cargo

Precompiled binaries can be found on the [AUR](https://aur.archlinux.org)
```bash
paru -Syu yt-feeds
```

For cross-platform support, use [cargo-binstall](https://github.com/cargo-bins/cargo-binstall):
```bash
cargo binstall yt-feeds
```

Or, by source with [cargo](https://github.com/cargo-bins/cargo-binstall):
```bash
cargo install --git https://github.com/nate-craft/yt-feeds
```

YT-Feeds requires mpv and yt-dlp to be installed in order to function:
- [mpv](https://github.com/mpv-player/mpv)
- [yt-dlp](https://github.com/yt-dlp/yt-dlp)

<details><summary>Linux Runtime Dependencies</summary>

### Fedora/RHEL-Based Systems
```sh
sudo dnf install yt-dlp mpv
```

### Debian-Based Systems
```sh
sudo apt install yt-dlp mpv
```

### Arch-Based Systems
```sh
sudo pacman -Syu yt-dlp mpv
```

### Alpine-Based Systems
```sh
doas apk add yt-dlp mpv
```

</details>

<details><summary>MacOS Runtime Dependencies</summary>

### MacOS
```
# Brew can be installed at https://brew.sh/
brew install yt-dlp mpv
```

</details>
<details><summary>Windows Runtime Dependencies</summary>

### Windows
```
# WSL can be installed at https://learn.microsoft.com/en-us/windows/wsl/install
# This is recommended and will allow you to follow Linux instructions with support

# OR

# Chocolatey can be installed at https://chocolatey.org/install
# Note: untested
choco install yt-dlp mpv

# OR

# Winget can be installed at https://github.com/microsoft/winget-cli
# Note: untested
winget install yt-dlp mpv
```

</details>

