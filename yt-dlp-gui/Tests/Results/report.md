# TestSet verification

- Run: `run-1793269276`
- Started (epoch seconds): `1793269276`
- Finished (epoch seconds): `1793269587`
- Test set: `Tests/TestSet.md`
- Results: `Tests/Results`
- yt-dlp: `2026.10.26`
- FFmpeg: `ffmpeg version 6.1.1 Copyright (c) 2000-2023 the FFmpeg developers`
- Cases: **33** total, **33** backend pass, **33** fully pass, **0** UI-only, **0** N/A, **0** fail

## Summary

| # | Platform | Method | URL | Analysis | Download | UI | Result |
|---|----------|--------|-----|----------|----------|----|--------|
| 1 | YouTube | Video as video | https://www.youtube.com/watch?v=0qanF-91aJo | PASS | PASS | yes | PASS |
| 2 | YouTube | Video as audio | https://www.youtube.com/watch?v=0qanF-91aJo | PASS | PASS | yes | PASS |
| 3 | YouTube | Video as video | https://www.youtube.com/shorts/2Php6qUjM7U | PASS | PASS | yes | PASS |
| 4 | YouTube | Video as audio | https://www.youtube.com/shorts/2Php6qUjM7U | PASS | PASS | yes | PASS |
| 5 | Instagram | Image as | https://www.instagram.com/p/DXs_dQLiJ1Z/ | PASS | PASS | yes | PASS |
| 6 | Instagram | Image as | https://www.instagram.com/p/DXs_dQLiJ1Z/ | PASS | PASS | yes | PASS |
| 7 | Instagram | Image as | https://www.instagram.com/p/DXs_dQLiJ1Z/ | PASS | PASS | yes | PASS |
| 8 | Instagram | Carousel as | https://www.instagram.com/p/Del3uLqj-Bd/ | PASS | PASS | yes | PASS |
| 9 | Instagram | Carousel as | https://www.instagram.com/p/Del3uLqj-Bd/ | PASS | PASS | yes | PASS |
| 10 | Instagram | Video as video | https://www.instagram.com/p/DeHol0COVql/ | PASS | PASS | yes | PASS |
| 11 | Instagram | Video as video | https://www.instagram.com/p/DeUUWEEBR3k/ | PASS | PASS | yes | PASS |
| 12 | Instagram | Video as audio | https://www.instagram.com/p/DeHol0COVql/ | PASS | PASS | yes | PASS |
| 13 | Instagram | Video as audio | https://www.instagram.com/p/DeUUWEEBR3k/ | PASS | PASS | yes | PASS |
| 14 | Instagram | Video as gif | https://www.instagram.com/p/DeHol0COVql/ | PASS | PASS | yes | PASS |
| 15 | Instagram | Video as gif | https://www.instagram.com/p/DeUUWEEBR3k/ | PASS | PASS | yes | PASS |
| 16 | X | GIF in post | https://x.com/whippyberry/status/2114022926886596998 | PASS | PASS | yes | PASS |
| 17 | X | GIF as a post | https://x.com/i/status/2114022926886596998 | PASS | PASS | yes | PASS |
| 18 | X | Image as | https://x.com/Bonnymama7/status/2108936914166567227?s=20 | PASS | PASS | yes | PASS |
| 19 | X | Image as | https://x.com/Bonnymama7/status/2108936914166567227?s=20 | PASS | PASS | yes | PASS |
| 20 | X | Image as | https://x.com/Bonnymama7/status/2108936914166567227?s=20 | PASS | PASS | yes | PASS |
| 21 | X | Carousel as | https://x.com/malymon/status/2017391718954332185?s=20 | PASS | PASS | yes | PASS |
| 22 | X | Carousel as | https://x.com/malymon/status/2017391718954332185?s=20 | PASS | PASS | yes | PASS |
| 23 | X | Carousel as | https://x.com/malymon/status/2017391718954332185?s=20 | PASS | PASS | yes | PASS |
| 24 | X | Video as gif | https://x.com/KazaArte/status/2110379220325343539?s=20 | PASS | PASS | yes | PASS |
| 25 | X | Video as gif | https://x.com/KazaArte/status/2110379220325343539/video/1 | PASS | PASS | yes | PASS |
| 26 | X | Video as gif | https://x.com/KosherRem/status/2113934488711430648/video/1 | PASS | PASS | yes | PASS |
| 27 | X | Video as gif | https://x.com/KosherRem/status/2113934488711430648?s=20 | PASS | PASS | yes | PASS |
| 28 | X | Video as video | https://x.com/KazaArte/status/2110379220325343539?s=20 | PASS | PASS | yes | PASS |
| 29 | X | Video as video | https://x.com/KazaArte/status/2110379220325343539/video/1 | PASS | PASS | yes | PASS |
| 30 | X | Video as video | https://x.com/KosherRem/status/2113934488711430648/video/1 | PASS | PASS | yes | PASS |
| 31 | X | Video as video | https://x.com/KosherRem/status/2113934488711430648?s=20 | PASS | PASS | yes | PASS |
| 32 | X | Video as audio | https://x.com/KazaArte/status/2110379220325343539?s=20 | PASS | PASS | yes | PASS |
| 33 | X | Video as audio | https://x.com/KazaArte/status/2110379220325343539/video/1 | PASS | PASS | yes | PASS |

