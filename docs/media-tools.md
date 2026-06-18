# Media tools for the breakdown engine

Regex shells out to three sidecar binaries via Tauri `externalBin`. They are **not**
committed to the repo. Place them in `src-tauri/binaries/` using Tauri's target-triple
naming.

## macOS (Apple Silicon)

```text
src-tauri/binaries/
  yt-dlp-aarch64-apple-darwin
  ffmpeg-aarch64-apple-darwin
  whisper-aarch64-apple-darwin
```

## macOS (Intel)

```text
  yt-dlp-x86_64-apple-darwin
  ffmpeg-x86_64-apple-darwin
  whisper-x86_64-apple-darwin
```

Tauri picks the correct binary for the machine at build time. If you ship a universal app,
include **both** triples.

## Obtaining binaries

### yt-dlp

```bash
curl -L https://github.com/yt-dlp/yt-dlp/releases/latest/download/yt-dlp_macos -o src-tauri/binaries/yt-dlp-aarch64-apple-darwin
chmod +x src-tauri/binaries/yt-dlp-aarch64-apple-darwin
```

Use `yt-dlp_macos_legacy` for Intel Macs.

### ffmpeg

Download a static build from [evermeet.cx/ffmpeg](https://evermeet.cx/ffmpeg/) or
[ffmpeg.org](https://ffmpeg.org/download.html), rename to `ffmpeg-<triple>`, and mark
executable.

### whisper.cpp

Build from [ggml-org/whisper.cpp](https://github.com/ggml-org/whisper.cpp):

```bash
git clone https://github.com/ggml-org/whisper.cpp
cd whisper.cpp
make
cp build/bin/whisper-cli ../project-regex/src-tauri/binaries/whisper-aarch64-apple-darwin
chmod +x ../project-regex/src-tauri/binaries/whisper-aarch64-apple-darwin
```

### Whisper model

Download a model into app data (not bundled):

```bash
mkdir -p ~/Library/Application\ Support/app.regex.desktop/models
curl -L https://huggingface.co/ggerganov/whisper.cpp/resolve/main/ggml-base.en.bin \
  -o ~/Library/Application\ Support/app.regex.desktop/models/ggml-base.en.bin
```

## Development stubs

Stub scripts ship for Apple Silicon dev builds so `tauri dev` compiles without real
binaries. Replace them before running a real breakdown.

## Personal use

Only process content you have the right to use for personal reference. Regex does not
bypass platform protections or DRM. If a source cannot be fetched permissibly, the
breakdown fails with a clear message.

## Spike: cross-architecture bundling

- **Apple Silicon:** Tauri resolves `*-aarch64-apple-darwin` sidecars correctly when the
  file is present and executable.
- **Intel:** Requires the matching `x86_64-apple-darwin` filenames; same pipeline code.
- **Validation:** Run `npm run tauri dev`, then **Projects → Run breakdown** with real
  binaries installed. Check `check_media_tools` in Settings (coming) or the breakdown
  panel for availability.
