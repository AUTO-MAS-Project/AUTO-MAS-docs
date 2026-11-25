# Custom Appearance Packages

An appearance package is a ZIP archive that lets you customize the color palette, full-window background, sidebar menu icons, mouse cursors, and an optional corner decoration image.

::: tip Desktop Mascots
Clients that support desktop mascots render a theme's bundled mascot as an independent floating window. Clients with standalone mascot support can also import separate mascot packages—see [Desktop Mascot Packages](./mascot) for configuration and animation asset guidelines. The notes below about in-window corner decorations, reserved layout spacing, and hiding on narrow windows apply to older clients.
:::

## Importing and Switching

Go to **Settings → Interface Settings → Theme Mode** (设置 → 界面设置 → 主题模式), click **Import Appearance** (导入外观), and select your ZIP file. After a successful import, a preview dialog appears—click **Apply Appearance** (应用外观) to activate it.

- Imported appearances appear in the **Theme Mode** dropdown alongside Follow System, Light Mode, and Dark Mode, so you can switch back to built-in themes anytime.
- When a custom appearance is selected, a **Remove Appearance** (移除外观) button appears beside it.
- Importing a package with the same `id` prompts you to replace the existing one.
- Appearance packages are stored in your user data directory and are preserved when upgrading AUTO-MAS.
- While a custom appearance is active, its color palette overrides the built-in **Theme Color** option, which becomes unavailable.

## How to Create an Appearance Package

### 1. Create a Working Folder

```text
my-appearance/              ← Working folder only; do not include this folder in the ZIP
├── theme.json              # Required: appearance configuration
├── preview.png             # Optional: preview image shown on import
└── assets/                 # Put background, mascot, menu icons, and cursors here
    ├── background.jpg
    ├── mascot.png
    ├── menu-default.png
    ├── menu-home.png
    └── cursor-pointer.png
```

Use only ASCII letters, digits, and hyphens in filenames.

### 2. Prepare Images

Only PNG, JPEG, and WebP are supported. The dimensions below are recommendations (except for cursors, which have strict limits):

| Purpose | Recommended size | Recommended format | Notes |
| --- | --- | --- | --- |
| Background | 1920×1080 or larger, 16:9 | JPEG / WebP | Anchored to the bottom-right and scaled to `cover` by default; place the main subject at the bottom-right and keep the top-left clean for content |
| Corner decoration | 300–600 px wide | Transparent PNG / WebP | Display width is set by `mascot.width`; using an image around 2× that width keeps it sharp |
| Menu icons | 48×48 or 96×96 | Transparent PNG | Displayed at 24px in the sidebar; use bold lines and simple silhouettes |
| Cursors | 32×32 | Transparent PNG | **Must** be between 1×1 and 64×64 pixels; note the pixel coordinates of the pointer tip |
| Preview image | 16:10 recommended (e.g., 1280×800) | PNG / JPEG / WebP | Used in the import preview and optionally as the Resource Store cover; ratio and size are not strictly enforced on import |

Each image file must be **8 MB or smaller**. Prefer JPEG or WebP compression for background images.

When uploading to the Resource Store for the first time, you can use the package's bundled preview image as the cover or pick another image; if no preview image is included in the package, choosing a cover is required. When uploading a new version, if an existing cover can be reused, **Keep current cover** is selected by default, though you can switch to the package preview or upload a new image. Choosing a cover is mandatory only when there is neither a reusable cover nor a bundled preview image.

Covers support PNG, JPEG, and WebP up to 8 MB, preferably in 16:10. Both the Resource Store listing and upload dialog center-crop covers to 16:10, so keep important text and subjects centered to avoid cropping edges.

### 3. Write `theme.json`

Start from the example below and delete any sections whose images you are not including. Save the file using UTF-8 encoding.