## Details

### 1. YouTube — Video as video

- URL: `https://www.youtube.com/watch?v=0qanF-91aJo`
- Expected source / format: `video` / `mp4`
- Actual source: `video`
- Title: BLACK SABBATH - "Paranoid" (Official Video)
- Id: `0qanF-91aJo`
- Duration: `171` seconds
- Expected file count: `1`; actual: `1`
- Files:
  - `/home/catchy/Projects/Code/yt-dlp-gui/tauri-ytdlp-gui/Tests/Results/media/run-1793269276/youtube/video-as-video/url-1/BLACK SABBATH - _Paranoid_ (Official Video) [0qanF-91aJo].mp4` — 42711042 bytes — magic `MP4` (OK) — extension OK
- Backend: PASS; UI format offered: yes

### 2. YouTube — Video as audio

- URL: `https://www.youtube.com/watch?v=0qanF-91aJo`
- Expected source / format: `video` / `mp3`
- Actual source: `video`
- Title: BLACK SABBATH - "Paranoid" (Official Video)
- Id: `0qanF-91aJo`
- Duration: `171` seconds
- Expected file count: `1`; actual: `1`
- Files:
  - `/home/catchy/Projects/Code/yt-dlp-gui/tauri-ytdlp-gui/Tests/Results/media/run-1793269276/youtube/video-as-audio/url-1/BLACK SABBATH - _Paranoid_ (Official Video) [0qanF-91aJo].mp3` — 5874765 bytes — magic `MP3` (OK) — extension OK
- Backend: PASS; UI format offered: yes

### 3. YouTube — Video as video

- URL: `https://www.youtube.com/shorts/2Php6qUjM7U`
- Expected source / format: `video` / `mp4`
- Actual source: `video`
- Title: Italo Vegliante and his interpretation of the spaghetti Western soundtrack "For a few dollars more"
- Id: `2Php6qUjM7U`
- Duration: `53` seconds
- Expected file count: `1`; actual: `1`
- Files:
  - `/home/catchy/Projects/Code/yt-dlp-gui/tauri-ytdlp-gui/Tests/Results/media/run-1793269276/youtube/video-as-video/url-1/Italo Vegliante and his interpretation of the spaghetti Western soundtrack _For a few dollars more_ [2Php6qUjM7U].mp4` — 4630353 bytes — magic `MP4` (OK) — extension OK
- Backend: PASS; UI format offered: yes

### 4. YouTube — Video as audio

- URL: `https://www.youtube.com/shorts/2Php6qUjM7U`
- Expected source / format: `video` / `mp3`
- Actual source: `video`
- Title: Italo Vegliante and his interpretation of the spaghetti Western soundtrack "For a few dollars more"
- Id: `2Php6qUjM7U`
- Duration: `53` seconds
- Expected file count: `1`; actual: `1`
- Files:
  - `/home/catchy/Projects/Code/yt-dlp-gui/tauri-ytdlp-gui/Tests/Results/media/run-1793269276/youtube/video-as-audio/url-1/Italo Vegliante and his interpretation of the spaghetti Western soundtrack _For a few dollars more_ [2Php6qUjM7U].mp3` — 1103421 bytes — magic `MP3` (OK) — extension OK
- Backend: PASS; UI format offered: yes

### 5. Instagram — Image as

- URL: `https://www.instagram.com/p/DXs_dQLiJ1Z/`
- Expected source / format: `image` / `png`
- Actual source: `image`
- Title: Video by deadlockupdate
- Id: `DXs_dQLiJ1Z`
- Expected file count: `1`; actual: `1`
- Files:
  - `/home/catchy/Projects/Code/yt-dlp-gui/tauri-ytdlp-gui/Tests/Results/media/run-1793269276/instagram/image-as/url-1/Video by deadlockupdate [DXs_dQLiJ1Z].png` — 2981396 bytes — magic `PNG` (OK) — extension OK
- Backend: PASS; UI format offered: yes

### 6. Instagram — Image as

