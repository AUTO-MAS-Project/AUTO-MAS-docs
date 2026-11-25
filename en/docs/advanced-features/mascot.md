# Desktop Mascot Packages

A mascot package is a ZIP archive containing a character image, optional animation sprite sheets, and a configuration file. You can start with a single transparent image and later add idle, working, mouse-tracking, success, failure, and drag animations.

Desktop mascots appear in an independent floating window that you can drag anywhere on your desktop. They stay visible when the main window is minimized and close when you exit AUTO-MAS. Mascots can be bundled inside a theme package or built and imported as standalone mascot packages. For theme colors and backgrounds, see [Custom Appearance Packages](./appearance).

::: info Supported Clients
This guide applies to clients that include **Settings → Interface Settings → Appearance Configuration → Import Mascot** (设置 → 界面设置 → 外观配置 → 导入桌宠). Older clients without a standalone mascot import option must be updated first.
:::

Custom drag sprite sheets also require client support for `mascot.animation.dragPath`. For older mascot-capable clients, omit `dragPath` and its image to keep the built-in drag motion.

## Importing and Switching

1. Open **Settings → Interface Settings** (设置 → 界面设置) and locate **Appearance Configuration** (外观配置).
2. Click **Import Mascot** (导入桌宠) and select your ZIP file—do not extract it manually.
3. Once imported, the mascot is automatically selected; confirm that **Show Desktop Mascot** (显示桌面桌宠) is enabled.
4. You can switch between installed mascots anytime from the **Mascot Selection** (桌宠选择) dropdown.
5. To update an existing mascot, keep its `id` unchanged, import the updated ZIP, and confirm replacement.

**Follow current theme** (跟随当前主题) uses the mascot bundled with the active theme (or hides the mascot if the theme does not include one). Once a standalone mascot is selected, switching themes preserves your chosen mascot. Turning off **Show Desktop Mascot** hides it without deleting the package.

Local **Import Mascot** can also accept an older theme package that contains a mascot: the client extracts only the mascot assets without applying the theme's colors or background. For sharing a standalone mascot package, however, always use the `mascot.json` format described below.

## Bundling a Mascot in a Theme Package

Theme packages continue to use `theme.json` at the ZIP root. Copy the entire `mascot` object from the example below into `theme.json`, keep the theme's `mode`, `tokens`, and other fields, and place the mascot images in the theme's `assets/` folder. Animations use the same `mascot.animation` fields and sprite-sheet layouts—do not add a separate `mascot.json`.

