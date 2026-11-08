# Third-party licenses

The source code of this project is licensed under the MIT License. See
[`../LICENSE`](../LICENSE). That license applies **only** to this project's own
code. The third-party components below keep their own licenses.

## Bundled executables

| Component | Version | Source | License | License text |
| --- | --- | --- | --- | --- |
| `yt-dlp` | 2026.10.26 | <https://github.com/yt-dlp/yt-dlp/releases/tag/2026.10.26> | Unlicense (public domain) | [`yt-dlp-LICENSE.txt`](yt-dlp-LICENSE.txt) |
| `ffmpeg` (Linux) | 7.0.2 static | <https://johnvansickle.com/ffmpeg/> | GPL-3.0-or-later | [`ffmpeg-GPLv3.txt`](ffmpeg-GPLv3.txt) |
| `ffprobe` (Linux) | 7.0.2 static | <https://johnvansickle.com/ffmpeg/> | GPL-3.0-or-later | [`ffmpeg-GPLv3.txt`](ffmpeg-GPLv3.txt) |
| `ffmpeg` / `ffprobe` (Windows) | 7.0.2 full build | <https://github.com/GyanD/codexffmpeg/releases/tag/7.0.2> | GPL-3.0-or-later | [`ffmpeg-GPLv3.txt`](ffmpeg-GPLv3.txt) |

The exact build configuration of the bundled Linux binaries is recorded in
[`../yt-dlp-gui/src-tauri/binaries/README.md`](../yt-dlp-gui/src-tauri/binaries/README.md).
The Windows binaries are downloaded by
[`.github/workflows/release.yml`](../.github/workflows/release.yml) at release
time.

## How the application uses FFmpeg

The application runs `ffmpeg` and `ffprobe` as **separate processes**. It does
not link against FFmpeg libraries and does not include FFmpeg source code. The
FFmpeg binaries are aggregated with the application for distribution; they are
not relicensed under MIT, and the MIT license of this project does not extend to
them.

## Corresponding source for the GPLv3 FFmpeg builds

The bundled FFmpeg binaries are built with `--enable-gpl --enable-version3` and
are therefore licensed under the GNU General Public License, version 3 or later.
The GPL requires that recipients can obtain the corresponding source code for
the exact binaries they received.

- **Linux (FFmpeg 7.0.2):** the FFmpeg source is available from
  <https://ffmpeg.org/releases/ffmpeg-7.0.2.tar.xz>. The build configuration and
  static library list are printed by `ffmpeg -buildconf` and are recorded in
  the sidecar README. The build scripts are published by
  <https://johnvansickle.com/ffmpeg/>.
- **Windows (FFmpeg 7.0.2 full build):** the build and its source are published
  by <https://github.com/GyanD/codexffmpeg/releases/tag/7.0.2>.
- **Statically linked libraries:** these builds link libraries such as x264,
  x265, libvpx, libaom, libdav1d, libopus, libvorbis, and others listed by
  `ffmpeg -buildconf`. Their source is available from their respective upstream
  projects.

The formal written offer, including the source assets attached to each release,
is in [`SOURCE-OFFER.md`](SOURCE-OFFER.md).

## Rust and JavaScript dependencies

The application is built with Tauri 2, React, and their dependency trees. All
Rust crates used by the build are licensed under permissive licenses (MIT,
Apache-2.0, BSD, ISC, Zlib, Unicode, BSL-1.0, CC0, or CDLA-Permissive-2.0) or
weak-copyleft MPL-2.0. No GPL- or LGPL-only crate is linked into the
application. The MPL-2.0 crates (`cssparser`, `selectors`, `dtoa-short`,
`option-ext`) are used unmodified and their source is available from
crates.io and in the published crate sources.

The JavaScript dependencies are MIT, Apache-2.0, ISC, BSD-3-Clause, or
MPL-2.0 (the MPL-2.0 packages, `lightningcss`, are build-time tooling and are
not shipped in the application bundle). Tauri's MIT license text is included as
[`tauri-MIT.txt`](tauri-MIT.txt).

Tauri is also available under Apache-2.0; the MIT option is the one reproduced
here.