```json
{
  "formatVersion": 1,
  "id": "my-appearance",
  "name": "My Appearance",
  "description": "A warm light-mode appearance example",
  "mode": "light",
  "tokens": {
    "colorPrimary": "#1677ff",
    "colorBgLayout": "#f5f7fb",
    "colorBgContainer": "#ffffff",
    "colorBgElevated": "#ffffff",
    "colorText": "#1f2937",
    "colorTextSecondary": "#667085",
    "colorBorder": "#d0d5dd",
    "colorBorderSecondary": "#eaecf0",
    "borderRadius": 8
  },
  "background": {
    "path": "assets/background.jpg",
    "opacity": 1,
    "surfaceOpacity": 0.72,
    "position": "right bottom",
    "size": "cover"
  },
  "mascot": {
    "path": "assets/mascot.png",
    "width": 144,
    "opacity": 1,
    "position": "bottom-right"
  },
  "preview": "preview.png",
  "menuIcons": {
    "default": "assets/menu-default.png",
    "home": "assets/menu-home.png"
  },
  "cursors": {
    "pointer": {
      "path": "assets/cursor-pointer.png",
      "hotspotX": 6,
      "hotspotY": 0
    }
  }
}
```

### 4. Pack as a ZIP {#pack-as-a-zip}

**Select `theme.json`, `preview.png`, and `assets` directly and compress them—do not zip the outer `my-appearance` folder.** `theme.json` must sit at the root of the ZIP archive.

Choose any of the following methods:

- **File Explorer**: select those items → right-click → **Compress to ZIP file** (on Windows 10: **Send to → Compressed (zipped) folder**).
- **Command line**: open a terminal inside `my-appearance` and run (Windows 10/11 includes `tar`):

  ```powershell
  tar -a -c -f ..\my-appearance.zip theme.json preview.png assets
  ```

- **7-Zip, Bandizip, or similar archivers**: select those items directly and pack them into a ZIP file.

::: warning
Do **not** use `Compress-Archive` in built-in Windows PowerShell 5.1: it writes backslash paths into the ZIP archive, which will be rejected on import. `Compress-Archive` in PowerShell 7 does not have this issue.
:::

### 5. Import, Test, and Iterate

After importing, click **Apply Appearance** in the preview dialog to inspect the result. Whenever you modify images or `theme.json`, repack the ZIP and import it again—choose **Replace** when prompted that the appearance already exists; there is no need to delete the old one first.

## Field Reference

### Basic Metadata

| Field | Required | Description |
| --- | --- | --- |
| `formatVersion` | Yes | Must be `1` |
| `id` | Yes | Unique identifier: lowercase letters, digits, underscores (`_`), and hyphens (`-`) only; must start with a letter or digit; max 64 characters; avoid Windows reserved names such as `con`, `nul`, `com1` |
| `name` | Yes | Display name, max 80 characters |
| `description` | No | Description shown in the preview dialog, max 300 characters |
| `mode` | Yes | `light` or `dark`; determines the base UI palette and should match the brightness of your background image |
| `tokens` | Yes | Color and radius tokens (see below) |
| `background` | No | Full-window background configuration |
| `mascot` | No | Corner decoration / bundled mascot configuration |
| `preview` | No | Path to the preview image; may sit at the ZIP root or inside `assets/` |
| `menuIcons` | No | Sidebar menu icon overrides |
| `cursors` | No | Custom cursor definitions |

### Color Tokens (`tokens`)

All colors must use 6-digit `#rrggbb` hex notation. Alpha channels, 3-digit shorthand, and named colors are not supported.

| Field | Required | Purpose |
| --- | --- | --- |
| `colorPrimary` | Yes | Primary accent color: buttons, active items, links |
| `colorPageTitle` | No | Dedicated color for top-level page titles (such as Managed Scripts, Plans, and Emulators); does not affect buttons, menus, body text, or card headers |
| `colorBgLayout` | No | Base page layout background |
| `colorBgContainer` | No | Background for containers such as cards and inputs |
| `colorBgElevated` | No | Background for title bar, sidebar, dialogs, and dropdown menus |
| `colorText` | No | Primary body text |
| `colorTextSecondary` | No | Secondary text |
| `colorBorder` | No | Standard borders |
| `colorBorderSecondary` | No | Subtle borders and dividers |
| `borderRadius` | No | Corner radius, a number from `0` to `32` |

