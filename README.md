# hevc-datamosh

## usage

It's really not designed for anyone to use it except for me right now, but my basic workflow looks like:

```
cd hevc-datamosh
ffmpeg -i /wheverever/your_video.mkv -s hd720 -pix_fmt yuv420p -f rawvideo - | x265 --input-res 1280x720 --fps 29.97 --input - --output in.hevc
cargo run --release && mpv out.hevc
```

