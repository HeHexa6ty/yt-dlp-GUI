# Bundled sidecars

These binaries are committed so `cargo build`, `cargo test`, and `tauri build`
work offline. Tauri copies each file next to the app executable during a build
and into the platform bundle.

| Sidecar | Version | Source | License |
| --- | --- | --- | --- |
| `yt-dlp-sidecar` | 2026.10.26 | <https://github.com/yt-dlp/yt-dlp/releases/tag/2026.10.26> (`yt-dlp_linux`) | Unlicense, see `yt-dlp-LICENSE.txt` |
| `ffmpeg-sidecar` | 7.0.2 | <https://johnvansickle.com/ffmpeg/> (`ffmpeg-release-amd64-static`) | GPL-3.0-or-later, see `ffmpeg-GPLv3.txt` |
| `ffprobe-sidecar` | 7.0.2 | <https://johnvansickle.com/ffmpeg/> (`ffmpeg-release-amd64-static`) | GPL-3.0-or-later, see `ffmpeg-GPLv3.txt` |

The FFmpeg/ffprobe builds use `--enable-gpl --enable-version3`, so the binaries
are GPLv3-or-later. See
[`../../../THIRD-PARTY-LICENSES/README.md`](../../../THIRD-PARTY-LICENSES/README.md)
for corresponding-source information. The MIT license of this project does not
apply to these binaries.

The FFmpeg/ffprobe binaries are statically linked, non-PIE (ET_EXEC) amd64
builds from johnvansickle.com. Keep them non-PIE: the AppImage bundler runs
`patchelf` on every ELF executable in the AppDir, and that corrupts static-PIE
binaries so they segfault at startup. The FFmpeg source for version 7.0.2 is
available at <https://ffmpeg.org/releases/ffmpeg-7.0.2.tar.xz>.

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
e7e7fb30477f717e6f55f9180a70386c62677ef8a4d4d1a5d948f4098aa3eb99  ffmpeg-sidecar-x86_64-unknown-linux-gnu
4f231a1960d83e403d08f7971e271707bec278a9ae18e21b8b5b03186668450d  ffprobe-sidecar-x86_64-unknown-linux-gnu
```

## Updating

Replace the target-triple-suffixed files with the matching release assets and
update the versions and checksums above. For Windows, use the
`x86_64-pc-windows-msvc` suffix and the `.exe` assets from the
`GyanD/codexffmpeg` release tag matching `FFMPEG_VERSION`
(<https://github.com/GyanD/codexffmpeg/releases>). For macOS, use the
`x86_64-apple-darwin` or `aarch64-apple-darwin` suffix.

The release workflow records `ffmpeg -version` and `ffmpeg -buildconf` for each
platform and attaches them, together with the FFmpeg source archive, to the
GitHub release. See
[`SOURCE-OFFER.md`](../../../THIRD-PARTY-LICENSES/SOURCE-OFFER.md).