- URL: `https://www.instagram.com/p/DXs_dQLiJ1Z/`
- Expected source / format: `image` / `jpg`
- Actual source: `image`
- Title: Video by deadlockupdate
- Id: `DXs_dQLiJ1Z`
- Expected file count: `1`; actual: `1`
- Files:
  - `/home/catchy/Projects/Code/yt-dlp-gui/tauri-ytdlp-gui/Tests/Results/media/run-1793269276/instagram/image-as/url-1/Video by deadlockupdate [DXs_dQLiJ1Z].jpg` — 238751 bytes — magic `JPG` (OK) — extension OK
- Backend: PASS; UI format offered: yes

### 7. Instagram — Image as

- URL: `https://www.instagram.com/p/DXs_dQLiJ1Z/`
- Expected source / format: `image` / `gif`
- Actual source: `image`
- Title: Video by deadlockupdate
- Id: `DXs_dQLiJ1Z`
- Expected file count: `1`; actual: `1`
- Files:
  - `/home/catchy/Projects/Code/yt-dlp-gui/tauri-ytdlp-gui/Tests/Results/media/run-1793269276/instagram/image-as/url-1/Video by deadlockupdate [DXs_dQLiJ1Z].gif` — 167860 bytes — magic `GIF` (OK) — extension OK
- Backend: PASS; UI format offered: yes

### 8. Instagram — Carousel as

- URL: `https://www.instagram.com/p/Del3uLqj-Bd/`
- Expected source / format: `image` / `png`
- Actual source: `image`
- Title: Video by youlooksmartman
- Id: `Del2uCbvQKq`
- Expected file count: `>= 2`; actual: `10`
- Files:
  - `/home/catchy/Projects/Code/yt-dlp-gui/tauri-ytdlp-gui/Tests/Results/media/run-1793269276/instagram/carousel-as/url-1/Video by youlooksmartman [Del2uCbvQKq]_1.png` — 161653 bytes — magic `PNG` (OK) — extension OK
  - `/home/catchy/Projects/Code/yt-dlp-gui/tauri-ytdlp-gui/Tests/Results/media/run-1793269276/instagram/carousel-as/url-1/Video by youlooksmartman [Del2uCbvQKq]_2.png` — 187198 bytes — magic `PNG` (OK) — extension OK
  - `/home/catchy/Projects/Code/yt-dlp-gui/tauri-ytdlp-gui/Tests/Results/media/run-1793269276/instagram/carousel-as/url-1/Video by youlooksmartman [Del2uCbvQKq]_3.png` — 162442 bytes — magic `PNG` (OK) — extension OK
  - `/home/catchy/Projects/Code/yt-dlp-gui/tauri-ytdlp-gui/Tests/Results/media/run-1793269276/instagram/carousel-as/url-1/Video by youlooksmartman [Del2uCbvQKq]_4.png` — 164767 bytes — magic `PNG` (OK) — extension OK
  - `/home/catchy/Projects/Code/yt-dlp-gui/tauri-ytdlp-gui/Tests/Results/media/run-1793269276/instagram/carousel-as/url-1/Video by youlooksmartman [Del2uCbvQKq]_5.png` — 159463 bytes — magic `PNG` (OK) — extension OK
  - `/home/catchy/Projects/Code/yt-dlp-gui/tauri-ytdlp-gui/Tests/Results/media/run-1793269276/instagram/carousel-as/url-1/Video by youlooksmartman [Del2uCbvQKq]_6.png` — 151347 bytes — magic `PNG` (OK) — extension OK
  - `/home/catchy/Projects/Code/yt-dlp-gui/tauri-ytdlp-gui/Tests/Results/media/run-1793269276/instagram/carousel-as/url-1/Video by youlooksmartman [Del2uCbvQKq]_7.png` — 164509 bytes — magic `PNG` (OK) — extension OK
  - `/home/catchy/Projects/Code/yt-dlp-gui/tauri-ytdlp-gui/Tests/Results/media/run-1793269276/instagram/carousel-as/url-1/Video by youlooksmartman [Del2uCbvQKq]_8.png` — 156065 bytes — magic `PNG` (OK) — extension OK
  - `/home/catchy/Projects/Code/yt-dlp-gui/tauri-ytdlp-gui/Tests/Results/media/run-1793269276/instagram/carousel-as/url-1/Video by youlooksmartman [Del2uCbvQKq]_9.png` — 137925 bytes — magic `PNG` (OK) — extension OK
  - `/home/catchy/Projects/Code/yt-dlp-gui/tauri-ytdlp-gui/Tests/Results/media/run-1793269276/instagram/carousel-as/url-1/Video by youlooksmartman [Del2uCbvQKq]_10.png` — 157137 bytes — magic `PNG` (OK) — extension OK