Except for `colorPageTitle`, omitted color tokens are derived automatically from `mode` and `colorPrimary`.

#### Setting a Dedicated Page Title Color

When a light primary color sits on top of a background image, large page titles can lose contrast. You can keep your button accent color while giving page titles a more legible shade:

```json
"tokens": {
  "colorPrimary": "#548466",
  "colorPageTitle": "#244a38"
}
```

`colorPageTitle` only changes top-level page titles—it does not modify the home greeting, window title, or card titles. When omitted, each page keeps its default styling (pages with a primary gradient keep the gradient; pages using text color keep text color). Older themes do not need to add this field, and removing it restores default behavior.

::: tip Client Compatibility
Dedicated page title colors require a client that supports `colorPageTitle`. Older clients ignore this field while still applying the rest of the palette and images; importing and re-exporting a package through an older client will drop this field.
:::

When using [MAS Theme Studio](https://github.com/jinghero/AUTO-MAS-Theme-Studio), enable **Custom Page Title Color** (单独设置页面标题色) under **Colors & Background** (配色与背景), pick a color, and click **Preview Page Title** (预览页面标题). Unchecking removes the field; switching modes or color presets preserves it, while resetting to default MAS colors clears it. See the studio's [illustrated guide](https://github.com/jinghero/AUTO-MAS-Theme-Studio/blob/main/guide.md) for examples.

### Background (`background`)

A fixed full-window background spanning the title bar, sidebar, and content area. The image stays anchored when switching pages or scrolling.

| Field | Default | Description |
| --- | --- | --- |
| `path` | — | Required; must be inside `assets/` |
| `opacity` | `1` | Opacity of the image itself (`0` to `1`) |
| `surfaceOpacity` | `0.72` | Opacity of layout and card surfaces (`0` to `1`; `0` is fully transparent, `1` is opaque) |
| `position` | `right bottom` | Horizontal alignment (`left` / `center` / `right`), optionally followed by a space and vertical alignment (`top` / `center` / `bottom`) |
| `size` | `cover` | `cover` fills the window (may crop), `contain` shows the full image (may letterbox), `auto` uses native size |

Text, icons, buttons, and inputs stay crisp and opaque; dialogs, drawers, and dropdown menus also remain opaque. Translucent surfaces are enabled only when a background image is provided—color-only appearance packages keep solid backgrounds.

Emulator Management, Resource Store, and Account Editor pages follow the same surface opacity rules: the outer page reveals the background while content cards render at `surfaceOpacity` without double-layering inner panels. When no background image is configured, standard backgrounds are preserved.

### Corner Decoration (`mascot`) {#corner-decoration-mascot}

To bundle a mascot with a theme package, configure the `mascot` object inside `theme.json` and place the images under `assets/`—do not add a separate `mascot.json`. On clients that support desktop mascots, it renders as a standalone floating window when **Follow current theme** (跟随当前主题) and **Show desktop mascot** (显示桌面桌宠) are enabled.

A theme's bundled mascot can also define `mascot.animation` using the exact same fields and sprite-sheet layouts as standalone mascot packages. See [Adding Animations and Mouse Tracking](./mascot#adding-animations-and-mouse-tracking) for creating idle, working, mouse-tracking, and result animations; you can copy the entire `mascot` object from the [Complete Configuration Example](./mascot#complete-configuration-example) into `theme.json` while keeping your theme fields.

To switch to a dedicated pose while dragging on supported clients, add `mascot.animation.dragPath`—see [Custom Drag Animation](./mascot#custom-drag-animation). Themes without this field continue using the built-in drag motion.

| Field | Default | Description |
| --- | --- | --- |
| `path` | — | Required; must be inside `assets/` |
| `width` | `144` | Target display width in pixels (`16` to `1024`); for floating desktop mascots, actual size is also clamped to 35% of the monitor work-area width and 30% of its height |
| `opacity` | `1` | `0` to `1` |
| `position` | `bottom-right` | `top-left`, `top-right`, `bottom-left`, or `bottom-right` |

Older clients reserve layout padding on the decoration's side to avoid covering content and hide the decoration when window width is 800px or less. The initialization screen and standalone log window apply the appearance's palette and background without showing the decoration. For floating desktop mascot sizing and behavior, see [Character Display](./mascot#character-display).

### Menu Icons (`menuIcons`)

Each value points to an image under `assets/`, and multiple keys may reuse the same image. A menu item checks its specific key first, falls back to `menuIcons.default`, and keeps the built-in icon if neither is set.

To replace all sidebar icons with one uniform image, you only need to set `default`; add specific keys only for items that need unique icons. For instance, the example above uses `menu-home.png` for Home and `menu-default.png` for all other menus. Note that `menuIcons.default` is the fallback menu icon, unrelated to the default mouse pointer `cursors.default`.

| Key | Menu item |
| --- | --- |
| `default` | Fallback icon for menus without a dedicated key |
| `home` | Home (主页) |
| `scripts` | Managed Scripts (托管管理) |
| `plans` | Plans (计划管理) |
| `emulators` | Emulators (模拟器管理) |
| `queue` | Queue (调度队列) |
| `scheduler` | Scheduler (调度中心) |
| `gameSign` | Community Sign-in (游戏社区) |
| `history` | History (历史记录) |
| `tools` | Tools (工具) |
| `themeStore` | Resource Store (资源商店) |
| `settings` | Settings (设置) |

Development builds also support four developer menu keys: `testRouter`, `ocrDev`, `overlayMaskDev`, and `updateDownloadDev`.

`menuIcons.default` requires client support. If an older client reports `default` as an unknown menu key, remove `default` and assign icons per menu key instead.

### Cursors (`cursors`)

| Key | Where it is used |
| --- | --- |
| `default` | Standard pointer |
| `pointer` | Hovering over clickable elements such as buttons and links |
| `text` | Hovering over text inputs and selectable text |

- `path` must point to a PNG inside `assets/` with both width and height between `1` and `64` pixels.
- `hotspotX` and `hotspotY` define the active click pixel measured from the top-left corner `(0, 0)`. Both default to `0` and must lie within the image dimensions. Standard arrows usually use `(0, 0)`, hand pointers use the fingertip, and text cursors use the midpoint of the I-beam.
- Any omitted key keeps the system cursor.

## Limits

| Item | Limit |
| --- | --- |
| ZIP archive size | 16 MB |
| Total extracted size | 32 MB |
| ZIP entries (including folders) | 32 |
| Single file size | 8 MB |
| `theme.json` size | 256 KB |
| Path length inside ZIP | 240 characters |

Every file inside the ZIP must be referenced in `theme.json`. Unreferenced files and empty files are not allowed.

## Common Import Errors

| Message | Cause and fix |
| --- | --- |
| `theme.json` must be at the ZIP root | You zipped the parent folder instead of its contents. Open the folder, select the files inside, and compress them |
| Missing `theme.json` | No `theme.json` in the ZIP, or wrong filename casing |
| ZIP contains absolute or Windows paths | Packed with Windows PowerShell 5.1 `Compress-Archive`; use another packing method |
| Unsupported image type / undeclared files | Hidden system files (`Thumbs.db`, `desktop.ini`, `.DS_Store`, `__MACOSX`) or unreferenced images are inside the ZIP; remove them and repack |
| Missing asset | A path in `theme.json` does not match the file in the ZIP; check spelling, casing, and extensions |
| Image content does not match extension | For example, a JPEG renamed to `.png`; re-export properly with an image editor |
| Cursor dimensions must be between 1 and 64 pixels | Resize the cursor PNG to 64×64 or smaller |
| Hotspot must be within image bounds | `hotspotX` or `hotspotY` exceeds the cursor image dimensions |
| ... must be a `#rrggbb` color | You used `#fff`, `rgb(...)`, or an 8-digit hex value with alpha |
| Unable to parse `theme.json` | JSON syntax error, commonly a trailing comma or curly quotes |
| `id` may only contain lowercase letters, digits, underscores, and hyphens | Remove uppercase letters, spaces, or non-ASCII characters from `id` |

## Creating Assets with AI

You can ask an AI assistant to draft `theme.json` and generate image assets. Copy the prompts below and replace the angle-bracket placeholders with your own details.

### Generating `theme.json`

Send the following prompt to a conversational AI (ChatGPT, Claude, DeepSeek, etc.):

```text
You are an assistant for building AUTO-MAS custom appearance packages. Generate a valid theme.json based on my description. Output JSON only, with no explanation.

My style: <describe style and colors, e.g., warm cream yellow and light brown, cozy light theme>
Appearance name: <e.g., Cream Pudding>
Files I have prepared (paths inside ZIP):
- Background: <assets/background.jpg, light or dark overall, main subject at bottom-right>
- Corner decoration: <assets/mascot.png, or "none">
- Preview image: <preview.png, or "none">
- Menu icons: <e.g., default -> assets/menu-default.png, home -> assets/menu-home.png; or "none">
- Cursors: <e.g., default -> assets/cursor-default.png, 32x32, tip at (0,0); or "none">

Format requirements:
1. formatVersion must be 1.
2. id: lowercase letters, digits, underscores, and hyphens only, starting with a letter or digit, max 64 chars; name max 80 chars; description max 300 chars.
3. mode: "light" or "dark", matching the background brightness.
4. tokens may only include: colorPrimary (required), colorPageTitle, colorBgLayout, colorBgContainer, colorBgElevated, colorText, colorTextSecondary, colorBorder, colorBorderSecondary, borderRadius. All colors must be 6-digit #rrggbb hex; borderRadius is a number from 0 to 32. Body text contrast against background should be at least 4.5:1. Include colorPageTitle only if I explicitly request a separate page title color; otherwise omit it.
5. background: path must start with assets/; opacity and surfaceOpacity are 0 to 1 (higher surfaceOpacity such as 0.6–0.85 for busy backgrounds); position is "left", "center", or "right", optionally followed by a space and "top", "center", or "bottom"; size is "cover", "contain", or "auto".
6. mascot: path starts with assets/; width 16 to 1024; opacity 0 to 1; position is one of "top-left", "top-right", "bottom-left", "bottom-right".
7. menuIcons: may use default for fallback plus any of home, scripts, plans, emulators, queue, scheduler, gameSign, history, tools, themeStore, settings. Only include keys for which I provided files.
8. cursors: allowed keys are default, pointer, text; path must be a .png under assets/; hotspotX and hotspotY are integers from 0 to 63 within the image dimensions.
9. Reference only the files I listed. Omit any section marked "none". Do not invent paths or extra fields.
```

### Generating Image Assets

Image models (Midjourney, DALL·E, etc.) rarely output exact pixel dimensions. Crop and resize the generated images to the sizes listed in **Prepare Images** above.

**Background**

```text
An illustration in <style and theme>, designed as a full-window background for a desktop app, landscape 16:9. Main subject concentrated in the bottom-right corner, with a large clean, low-contrast area across the left and top for overlaying text and UI cards. <Light / Dark> overall tone centered on <primary color>. No text, watermarks, UI elements, or borders.
```

**Corner Decoration**

```text
Full-body illustration of <character or object>, <style>, transparent background, complete uncropped subject with slight padding around edges, clean edges, no text and no ground shadow base.
```

**Menu Icons**

```text
A unified set of 11 desktop sidebar icons representing: Home, Managed Scripts, Plans, Emulators, Queue, Scheduler, Community Sign-in, History, Tools, Resource Store, Settings. <Style>, <primary color> palette, transparent background, each icon centered at the same scale, simple silhouette and bold strokes so it stays recognizable at 24px. No text.
```

Image models usually generate all icons in one sheet—slice them into separate PNG files and map each to its key in `menuIcons`.

If you want one icon across all menus, generate a single general icon and assign it to `menuIcons.default`.

**Cursor**

```text
Mouse pointer icon, <style>, transparent background, square canvas, pointer tip at the top-left corner, clean simple shape that remains sharp when scaled down to 32x32 pixels. No text.
```

After resizing to 32×32, inspect the pixel coordinates of the pointer tip in an image editor and enter them as `hotspotX` and `hotspotY`.
