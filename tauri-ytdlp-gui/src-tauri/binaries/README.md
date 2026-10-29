# Bundled sidecars

These binaries are committed so `cargo build`, `cargo test`, and `tauri build`
work offline. Tauri copies each file next to the app executable during a build
and into the platform bundle.

| Sidecar | Version | Source | License |
| --- | --- | --- | --- |
| `yt-dlp-sidecar` | 2026.10.26 | <https://github.com/yt-dlp/yt-dlp/releases/tag/2026.10.26> (`yt-dlp_linux`) | Unlicense, see `yt-dlp-LICENSE.txt` |
| `ffmpeg-sidecar` | 6.1.1 | <https://github.com/wader/static-ffmpeg> (`mwader/static-ffmpeg:6.1.1`) | GPLv3, see `ffmpeg-GPLv3.txt` |
| `ffprobe-sidecar` | 6.1.1 | <https://github.com/wader/static-ffmpeg> (`mwader/static-ffmpeg:6.1.1`) | GPLv3, see `ffmpeg-GPLv3.txt` |

The FFmpeg/ffprobe binaries are static musl builds from the `mwader/static-ffmpeg`
image. The FFmpeg source for version 6.1.1 is available at
<https://ffmpeg.org/releases/ffmpeg-6.1.1.tar.xz>.

The target-triple suffix is required by Tauri's `bundle.externalBin` setting:

- `yt-dlp-sidecar-x86_64-unknown-linux-gnu`
- `ffmpeg-sidecar-x86_64-unknown-linux-gnu`
- `ffprobe-sidecar-x86_64-unknown-linux-gnu`

The sidecars are deliberately named `*-sidecar` instead of `yt-dlp`, `ffmpeg`,
and `ffprobe`. Tauri installs external binaries into `/usr/bin` for `.deb`
packages, so the plain names would conflict with the distribution packages.

SHA-256 checksums:

```text
767ba9dce7052aa196f8b12652f433d76de128ba168966fa84b6394d5e6c6795  yt-dlp-sidecar-x86_64-unknown-linux-gnu
dcf4c806caad507b11c73754b86a8cbd0ce686ae12f90bf8ba235d145f53edc0  ffmpeg-sidecar-x86_64-unknown-linux-gnu
aef7126f9c9845cdb8692c1dfd3c9eb74016210a68f8534325cf9f9a631cf9f1  ffprobe-sidecar-x86_64-unknown-linux-gnu
```

## Updating

Replace the target-triple-suffixed files with the matching release assets and
update the versions and checksums above. For Windows, use the
`x86_64-pc-windows-msvc` suffix and the `.exe` assets. For macOS, use the
`x86_64-apple-darwin` or `aarch64-apple-darwin` suffix.
