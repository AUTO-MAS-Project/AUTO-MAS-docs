# Community Note Backgrounds

These five backgrounds were supplied by the AUTO-MAS requester on 2026-09-04.
On 2026-09-07 the requester identified them as public materials from the
respective game Wikis and requested distribution through this documentation
repository in WebP format.

| File | Game | Reported source | Specific source URL and image license |
| --- | --- | --- | --- |
| arknights-background.webp | Arknights | Game Wiki, requester statement | Not supplied |
| endfield-background.webp | Arknights: Endfield | Game Wiki, requester statement | Not supplied |
| genshin-background.webp | Genshin Impact | Game Wiki, requester statement | Not supplied |
| star-rail-background.webp | Honkai: Star Rail | Game Wiki, requester statement | Not supplied |
| zenless-background.webp | Zenless Zone Zero | Game Wiki, requester statement | Not supplied |

Copyright remains with the respective rights holders. The documentation
repository's software license does not grant rights to third-party game art.
Public accessibility is not independent evidence of redistribution permission;
the exact Wiki file pages and applicable terms still need to be recorded.

The source PNGs and existing notices are preserved in
`AUTO-MAS/frontend/src/assets/community-notes/ATTRIBUTION.md` and are excluded
from the application package. `manifest.json` records source and derivative
SHA-256 hashes, dimensions, and conversion settings. Conversion preserves
aspect ratio and alpha, limits dimensions to 1600 by 900 without upscaling,
and uses Pillow WebP quality 82, method 6.

The repository stores these assets under `public/community-notes/`.
VitePress copies the public directory into the site's output directory
without changing filenames. Publish the documentation assets before releasing
the app that references `https://doc.auto-mas.top/community-notes/`.
