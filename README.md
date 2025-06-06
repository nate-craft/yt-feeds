# YT-Feeds

YT-Feeds is a cross-platform simple, fast, and distraction free CLI application to view videos from your favorite channels.

## Features

- 🔥 Blazingly fast - more than any web or electron-based system

- 💻 Uses minimal resources

- 👀 Distraction and short form content free!

- 🔍 Search for your favorite channels and subscribe/unsubscribe

- 📜 Automatically tracks and resumes watch history

- 🎥 Shows recent videos from subscriptions organized by date

- ＞ Never requires leaving the terminal or using your mouse  

## Installing

YT-Feeds can be installed via [cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html):
```bash
cargo install --git https://github.com/higgsbi/yt-feeds
```

YT-Feeds requires mpv and yt-dlp to be installed in order to function:
- [mpv](https://github.com/mpv-player/mpv)
- [yt-dlp](https://github.com/yt-dlp/yt-dlp)

<details><summary>Linux Runtime Dependencies</summary>

#### Fedora/RHEL-Based Systems
```sh
sudo dnf install yt-dlp mpv
```

#### Debian-Based Systems
```sh
sudo apt install yt-dlp mpv
```

#### Arch-Based Systems
```sh
sudo pacman -S yt-dlp mpv
```

#### Alpine-Based Systems
```sh
doas apk add yt-dlp mpv
```

</details>

<details><summary>MacOS Runtime Dependencies</summary>

#### MacOS
```
# Brew can be installed at https://brew.sh/
brew install yt-dlp mpv
```

</details>
<details><summary>Windows Runtime Dependencies</summary>

#### Windows
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

## Troubleshooting

Error logs can be found in the `log.txt` in the locations as specified in the customization section.

## Customization

Basic configuration is done in `yt-feeds.toml` located in the following locations:
 
**Linux**: `~/.config/yt-feeds/config.toml`  
**MacOS**: `~/Library/Application Support/yt-feeds/config.toml`  
**Windows**: `YOUR_DRIVE:\Users\YOUR_USER\AppData\Local\yt-feeds\config.toml`  
  
Saved data and caches can be found in the following location:

**Linux**: `~/.local/share/yt-feeds/`  
**MacOS**: `~/Library/Application Support/yt-feeds/`  
**Windows**: `YOUR_DRIVE:\Users\YOUR_USER\AppData\Local\yt-feeds\`  

### MPV

Much of the customization can be performed in mpv's configuration like my own:

```txt
fullscreen=yes
screenshot-directory=~/Pictures/mpv/
slang=en
force-seekable=yes

# streaming
ytdl-format="bv*[height<=720]+ba/best"
script-opts=ytdl_hook-ytdl_path=yt-dlp

# progress
save-position-on-quit
write-filename-in-watch-later-config

# apperance
msg-level=all=no
display-tags-clr
osd-bar=no
border=no

# efficiency
hwdec=auto
gpu-api=auto
profile=fast
cache=yes
demuxer-max-bytes=64MiB
demuxer-max-back-bytes=32MiB
video-sync=display-resample
vd-lavc-threads=8

```

This can be created in the following locations:

**Linux**: `~/.config/mpv/mpv.conf`  
**MacOS**: `~/.config/mpv/mpv.conf`  
**Windows**: `YOUR_DRIVE:\Users\YOUR_USER\AppData\Roaming\mpv\mpv.conf`

> Note: for watch history to function, both `save-position-on-quit` and `write-filename-in-watch-later-config` must be in your `mpv.conf`

### Modifying MPV Display

You can modify how the MPV video viewer looks like either with the above configuration file, or with lua plugins.
I recommend these:
- [UOSC](https://github.com/tomasklaen/uosc) for a modern interface
- [Thumbfast](https://github.com/po5/thumbfast) for inline video thumbnails
- [Sponsorblock](https://github.com/po5/mpv_sponsorblock) to skip video sponsors

### yt-dlp

Outside of the MPV configuration, we can also specify specific yt-dlp options.

```txt
# format
-f bv*[height<=720]+ba
--merge-output-format mkv

# metadata
--embed-chapters
--sponsorblock-mark all
--embed-metadata
--embed-thumbnail
--add-metadata
--embed-subs
--sub-lang en
--progress
--hls-use-mpegts

# efficiency
--no-check-certificate
--no-playlist
--geo-bypass
--youtube-skip-dash-manifest
--downloader aria2c -N 32
```

This can be found in the following locations:

**Linux**: `~/.config/mpv/mpv.conf`  
**MacOS**: `~/Library/Application Support/yt-dlp/config`  
**Windows**: `YOUR_DRIVE:\Users\YOUR_USER\AppData\Roaming\yt-dlp\config`

