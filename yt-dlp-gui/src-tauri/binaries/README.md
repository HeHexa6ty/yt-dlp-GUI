# Bundled sidecars

These binaries are committed so `cargo build`, `cargo test`, and `tauri build`
work offline. Tauri copies each file next to the app executable during a build
and into the platform bundle.

| Sidecar | Version | Source | License |
| --- | --- | --- | --- |
| `yt-dlp-sidecar` | 2026.10.26 | <https://github.com/yt-dlp/yt-dlp/releases/tag/2026.10.26> (`yt-dlp_linux`) | Unlicense, see `yt-dlp-LICENSE.txt` |

FFmpeg is **not** bundled. The application looks for a system `ffmpeg` (or a
binary selected in the FFmpeg tab) and can install it through the platform
package manager. Keeping FFmpeg outside the bundle avoids distributing GPLv3
binaries and their corresponding source. See
[`../../../THIRD-PARTY-LICENSES/README.md`](../../../THIRD-PARTY-LICENSES/README.md).

The target-triple suffix is required by Tauri's `bundle.externalBin` setting:

- `yt-dlp-sidecar-x86_64-unknown-linux-gnu`
- `yt-dlp-sidecar-x86_64-pc-windows-msvc.exe`

The sidecar is deliberately named `*-sidecar` instead of `yt-dlp`. Tauri
installs external binaries into `/usr/bin` for `.deb` packages, so the plain
name would conflict with the distribution package.

SHA-256 checksum:

```text
767ba9dce7052aa196f8b12652f433d76de128ba168966fa84b6394d5e6c6795  yt-dlp-sidecar-x86_64-unknown-linux-gnu
```

## Updating

Replace the target-triple-suffixed file with the matching release asset and
update the version and checksum above. For Windows, use the
`x86_64-pc-windows-msvc` suffix and the `.exe` asset. For macOS, use the
`x86_64-apple-darwin` or `aarch64-apple-darwin` suffix.