- Backend: PASS; UI format offered: yes

### 9. Instagram — Carousel as

- URL: `https://www.instagram.com/p/Del3uLqj-Bd/`
- Expected source / format: `image` / `jpg`
- Actual source: `image`
- Title: Video by youlooksmartman
- Id: `Del2uCbvQKq`
- Expected file count: `>= 2`; actual: `10`
- Files:
  - `/home/catchy/Projects/Code/yt-dlp-gui/tauri-ytdlp-gui/Tests/Results/media/run-1793269276/instagram/carousel-as/url-1/Video by youlooksmartman [Del2uCbvQKq]_1.jpg` — 39599 bytes — magic `JPG` (OK) — extension OK
  - `/home/catchy/Projects/Code/yt-dlp-gui/tauri-ytdlp-gui/Tests/Results/media/run-1793269276/instagram/carousel-as/url-1/Video by youlooksmartman [Del2uCbvQKq]_2.jpg` — 44408 bytes — magic `JPG` (OK) — extension OK
  - `/home/catchy/Projects/Code/yt-dlp-gui/tauri-ytdlp-gui/Tests/Results/media/run-1793269276/instagram/carousel-as/url-1/Video by youlooksmartman [Del2uCbvQKq]_3.jpg` — 37583 bytes — magic `JPG` (OK) — extension OK
  - `/home/catchy/Projects/Code/yt-dlp-gui/tauri-ytdlp-gui/Tests/Results/media/run-1793269276/instagram/carousel-as/url-1/Video by youlooksmartman [Del2uCbvQKq]_4.jpg` — 39621 bytes — magic `JPG` (OK) — extension OK
  - `/home/catchy/Projects/Code/yt-dlp-gui/tauri-ytdlp-gui/Tests/Results/media/run-1793269276/instagram/carousel-as/url-1/Video by youlooksmartman [Del2uCbvQKq]_5.jpg` — 41776 bytes — magic `JPG` (OK) — extension OK
  - `/home/catchy/Projects/Code/yt-dlp-gui/tauri-ytdlp-gui/Tests/Results/media/run-1793269276/instagram/carousel-as/url-1/Video by youlooksmartman [Del2uCbvQKq]_6.jpg` — 35433 bytes — magic `JPG` (OK) — extension OK
  - `/home/catchy/Projects/Code/yt-dlp-gui/tauri-ytdlp-gui/Tests/Results/media/run-1793269276/instagram/carousel-as/url-1/Video by youlooksmartman [Del2uCbvQKq]_7.jpg` — 38962 bytes — magic `JPG` (OK) — extension OK
  - `/home/catchy/Projects/Code/yt-dlp-gui/tauri-ytdlp-gui/Tests/Results/media/run-1793269276/instagram/carousel-as/url-1/Video by youlooksmartman [Del2uCbvQKq]_8.jpg` — 37848 bytes — magic `JPG` (OK) — extension OK
  - `/home/catchy/Projects/Code/yt-dlp-gui/tauri-ytdlp-gui/Tests/Results/media/run-1793269276/instagram/carousel-as/url-1/Video by youlooksmartman [Del2uCbvQKq]_9.jpg` — 34953 bytes — magic `JPG` (OK) — extension OK
  - `/home/catchy/Projects/Code/yt-dlp-gui/tauri-ytdlp-gui/Tests/Results/media/run-1793269276/instagram/carousel-as/url-1/Video by youlooksmartman [Del2uCbvQKq]_10.jpg` — 40892 bytes — magic `JPG` (OK) — extension OK
- Backend: PASS; UI format offered: yes

### 10. Instagram — Video as video

- URL: `https://www.instagram.com/p/DeHol0COVql/`
- Expected source / format: `video` / `mp4`
- Actual source: `video`
- Title: Video by korsaghost
- Id: `DeHol0COVql`
- Duration: `12.236916` seconds
- Expected file count: `1`; actual: `1`
- Files:
  - `/home/catchy/Projects/Code/yt-dlp-gui/tauri-ytdlp-gui/Tests/Results/media/run-1793269276/instagram/video-as-video/url-1/Video by korsaghost [DeHol0COVql].mp4` — 1834989 bytes — magic `MP4` (OK) — extension OK
- Backend: PASS; UI format offered: yes

### 11. Instagram — Video as video

