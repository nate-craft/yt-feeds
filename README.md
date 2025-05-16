# YT-Feeds

YT-Feeds is a simple, fast, and distraction free CLI application to view videos from your favorite channels.

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
brew install yt-dlp mpv
```

</details>
<details><summary>Windows Runtime Dependencies</summary>

#### Windows
```
# Chocolatey
choco install yt-dlp mpv

# Winget
winget install yt-dlp mpv
```

</details>

## Customization

### MPV

Much of the customization can be performed in mpv's configuration like my own:

```txt
fullscreen=yes
screenshot-directory=~/Pictures/mpv/
slang=en
script-opts=ytdl_hook-ytdl_path=yt-dlp
ytdl-raw-options=cookies-from-browser=firefox
osd-bar=no
border=no
hwdec=vaapi
gpu-api=auto
save-position-on-quit
write-filename-in-watch-later-config
display-tags-clr
msg-level=all=no
```

This can be found in the following locations:  
Linux: `~/.config/mpv/mpv.conf`  
MacOS: `~/Library/Application Support/mpv/mpv.conf`  
Windows: `YOUR_DRIVE:\Users\YOUR_USER\AppData\Roaming\mpv\mpv.conf`

> Note: for watch history to function, both `save-position-on-quit` and `write-filename-in-watch-later-config` must be in your `mpv.conf`

### Modifying MPV Display

You can modify how the MPV video viewer looks like either with the above configuration file, or with lua plugins.
I recommend these:
- [UOSC](https://github.com/tomasklaen/uosc) for a modern interface
- [Thumbfast](https://github.com/po5/thumbfast) for inline video thumbnails