See [Custom Appearance Packages](./appearance#corner-decoration-mascot) for other theme fields and packaging steps. After applying the theme via **Import Appearance**, set **Mascot Selection** to **Follow current theme** and enable **Show Desktop Mascot**. Themes may omit `mascot` entirely without affecting colors or backgrounds.

## Building a Minimal Mascot Package

### 1. Create a Folder Structure

Create a working folder containing the config file and character image:

```text
my-mascot/                  ← Working folder only; do not include this folder in the ZIP
├── mascot.json             ← Required: standalone mascot configuration
└── assets/
    └── mascot.png          ← Required: static character image
```

Use ASCII letters, digits, and hyphens for filenames. Always use forward slashes (`/`) in paths, such as `assets/mascot.png`.

### 2. Prepare a Transparent Image

Use a transparent PNG with the full character, tail, and props intact, slight margin around the edges, and clean alpha edges without stray pixels. Static images also accept JPEG and WebP, though JPEG does not support transparency.

For example, if you plan to display the mascot at roughly 192 pixels wide, prepare a 384-pixel-wide source image. Extra transparent padding counts toward the display width, so more empty margin makes the character appear smaller.

Remember that a checkerboard pattern is only how image editors visualize transparency—never bake it into the image. Preview your PNG against both light and dark backgrounds to catch white fringes, dark outlines, or leftover background pixels.

### 3. Write `mascot.json`

Save as a UTF-8 plain text file named exactly `mascot.json` (make sure Windows does not rename it to `mascot.json.txt`).

Minimal starter configuration:

```json
{
  "formatVersion": 1,
  "id": "my-mascot",
  "name": "My Mascot",
  "mascot": {
    "path": "assets/mascot.png",
    "width": 192
  },
  "preview": "assets/mascot.png"
}
```

Change `id`, `name`, and your image file to start. Standalone mascots do not use theme fields like `mode`, `tokens`, or `background`.

### 4. Pack as a ZIP

Open the `my-mascot` folder, select `mascot.json` and `assets`, and compress them into a ZIP archive. Opening the ZIP should reveal those two items immediately, not an outer `my-mascot` folder.

You can use File Explorer, 7-Zip, Bandizip, or Windows' built-in `tar` from a terminal inside the working folder:

```powershell
tar -a -c -f ..\my-mascot.zip mascot.json assets
```

ZIP entries must use forward slashes (`/`). If you encounter a "Windows path" or backslash error on import, repack using one of the methods above. As noted in [Custom Appearance Packages](./appearance#pack-as-a-zip), avoid `Compress-Archive` in Windows PowerShell 5.1.

### 5. Import and Verify

After importing, check that the character is uncropped, transparent areas are clean, and the display size looks right. After editing images or `mascot.json`, repack and re-import using the same `id` and choose **Replace**.

## Adding Animations and Mouse Tracking {#adding-animations-and-mouse-tracking}

All animation assets are optional. Prepare only the sprite sheets you want and configure their corresponding fields; do not reference missing assets or leave unused files in the ZIP.

| Asset | Layout | Purpose |
| --- | --- | --- |
| `mascot.png` | Single image | Static display and base character image (always required) |
| `animation.png` | 2 rows × `columns` columns | Row 1: idle; Row 2: working |
| `look.png` | Fixed 3×3 grid | Nine static gaze directions following the mouse cursor |
| `results.png` | 2 rows × same columns as main animation | Row 1: failure; Row 2: success |
| `drag.png` | 1 row × same columns as main animation | Dedicated looping pose while being dragged |

### Idle and Working Animations

Combine sequential animation frames into a regular PNG sprite sheet. The client steps through grid cells rather than playing GIFs or videos.

For example, with 8 frames per row and 256×256 pixels per frame, `animation.png` should be 2048×512 pixels:

```text
Row 1: Idle0  Idle1  Idle2  Idle3  Idle4  Idle5  Idle6  Idle7
Row 2: Work0  Work1  Work2  Work3  Work4  Work5  Work6  Work7
```

Every cell must have identical width and height with no divider lines or padding gaps. Keep character scale, center, and foot baseline consistent across frames so only the intended motion changes. Make the last frame loop smoothly back to the first.

`columns` sets the number of frames per row (an integer from `2` to `16`). Changing `columns` from `8` to `16` does not generate extra frames—your image must actually have 16 cells per row.

### Nine-Direction Static Gaze

`look.png` is always a 3×3 grid, ordered from the viewer's perspective:

| Top-Left | Up | Top-Right |
| --- | --- | --- |
| Left | Center / Straight Ahead | Right |
| Bottom-Left | Down | Bottom-Right |

Matching the frame size of the main animation is easiest: if each main frame is 256×256 pixels, `look.png` is 768×768 pixels. Its 3×3 grid never changes with `columns`.

You can move just the eyes or add a slight head tilt across the nine cells, while keeping character scale, props, canvas, and baseline identical to prevent jumping when switching directions.

Mouse tracking activates during idle. After the cursor pauses briefly or moves away, the mascot returns to its idle animation. Mouse tracking pauses during working, dragging, and task result animations. Because gaze cells are static images, selecting a gaze direction freezes the entire frame—including any background code, particles, or decorations drawn into it.

### Success and Failure Animations

`results.png` has the same number of columns as the main animation: Row 1 is the **failure** animation and Row 2 is the **success** animation:

```text
Row 1: Fail0  Fail1  Fail2  Fail3  Fail4  Fail5  Fail6  Fail7
Row 2: Pass0  Pass1  Pass2  Pass3  Pass4  Pass5  Pass6  Pass7
```

In the 256×256 × 8-column example above, `results.png` is also 2048×512 pixels. Using the same cell dimensions as the main animation is recommended. When a task result arrives, the client plays the corresponding row at `workingFps`, then returns to the previous state once finished.

### Complete Configuration Example {#complete-configuration-example}

Full directory structure:

```text
my-mascot/
├── mascot.json
└── assets/
    ├── mascot.png
    ├── animation.png
    ├── look.png
    ├── results.png
    └── drag.png
```

The configuration below assumes 8 frames per row for both the main and result animations:

```json
{
  "formatVersion": 1,
  "id": "my-mascot",
  "name": "My Mascot",
  "description": "Includes idle, working, mouse gaze, success, failure, and drag animations.",
  "mascot": {
    "path": "assets/mascot.png",
    "width": 192,
    "opacity": 1,
    "position": "bottom-right",
    "animation": {
      "path": "assets/animation.png",
      "columns": 8,
      "idleFps": 6,
      "workingFps": 10,
      "lookPath": "assets/look.png",
      "resultPath": "assets/results.png",
      "dragPath": "assets/drag.png"
    }
  },
  "preview": "assets/mascot.png"
}
```

If you only create idle and working animations, remove `lookPath`, `resultPath`, and `dragPath` along with their image files. If you do not include any animations, remove the entire `animation` object to keep the minimal setup.

### Custom Drag Animation {#custom-drag-animation}

To switch to a lifted, dangling, or wiggling pose while dragging, create `drag.png` and reference it in `mascot.animation.dragPath`.

The drag sprite sheet has **one row** with the same number of columns as `columns`. With 8 columns at 256×256 pixels each, `drag.png` is 2048×256 pixels:

```text
Drag0  Drag1  Drag2  Drag3  Drag4  Drag5  Drag6  Drag7
```

Keep cell size, character proportions, and baseline consistent with the main animation. While dragging, the client plays this sprite sheet at `workingFps` without applying the built-in sway effect. On release, it resumes idle, working, or any ongoing task-result state. Mouse tracking is paused during dragging.

If `dragPath` is omitted, the client uses its default built-in drag motion with no extra assets needed. When animations are paused, low-performance mode is active, or the OS "Reduce motion" setting is enabled, dragging displays the first frame of `drag.png` without animating. Theme-bundled and standalone mascots use the exact same configuration.

### Current Support for Manually Stopped Tasks

The confused reaction shown after manually stopping a task is provided by the client itself and needs no extra artwork. The current format has no dedicated stopped-animation field—do not add `stoppedPath` or leave unused images inside the ZIP.

This reaction differs from right-clicking **Pause Animation** (暂停动画): the former reflects a manually aborted task, whereas the latter pauses mascot playback.

## Field Reference

### Basic Metadata

| Field | Required | Description |
| --- | --- | --- |
| `formatVersion` | Yes | Must be the number `1` |
| `id` | Yes | Unique identifier, max 64 characters; lowercase letters, digits, `_`, `-` only; must start with a letter or digit; avoid Windows reserved names (`con`, `nul`, `com1`, etc.) |
| `name` | Yes | Mascot display name, max 80 characters |
| `description` | No | Short description, max 300 characters; omit if unused |
| `mascot` | Yes | Character image and display settings |
| `preview` | No | Preview image path; may reuse `assets/mascot.png` or point to a separate preview image |

### Character Display {#character-display}

| Field | Default | Description |
| --- | --- | --- |
| `mascot.path` | Required | Path to the static image, must be inside `assets/` |
| `mascot.width` | `144` | Target display width of the full canvas (`16` to `1024` pixels) |
| `mascot.opacity` | `1` | Opacity from `0` to `1` (`0` hides the mascot) |
| `mascot.position` | `bottom-right` | Initial corner: `top-left`, `top-right`, `bottom-left`, or `bottom-right` |

Actual rendered size is also constrained by the current monitor's work area: width cannot exceed 35% of the work area, and height cannot exceed 30%. After you drag the mascot, the client remembers its new position—changing `position` later does not reset a remembered drag position.

### Animation

| Field | Default | Description |
| --- | --- | --- |
| `mascot.animation.path` | Required if animating | Path to the 2-row main animation PNG |
| `mascot.animation.columns` | Required if animating | Frames per row, integer from `2` to `16` |
| `mascot.animation.idleFps` | `4` | Idle frame rate (`1` to `24` FPS) |
| `mascot.animation.workingFps` | `8` | Working, result, and drag frame rate (`1` to `24` FPS) |
| `mascot.animation.lookPath` | Mouse tracking disabled | Path to the static 3×3 gaze PNG |
| `mascot.animation.resultPath` | No custom result sheet | Path to the 2-row result PNG (Row 1 failure, Row 2 success) |
| `mascot.animation.dragPath` | Built-in drag motion | Path to the 1-row drag PNG (same columns as main animation, played at `workingFps`) |

All animation images must live under `assets/` with the `.png` extension. `lookPath`, `resultPath`, and `dragPath` all belong inside `animation` and require the main animation to be configured.

## Drawing Scrolling Code Behind the Character

Scrolling background code can be baked directly into your animation frames—no scripts are needed inside the mascot package.

1. Prepare a seamlessly tiling text/code layer with transparency around the characters.
2. Shift the same text layer upward by a fixed pixel step on each frame, wrapping the repeated section in from the bottom.
3. Place the character frames in front of the code layer, avoiding covering the face and body.
4. Assemble the composited frames into the 2-row main animation (and apply the same scrolling layer to `results.png` if you want it to continue scrolling during task feedback).

For example, if one repeating text block is 80 pixels tall across 8 frames, shift upward by 0, 10, 20, 30, 40, 50, 60, and 70 pixels per frame so the next step (80 pixels) aligns seamlessly with frame 0. Do not regenerate different random code per frame, or it will flicker instead of scrolling smoothly.

A solid black background is unnecessary—colored text pixels behind the character on a transparent canvas work fine. When switching to the static gaze sheet or pausing animation, the code also freezes; this is expected with frame-based playback.

## Using Your Mascot After Import

- Drag the character to reposition it anywhere on the desktop without affecting the main window layout.
- Right-click **Pause Animation** (暂停动画) to pause playback, and uncheck it to resume.
- Right-click **Pause Messages** (暂停留言) to toggle idle speech bubbles. Messages and task reminders are rendered by the client, so do not draw speech bubbles or text into your sprite sheets.
- Switch sources anytime by choosing another standalone mascot or **Follow current theme**.
- Importing a package with the same `id` and choosing **Replace** updates the mascot in place; use distinct `id` values to keep multiple mascots installed side by side.

## Limits

| Item | Limit or requirement |
| --- | --- |
| ZIP archive size | 16 MiB |
| Total extracted size | 32 MiB |
| ZIP entries | 32 (including folder entries) |
| Single file size | 8 MiB |
| `mascot.json` size | 256 KiB |
| Path length inside ZIP | 240 characters |
| Main & result sprite sheets | Max 8192 px per dimension; width divisible by `columns`, height divisible by `2` |
| Drag sprite sheet | Max 8192 px per dimension; 1 row, width divisible by `columns` |
| File validity | Every image must be referenced in `mascot.json`; no empty files, unused files, or mismatched extensions |

PNG, JPEG, and WebP are real image encodings—renaming a file extension does not convert its format. Keep `Thumbs.db`, `desktop.ini`, `.DS_Store`, READMEs, and backup artwork out of the ZIP.

## Troubleshooting

| Symptom or error | What to check |
| --- | --- |
| Cannot find standalone mascot import | Make sure your client supports standalone mascots; older clients with only appearance import cannot import pure mascot packages |
| Missing `mascot.json` | Ensure `mascot.json` is lowercase and located at the ZIP root with no extra wrapper folder |
| Unable to parse configuration | Check for curly quotes, comments, trailing commas, or accidental `.txt` extension |
| Contains undeclared files | Remove unreferenced images, hidden OS files, docs, or spare artwork |
| Missing asset | Verify filenames, extensions, and paths match `mascot.json` exactly |
| Broken animation or misaligned slices | Check the 2-row layout, actual column count vs. `columns`, and remove any cell gaps or grid lines |
| Animation never plays | Check whether **Pause Animation** is enabled in the context menu, or whether low-performance mode or OS "Reduce motion" is active |
| Mouse gaze looks the wrong way | Verify the 3×3 grid order—left and right are from the viewer's perspective |
| Animation and code freeze while tracking mouse | Static gaze frames intentionally freeze playback; idle animation resumes after the mouse stops moving |
| Sad expression plays on task success | Row 1 of `results.png` is failure; Row 2 is success |
| Mascot jumps, shrinks, or jitters | Keep canvas size, character scale, center, and foot baseline identical across frames; do not auto-trim frames individually before packing |
| Mascot does not appear | Turn on **Show Desktop Mascot**, check that `opacity` is not `0`, and verify the theme includes a mascot when using **Follow current theme** |
| Black/white box or checkerboard appears behind character | Those pixels were baked into the image—re-export as a true transparent PNG |

## Creating Assets with AI

### Generating `mascot.json`

State your actual asset paths and grid layouts clearly so the AI references only real files. Copy and fill in the placeholders below:

```text
Generate a valid mascot.json for a standalone AUTO-MAS desktop mascot package. Output valid JSON only.

Name: <mascot name>
id: <lowercase letters, digits, underscores, hyphens; start with letter or digit; max 64 chars>
Prepared files:
- Static image: assets/mascot.png
- Main animation: <assets/animation.png or none>, <frames per row> columns, 2 rows (row 1 idle, row 2 working)
- Gaze sheet: <assets/look.png or none>, fixed 3x3
- Result sheet: <assets/results.png or none>, same columns as main animation, row 1 failure, row 2 success
- Drag sheet: <assets/drag.png or none>, 1 row, same columns as main animation
Display width: <e.g., 192>

Requirements:
- formatVersion must be 1; name max 80 chars; optional description max 300 chars.
- mascot.path is required; animation.path and columns are required when configuring animation.
- columns is an integer from 2 to 120; idleFps and workingFps are from 1 to 120 (recommend starting with 6 and 10).
- All character and animation asset paths start with assets/, and all animation sheets must be PNG.
- Only include lookPath, resultPath, and dragPath inside animation if main animation is present and those files exist.
- Omit any missing assets completely; do not generate theme tokens, background fields, or stoppedPath.
- Reuse assets/mascot.png for preview. Do not invent files, comments, or extra fields.
```

### Generating Character Animations

```text
Create a transparent-background desktop mascot sprite sheet for <character description>.
Canvas size: <total width>x<total height>, strictly 8 columns by 2 rows, each cell <frame width>x<frame height>.
Row 1 (8 frames): gentle idle loop with breathing, blinking, and subtle tail movement.
Row 2 (8 frames): continuous loop of <working action>.
One complete character per cell, keeping appearance, proportions, canvas center, and foot baseline consistent across all frames.
Do not crop the character or props. No grid lines, frame numbers, text, watermarks, ground shadow bases, or opaque background.
The last frame must loop smoothly into the first frame, using a clean true alpha channel.
```

Always inspect AI-generated sprite sheets manually for dimensions, cell count, and alignment. Rearrange frames onto an exact uniform grid in an image editor if needed, and set `columns` based on the actual final image rather than the prompt.

## Sharing and Updating

In clients with categorized **Resource Store** (资源商店) tabs, download mascots from the **Mascots** (桌宠) section and upload standalone mascot packages under the Mascots category using a pure mascot ZIP with `mascot.json` at the root.

Although older theme packages can have their mascot extracted locally, they cannot be uploaded directly as standalone mascots. To share a mascot on its own, keep only the mascot assets and create a clean `mascot.json` without theme colors or background files.

Pre-release checklist:

- [ ] Both fresh import and same-`id` replacement import succeed.
- [ ] Character silhouette and transparent edges look clean over both light and dark desktop wallpapers.
- [ ] Main animation loops smoothly, with idle in Row 1 and working in Row 2.
- [ ] 3×3 mouse gaze directions are mapped accurately and return to idle after the mouse stops.
- [ ] Result animation has failure in Row 1 and success in Row 2, matching the main animation column count.
- [ ] If a drag sheet is included, dragging plays the custom pose and releasing restores normal state; if omitted, built-in drag motion works normally.
- [ ] The ZIP contains only referenced files with no extra parent folder or leftover files.

When bundling a mascot inside a theme, follow the `theme.json` format in [Custom Appearance Packages](./appearance); when sharing a mascot independently, use the `mascot.json` format from this guide.