- URL: `https://www.instagram.com/p/DeUUWEEBR3k/`
- Expected source / format: `video` / `mp4`
- Actual source: `video`
- Title: Video by welloffables
- Id: `DeUUWEEBR3k`
- Duration: `29.720091` seconds
- Expected file count: `1`; actual: `1`
- Files:
  - `/home/catchy/Projects/Code/yt-dlp-gui/tauri-ytdlp-gui/Tests/Results/media/run-1793269276/instagram/video-as-video/url-2/Video by welloffables [DeUUWEEBR3k].mp4` — 7463663 bytes — magic `MP4` (OK) — extension OK
- Backend: PASS; UI format offered: yes

### 12. Instagram — Video as audio

- URL: `https://www.instagram.com/p/DeHol0COVql/`
- Expected source / format: `video` / `mp3`
- Actual source: `video`
- Title: Video by korsaghost
- Id: `DeHol0COVql`
- Duration: `12.236916` seconds
- Expected file count: `1`; actual: `1`
- Files:
  - `/home/catchy/Projects/Code/yt-dlp-gui/tauri-ytdlp-gui/Tests/Results/media/run-1793269276/instagram/video-as-audio/url-1/Video by korsaghost [DeHol0COVql].mp3` — 355603 bytes — magic `MP3` (OK) — extension OK
- Backend: PASS; UI format offered: yes

### 13. Instagram — Video as audio

- URL: `https://www.instagram.com/p/DeUUWEEBR3k/`
- Expected source / format: `video` / `mp3`
- Actual source: `video`
- Title: Video by welloffables
- Id: `DeUUWEEBR3k`
- Duration: `29.720091` seconds
- Expected file count: `1`; actual: `1`
- Files:
  - `/home/catchy/Projects/Code/yt-dlp-gui/tauri-ytdlp-gui/Tests/Results/media/run-1793269276/instagram/video-as-audio/url-2/Video by welloffables [DeUUWEEBR3k].mp3` — 895655 bytes — magic `MP3` (OK) — extension OK
- Backend: PASS; UI format offered: yes

### 14. Instagram — Video as gif

- URL: `https://www.instagram.com/p/DeHol0COVql/`
- Expected source / format: `video` / `gif`
- Actual source: `video`
- Title: Video by korsaghost
- Id: `DeHol0COVql`
- Duration: `12.236916` seconds
- Expected file count: `1`; actual: `1`
- Files:
  - `/home/catchy/Projects/Code/yt-dlp-gui/tauri-ytdlp-gui/Tests/Results/media/run-1793269276/instagram/video-as-gif/url-1/Video by korsaghost [DeHol0COVql].gif` — 21786519 bytes — magic `GIF` (OK) — extension OK
- Backend: PASS; UI format offered: yes

### 15. Instagram — Video as gif

- URL: `https://www.instagram.com/p/DeUUWEEBR3k/`
- Expected source / format: `video` / `gif`
- Actual source: `video`
- Title: Video by welloffables
- Id: `DeUUWEEBR3k`
- Duration: `29.720091` seconds
- Expected file count: `1`; actual: `1`
- Files:
  - `/home/catchy/Projects/Code/yt-dlp-gui/tauri-ytdlp-gui/Tests/Results/media/run-1793269276/instagram/video-as-gif/url-2/Video by welloffables [DeUUWEEBR3k].gif` — 71534231 bytes — magic `GIF` (OK) — extension OK
- Backend: PASS; UI format offered: yes

### 16. X — GIF in post

- URL: `https://x.com/whippyberry/status/2114022926886596998`
- Expected source / format: `gif` / `gif`
- Actual source: `gif`
- Title: Whippy🌸 (COMM OPENS) - fat rem being a fat chud after eating all the farms hits a big snooze...
- Id: `2114018698285703168`
- Expected file count: `1`; actual: `1`
- Files:
  - `/home/catchy/Projects/Code/yt-dlp-gui/tauri-ytdlp-gui/Tests/Results/media/run-1793269276/x/gif-in-post/url-1/Whippy🌸 (COMM OPENS) - fat rem being a fat chud after eating all the farms hits a big snooze [2114018698285703168].gif` — 425146 bytes — magic `GIF` (OK) — extension OK
- Backend: PASS; UI format offered: yes

### 17. X — GIF as a post

- URL: `https://x.com/i/status/2114022926886596998`
- Expected source / format: `gif` / `gif`
- Actual source: `gif`
- Title: Whippy🌸 (COMM OPENS) - fat rem being a fat chud after eating all the farms hits a big snooze...
- Id: `2114018698285703168`
- Expected file count: `1`; actual: `1`
- Files:
  - `/home/catchy/Projects/Code/yt-dlp-gui/tauri-ytdlp-gui/Tests/Results/media/run-1793269276/x/gif-as-a-post/url-1/Whippy🌸 (COMM OPENS) - fat rem being a fat chud after eating all the farms hits a big snooze [2114018698285703168].gif` — 425146 bytes — magic `GIF` (OK) — extension OK
