# Third-party licenses

The source code of this project is licensed under the MIT License. See
[`../LICENSE`](../LICENSE). That license applies **only** to this project's own
code. The third-party components below keep their own licenses.

## Bundled executable

| Component | Version | Source | License | License text |
| --- | --- | --- | --- | --- |
| `yt-dlp` | 2026.10.26 | <https://github.com/yt-dlp/yt-dlp/releases/tag/2026.10.26> | Unlicense (public domain) | [`yt-dlp-LICENSE.txt`](yt-dlp-LICENSE.txt) |

The exact build information for the bundled Linux and Windows binaries is in
[`../yt-dlp-gui/src-tauri/binaries/README.md`](../yt-dlp-gui/src-tauri/binaries/README.md).

## FFmpeg is not bundled

FFmpeg is a **runtime dependency**, not part of the application bundle. The
application runs `ffmpeg` and `ffprobe` as separate processes. It does not link
against FFmpeg libraries and does not include FFmpeg source code or binaries in
its installers.

Because the project does not distribute FFmpeg, it has no obligation to provide
the corresponding source for FFmpeg builds. The FFmpeg tab offers to:

- detect an existing `ffmpeg` binary (`FFMPEG_PATH`, a saved path, `PATH`, or a
  common install location),
- install FFmpeg through the platform package manager,
- open the official download page,
- or use a binary selected by the user.

FFmpeg builds have different licenses depending on how they were compiled. Most
prebuilt "full" or "GPL" builds are GPL-3.0-or-later; LGPL builds are also
common. The license of the build on the user's system is between the user and
the build's provider. A reference copy of the GPLv3 text is included as
[`ffmpeg-GPLv3.txt`](ffmpeg-GPLv3.txt) because many builds use it, but this
project does not distribute those builds.

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
