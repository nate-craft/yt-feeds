# Configuration
___

## YT-Feeds Files

Basic configuration is done in `yt-feeds.toml` located in the following locations:
 
**Linux**: `~/.config/yt-feeds/config.toml`  
**MacOS**: `~/Library/Application Support/yt-feeds/config.toml`  
**Windows**: `YOUR_DRIVE:\Users\YOUR_USER\AppData\Local\yt-feeds\config.toml`  
  
Saved data and error logs can be found in the following location:

**Linux**: `~/.local/share/yt-feeds/`  
**MacOS**: `~/Library/Application Support/yt-feeds/`  
**Windows**: `YOUR_DRIVE:\Users\YOUR_USER\AppData\Local\yt-feeds\`  

___

## MPV

Much of the customization can be performed in mpv's configuration like my own:

```txt
fullscreen=yes
screenshot-directory=~/Pictures/mpv/
slang=en
force-seekable=yes

# streaming
ytdl-format="bv*[height<=720]+ba/best"
script-opts=ytdl_hook-ytdl_path=yt-dlp

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

### Modifying MPV Display

You can modify how the MPV video viewer looks like either with the above configuration file, or with lua plugins.
I recommend these:
- [UOSC](https://github.com/tomasklaen/uosc) for a modern interface
- [Thumbfast](https://github.com/po5/thumbfast) for inline video thumbnails
- [Sponsorblock](https://github.com/po5/mpv_sponsorblock) to skip video sponsors

___

## yt-dlp

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