- Backend: PASS; UI format offered: yes

### 18. X — Image as

- URL: `https://x.com/Bonnymama7/status/2108936914166567227?s=20`
- Expected source / format: `image` / `png`
- Actual source: `image`
- Title: Bonny 😉 (Commission OPEN) - Spit her out 🐈  #deadlock
- Id: `2108936914166567227`
- Expected file count: `1`; actual: `1`
- Files:
  - `/home/catchy/Projects/Code/yt-dlp-gui/tauri-ytdlp-gui/Tests/Results/media/run-1793269276/x/image-as/url-1/Bonny 😉 (Commission OPEN) - Spit her out 🐈 #deadlock [2108936914166567227].png` — 7327082 bytes — magic `PNG` (OK) — extension OK
- Backend: PASS; UI format offered: yes

### 19. X — Image as

- URL: `https://x.com/Bonnymama7/status/2108936914166567227?s=20`
- Expected source / format: `image` / `jpg`
- Actual source: `image`
- Title: Bonny 😉 (Commission OPEN) - Spit her out 🐈  #deadlock
- Id: `2108936914166567227`
- Expected file count: `1`; actual: `1`
- Files:
  - `/home/catchy/Projects/Code/yt-dlp-gui/tauri-ytdlp-gui/Tests/Results/media/run-1793269276/x/image-as/url-1/Bonny 😉 (Commission OPEN) - Spit her out 🐈 #deadlock [2108936914166567227].jpg` — 710468 bytes — magic `JPG` (OK) — extension OK
- Backend: PASS; UI format offered: yes

### 20. X — Image as

- URL: `https://x.com/Bonnymama7/status/2108936914166567227?s=20`
- Expected source / format: `image` / `gif`
- Actual source: `image`
- Title: Bonny 😉 (Commission OPEN) - Spit her out 🐈  #deadlock
- Id: `2108936914166567227`
- Expected file count: `1`; actual: `1`
- Files:
  - `/home/catchy/Projects/Code/yt-dlp-gui/tauri-ytdlp-gui/Tests/Results/media/run-1793269276/x/image-as/url-1/Bonny 😉 (Commission OPEN) - Spit her out 🐈 #deadlock [2108936914166567227].gif` — 147473 bytes — magic `GIF` (OK) — extension OK
- Backend: PASS; UI format offered: yes

### 21. X — Carousel as

- URL: `https://x.com/malymon/status/2017391718954332185?s=20`
- Expected source / format: `image` / `png`
- Actual source: `image`
- Title: maly 🌸 VGen Artist - Deadlock Rem and Graves icons inspired by Overwatch Cute Player icons...
- Id: `2017391718954332185`
- Expected file count: `>= 2`; actual: `2`
- Files:
  - `/home/catchy/Projects/Code/yt-dlp-gui/tauri-ytdlp-gui/Tests/Results/media/run-1793269276/x/carousel-as/url-1/maly 🌸 VGen Artist - Deadlock Rem and Graves icons inspired by Overwatch Cute Player icons [2017391718954332185]_1.png` — 43764 bytes — magic `PNG` (OK) — extension OK
  - `/home/catchy/Projects/Code/yt-dlp-gui/tauri-ytdlp-gui/Tests/Results/media/run-1793269276/x/carousel-as/url-1/maly 🌸 VGen Artist - Deadlock Rem and Graves icons inspired by Overwatch Cute Player icons [2017391718954332185]_2.png` — 44903 bytes — magic `PNG` (OK) — extension OK
- Backend: PASS; UI format offered: yes

### 22. X — Carousel as

- URL: `https://x.com/malymon/status/2017391718954332185?s=20`
- Expected source / format: `image` / `jpg`
- Actual source: `image`
- Title: maly 🌸 VGen Artist - Deadlock Rem and Graves icons inspired by Overwatch Cute Player icons...
- Id: `2017391718954332185`
- Expected file count: `>= 2`; actual: `2`
- Files:
  - `/home/catchy/Projects/Code/yt-dlp-gui/tauri-ytdlp-gui/Tests/Results/media/run-1793269276/x/carousel-as/url-1/maly 🌸 VGen Artist - Deadlock Rem and Graves icons inspired by Overwatch Cute Player icons [2017391718954332185]_1.jpg` — 20920 bytes — magic `JPG` (OK) — extension OK
  - `/home/catchy/Projects/Code/yt-dlp-gui/tauri-ytdlp-gui/Tests/Results/media/run-1793269276/x/carousel-as/url-1/maly 🌸 VGen Artist - Deadlock Rem and Graves icons inspired by Overwatch Cute Player icons [2017391718954332185]_2.jpg` — 25985 bytes — magic `JPG` (OK) — extension OK
