# Corresponding source for the GPLv3 FFmpeg binaries

This project distributes `ffmpeg` and `ffprobe` binaries licensed under the GNU
General Public License, version 3 or later. This document is the written offer
of corresponding source required by the GPL.

## Binaries distributed

| Platform | Version | Provider | Build |
| --- | --- | --- | --- |
| Linux x86_64 | 7.0.2 | <https://johnvansickle.com/ffmpeg/> | static, `--enable-gpl --enable-version3` |
| Windows x86_64 | 7.0.2 | <https://github.com/GyanD/codexffmpeg> | `ffmpeg-7.0.2-full_build` (GPLv3) |

## FFmpeg source

The FFmpeg source for version 7.0.2 is available from:

- <https://ffmpeg.org/releases/ffmpeg-7.0.2.tar.xz>
- <https://ffmpeg.org/releases/ffmpeg-7.0.2.tar.xz.asc> (signature)

Releases built by the workflow in
[`.github/workflows/release.yml`](../.github/workflows/release.yml) attach the
following assets:

- `ffmpeg-7.0.2.tar.xz` and `ffmpeg-7.0.2.tar.xz.asc`
- `ffmpeg-buildconf-linux.txt` — `ffmpeg -version` and `ffmpeg -buildconf` output
  for the bundled Linux build
- `ffmpeg-buildconf-windows.txt` — the same output for the bundled Windows build

The build configuration lists every library that is statically linked into the
binaries.

## Build scripts

- Linux builds: published by <https://johnvansickle.com/ffmpeg/>.
- Windows builds: <https://github.com/GyanD/codexffmpeg> and the release
  workflow in [`.github/workflows/release.yml`](../.github/workflows/release.yml).

Releases up to and including v0.1.4 used the Windows FFmpeg build from
<https://github.com/BtbN/FFmpeg-Builds>. The source and build scripts for those
builds are available from that repository; if you need a specific older build,
use the contact details below.

## Complete corresponding source

The complete corresponding source also includes the source of the libraries
statically linked into the builds (for example x264, x265, libvpx, libaom,
libdav1d, libopus, and libvorbis; see `ffmpeg-buildconf-*.txt` for the exact
list). Their source is available from their respective upstream projects.

If you need a copy of any of these sources, or a source bundle for a specific
release, contact the maintainer:

- Open an issue: <https://github.com/HeHexa6ty/yt-dlp-GUI/issues>
- Maintainer: Kamil Gabrysiewicz (HeHexa6ty)

This offer is valid for at least three years after the release date of each
version.

## The application itself

The application source code is MIT-licensed. It invokes `ffmpeg` and `ffprobe`
as separate processes and does not link against FFmpeg libraries. The GPLv3
applies to the FFmpeg binaries, not to the application code.