- Backend: PASS; UI format offered: yes

### 23. X — Carousel as

- URL: `https://x.com/malymon/status/2017391718954332185?s=20`
- Expected source / format: `image` / `gif`
- Actual source: `image`
- Title: maly 🌸 VGen Artist - Deadlock Rem and Graves icons inspired by Overwatch Cute Player icons...
- Id: `2017391718954332185`
- Expected file count: `>= 2`; actual: `2`
- Files:
  - `/home/catchy/Projects/Code/yt-dlp-gui/tauri-ytdlp-gui/Tests/Results/media/run-1793269276/x/carousel-as/url-1/maly 🌸 VGen Artist - Deadlock Rem and Graves icons inspired by Overwatch Cute Player icons [2017391718954332185]_1.gif` — 42210 bytes — magic `GIF` (OK) — extension OK
  - `/home/catchy/Projects/Code/yt-dlp-gui/tauri-ytdlp-gui/Tests/Results/media/run-1793269276/x/carousel-as/url-1/maly 🌸 VGen Artist - Deadlock Rem and Graves icons inspired by Overwatch Cute Player icons [2017391718954332185]_2.gif` — 46113 bytes — magic `GIF` (OK) — extension OK
- Backend: PASS; UI format offered: yes

### 24. X — Video as gif

- URL: `https://x.com/KazaArte/status/2110379220325343539?s=20`
- Expected source / format: `video` / `gif`
- Actual source: `video`
- Title: 睾丸爆炸99 - Rem riding tricycle hitting heavy crate and die  #Deadlock
- Id: `2110379007078449152`
- Duration: `2.033` seconds
- Expected file count: `1`; actual: `1`
- Files:
  - `/home/catchy/Projects/Code/yt-dlp-gui/tauri-ytdlp-gui/Tests/Results/media/run-1793269276/x/video-as-gif/url-1/睾丸爆炸99 - Rem riding tricycle hitting heavy crate and die #Deadlock [2110379007078449152].gif` — 878098 bytes — magic `GIF` (OK) — extension OK
- Backend: PASS; UI format offered: yes

### 25. X — Video as gif

- URL: `https://x.com/KazaArte/status/2110379220325343539/video/1`
- Expected source / format: `video` / `gif`
- Actual source: `video`
- Title: 睾丸爆炸99 - Rem riding tricycle hitting heavy crate and die  #Deadlock
- Id: `2110379007078449152`
- Duration: `2.033` seconds
- Expected file count: `1`; actual: `1`
- Files:
  - `/home/catchy/Projects/Code/yt-dlp-gui/tauri-ytdlp-gui/Tests/Results/media/run-1793269276/x/video-as-gif/url-2/睾丸爆炸99 - Rem riding tricycle hitting heavy crate and die #Deadlock [2110379007078449152].gif` — 878098 bytes — magic `GIF` (OK) — extension OK
- Backend: PASS; UI format offered: yes

### 26. X — Video as gif

- URL: `https://x.com/KosherRem/status/2113934488711430648/video/1`
- Expected source / format: `video` / `gif`
- Actual source: `video`
- Title: Ruby - Rem #deadlockrem #deadlock  #Rem
- Id: `2113934326668689408`
- Duration: `12.195` seconds
- Expected file count: `1`; actual: `1`
- Files:
  - `/home/catchy/Projects/Code/yt-dlp-gui/tauri-ytdlp-gui/Tests/Results/media/run-1793269276/x/video-as-gif/url-3/Ruby - Rem #deadlockrem #deadlock #Rem [2113934326668689408].gif` — 13659164 bytes — magic `GIF` (OK) — extension OK
- Backend: PASS; UI format offered: yes

### 27. X — Video as gif

- URL: `https://x.com/KosherRem/status/2113934488711430648?s=20`
- Expected source / format: `video` / `gif`
- Actual source: `video`
- Title: Ruby - Rem #deadlockrem #deadlock  #Rem
- Id: `2113934326668689408`
- Duration: `12.195` seconds
- Expected file count: `1`; actual: `1`
- Files:
  - `/home/catchy/Projects/Code/yt-dlp-gui/tauri-ytdlp-gui/Tests/Results/media/run-1793269276/x/video-as-gif/url-4/Ruby - Rem #deadlockrem #deadlock #Rem [2113934326668689408].gif` — 13659164 bytes — magic `GIF` (OK) — extension OK
- Backend: PASS; UI format offered: yes

### 28. X — Video as video

- URL: `https://x.com/KazaArte/status/2110379220325343539?s=20`
- Expected source / format: `video` / `mp4`
- Actual source: `video`
- Title: 睾丸爆炸99 - Rem riding tricycle hitting heavy crate and die  #Deadlock
- Id: `2110379007078449152`
- Duration: `2.033` seconds
- Expected file count: `1`; actual: `1`
- Files:
  - `/home/catchy/Projects/Code/yt-dlp-gui/tauri-ytdlp-gui/Tests/Results/media/run-1793269276/x/video-as-video/url-1/睾丸爆炸99 - Rem riding tricycle hitting heavy crate and die #Deadlock [2110379007078449152].mp4` — 152833 bytes — magic `MP4` (OK) — extension OK
- Backend: PASS; UI format offered: yes

### 29. X — Video as video

- URL: `https://x.com/KazaArte/status/2110379220325343539/video/1`
- Expected source / format: `video` / `mp4`
- Actual source: `video`
- Title: 睾丸爆炸99 - Rem riding tricycle hitting heavy crate and die  #Deadlock
- Id: `2110379007078449152`
- Duration: `2.033` seconds
- Expected file count: `1`; actual: `1`
- Files:
  - `/home/catchy/Projects/Code/yt-dlp-gui/tauri-ytdlp-gui/Tests/Results/media/run-1793269276/x/video-as-video/url-2/睾丸爆炸99 - Rem riding tricycle hitting heavy crate and die #Deadlock [2110379007078449152].mp4` — 152833 bytes — magic `MP4` (OK) — extension OK
- Backend: PASS; UI format offered: yes

### 30. X — Video as video

- URL: `https://x.com/KosherRem/status/2113934488711430648/video/1`
- Expected source / format: `video` / `mp4`
- Actual source: `video`
- Title: Ruby - Rem #deadlockrem #deadlock  #Rem
- Id: `2113934326668689408`
- Duration: `12.195` seconds
- Expected file count: `1`; actual: `1`
- Files:
  - `/home/catchy/Projects/Code/yt-dlp-gui/tauri-ytdlp-gui/Tests/Results/media/run-1793269276/x/video-as-video/url-3/Ruby - Rem #deadlockrem #deadlock #Rem [2113934326668689408].mp4` — 390629 bytes — magic `MP4` (OK) — extension OK
- Backend: PASS; UI format offered: yes

### 31. X — Video as video

- URL: `https://x.com/KosherRem/status/2113934488711430648?s=20`
- Expected source / format: `video` / `mp4`
- Actual source: `video`
- Title: Ruby - Rem #deadlockrem #deadlock  #Rem
- Id: `2113934326668689408`
- Duration: `12.195` seconds
- Expected file count: `1`; actual: `1`
- Files:
  - `/home/catchy/Projects/Code/yt-dlp-gui/tauri-ytdlp-gui/Tests/Results/media/run-1793269276/x/video-as-video/url-4/Ruby - Rem #deadlockrem #deadlock #Rem [2113934326668689408].mp4` — 390629 bytes — magic `MP4` (OK) — extension OK
- Backend: PASS; UI format offered: yes

### 32. X — Video as audio

- URL: `https://x.com/KazaArte/status/2110379220325343539?s=20`
- Expected source / format: `video` / `mp3`
- Actual source: `video`
- Title: 睾丸爆炸99 - Rem riding tricycle hitting heavy crate and die  #Deadlock
- Id: `2110379007078449152`
- Duration: `2.033` seconds
- Expected file count: `1`; actual: `1`
- Files:
  - `/home/catchy/Projects/Code/yt-dlp-gui/tauri-ytdlp-gui/Tests/Results/media/run-1793269276/x/video-as-audio/url-1/睾丸爆炸99 - Rem riding tricycle hitting heavy crate and die #Deadlock [2110379007078449152].mp3` — 21890 bytes — magic `MP3` (OK) — extension OK
- Backend: PASS; UI format offered: yes

### 33. X — Video as audio

- URL: `https://x.com/KazaArte/status/2110379220325343539/video/1`
- Expected source / format: `video` / `mp3`
- Actual source: `video`
- Title: 睾丸爆炸99 - Rem riding tricycle hitting heavy crate and die  #Deadlock
- Id: `2110379007078449152`
- Duration: `2.033` seconds
- Expected file count: `1`; actual: `1`
- Files:
  - `/home/catchy/Projects/Code/yt-dlp-gui/tauri-ytdlp-gui/Tests/Results/media/run-1793269276/x/video-as-audio/url-2/睾丸爆炸99 - Rem riding tricycle hitting heavy crate and die #Deadlock [2110379007078449152].mp3` — 21890 bytes — magic `MP3` (OK) — extension OK
- Backend: PASS; UI format offered: yes

