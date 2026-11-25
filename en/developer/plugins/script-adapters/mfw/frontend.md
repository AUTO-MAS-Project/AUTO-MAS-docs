# MFW Frontend and Interaction

The MFW managed-script and account pages live under `frontend/src/views/EditView/` and handle project import, runtime environments, device settings, and task configuration. Shared pages read project-specific wording and components from the [Flavor](./flavor.md) registry.

## Managed-Script Page

The managed-script page handles project import, runtime-environment preparation, device settings, and updates. Manual updates execute in the background, and the progress panel can [reattach to the job](./update.md#update-trigger-paths) when the edit page is reopened.

| Design | Why |
| --- | --- |
| New managed scripts use an onboarding wizard: step 0 requires the runtime environment to be ready before continuing; the final step features a primary "Create first account" button and supports importing accounts from shell configs ([#993][993], [#997][997]) | Configuring further steps is pointless if the environment cannot run |
| Basic info puts the interface overview table on the left (columns: Project name · Version · Summary) and the "Runtime environment" panel on the right; the "Prepare runtime environment" button sits in the panel header, bypasses the cache, and acts as "Retry" on failure ([#843][843]) | — |
| **The conclusion is the last line of the log in a single accent color**, using the backend's conclusion text directly—no extra status tags, progress bars, or popups; in dark mode, log boxes use borders without solid fill backgrounds | One place to read the outcome |
| After import, the directory field shows the copy path and its picker button is locked; embedded-copy metadata (source, imported version, current version, shared managed-script count) appears only in the opening lines of the runtime-environment log | After import, the source is only a record; users should not think changing the directory switches projects |
| Developer mode: the status row shows a "Developer mode" or "Source layout" tag and sync timestamp next to "Sync now"; in source layout, the toggle is locked on ([#1294][1294]) | — |
| Win32 control fits in two rows: Launch mode \| Game executable; Unity resolution \| Launch args \| Wait time (s) \| Key mapping | — |
| ADB control: the screencap/input strategy table stays at two rows next to the package name; under Emulator 2.0, selecting a concrete device displays capabilities by device type | — |
| Game package name: first interface read fills it only when empty; switching resources always overwrites it; left untouched if ambiguous or no candidate is inferred ([#855][855]) | Different servers use different package names |
| Project update section: "MirrorChyan CDK \| Proxy address" sits on one row with **no helper text below inputs**—explanations move into question-mark tooltips; the CDK tooltip itself links directly to MirrorChyan; a warning line appears only when MirrorChyan is selected without a CDK ([#943][943]) | Keeps the layout clean |
| When a CDK is entered, a small status line below the input shows remaining days and expiration date (highlighted in warning color at ≤ 7 days) or the error reason (expired, invalid, daily quota exhausted, etc.); hidden when empty, when the project has no MirrorChyan integration, or when querying fails. Queried once on page load, after CDK prefill, and on blur if changed, regardless of the selected download source ([#1330][1330]) | Update checks omit the CDK, so bad CDKs previously surfaced only during download; hiding failed queries stays quieter than showing errors |
| Emulator and instance dropdowns use only the backend list's built-in "Unselected" option (value `-`) without adding a duplicate frontend "Unspecified" item, showing placeholder text while loading ([#1331][1331]) | Avoids duplicate options and matches other adapter edit pages |
| `Emulator.Id` of `-` means unselected; device dropdowns and config queries treat it as empty instead of querying emulator config with it ([#1345][1345]) | `-` is a sentinel, not an ID |
| Per-task time limit, per-task overrides, and the loop-guard toggle all live in the "Per-task time limits" dialog, with the loop-guard switch on the first row ([#1180][1180], [#1223][1223]) | — |
| Key mapping: a read-only box displays "Default / N customized" with a "Configure" button opening the dialog; hidden when `interface.json` has no `hotkey` options effective for the current controller ([#1174][1174]) | — |

## Account Page

The account page saves a single account's task snapshot. The task queue keeps a two-column layout (task list on the left, option editor on the right), while shared pages accept flavor customizations through sections or slots.

| Design | Why |
| --- | --- |
| Task queue uses two equal-height fixed columns (640 px) with independent scrolling at a 9 / 15 width ratio; below 992 px viewport width, they stack vertically at 420 px each ([#855][855], [#1037][1037]) | Clean overall layout |
| Queue rows display only the task name; entry and group move to the right pane's subtitle; task descriptions include a "Description" subheading | — |
| "Enabled" in basic info uses a dropdown; presets are not switched inside basic info (separating account info from the queue); never-run accounts display "Last run: Never" | — |
| No "Read interface" button on the account page | `interface.json` does not change while editing an account |
| No extra onboarding button when the project is not imported | An unimported project cannot be used yet; prompts belong on the managed-script page |
| Global options render once above the queue ([#1294][1294]) | One set per account |
| Tasks removed from `interface.json` remain as ghosts: strikethrough + "Task no longer exists in interface", deletable only; selecting one lists its previous settings on the right (passwords masked); unseen tasks in "Add task" carry a `✦ NEW` badge ([#1147][1147]) | Prevents silent config loss when a project renames tasks |
| Context menu: Duplicate task (inserts directly below with options and display name copied), Rename display name (up to 40 chars), Delete (with confirmation); ghosts allow Delete only; managed, unselectable, and currently unavailable tasks cannot be duplicated; right-clicking also selects the row ([#1301][1301]) | — |
| When two or more rows share the same visible label, display as "Name #N" numbered by queue order at render time only; grouping is based on the visible text | Users distinguish rows by what they see |
| Managed and unselectable tasks are hidden from "Add task" and presets; entries already in the queue stay visible with a notice above the queue | See [Flavor](./flavor.md) |
| The "Templates" dialog has "My templates / Project presets" tabs; the "Import config" dialog has tabs for other accounts under the same managed script and shell configs ([#1260][1260]) | — |
| Password inputs render as password fields showing only "Set"; checkbox min/max counts are enforced in the UI ([#973][973]) | Required by the PI protocol |
| Hotkey and setting sections stay outside the two-column queue | Hotkeys are managed-script level and belong on the managed-script page |

## Create Dialog and Reuse

MFW and all flavors share the same creation flow for project selection and reuse.

1. **Listing project sources**: Selecting MFW / M9A / MSS adds a "Where is the project from?" step: the first row is "Select local directory", followed by already-imported projects grouped by project name + version, filtered to the selected type. Reuse is a creation-time choice; after import, the directory field is display-only with no switcher.
2. **Checking reuse availability**: When all managed scripts holding a project are currently running, that row is disabled to preserve existing behavior.
3. **Opening the matching flavor wizard**: Reusing a project lands on the onboarding page for its type.

## Project Display Name

Naming order: `label` (counted only if different from `name`) → first segment of `title` split on `|` / `｜` with trailing version numbers stripped → `name` ([#993][993]). The managed-script name auto-updates to the project name only while it is still the default name, empty, or the previously auto-assigned project name; custom user names stay untouched. Frontend `utils/maafwProjectName.ts` and backend `interface_display_name` in `interface/preview.py` share this exact rule and must stay in sync.

Why: when `label` is omitted, the interface model fills `label` with `name`, so checking only `label` would show repository names; `title` is often a window title containing version numbers and slogans that are too long to display verbatim.

## Copy and Tooltip Principles

| Principle | Guideline |
| --- | --- |
| Keep only essential notices | Real failures, read errors, and settings that will be ignored |
| Do not restate widget state | Avoid "settings will be saved", "switching back restores it", or "executed by X" |
| Do not warn about unselectable items | Unselectable items cannot be picked anyway, and fallback is not an error |
| Clean up upstream script descriptions | Strip `true` / `false`, raw value lists, and sentences repeating the label; omit the tooltip when unsure |
| Reserve question marks for behavioral boundaries | E.g., directory must contain `interface.json`, meaning of leaving blank, update timing/sources, and CDK; put units directly in labels such as "Wait time (s)" |
| User-facing terminology | Always write "MFW", and "MaaFramework" for the framework itself; "copy" is acceptable; internal terms (view, group, registration, payload, projection, clone) must not appear in the UI |
| Logs only grow more detailed | Add stage timings, progress, and causes; never sanitize by translating English diagnostic logs into Chinese, deduplicating lines, or dropping lines |

## Internationalization

- Add strings to all three locale files (`zh-CN`, `en-US`, `ja-JP`); never hard-code new UI copy.
- Backend logs and notifications are not internationalized; mixed Chinese and English is acceptable.
- Selecting a project's own language files based on the UI language is not yet supported.

[843]: https://github.com/AUTO-MAS-Project/AUTO-MAS/pull/843
[855]: https://github.com/AUTO-MAS-Project/AUTO-MAS/pull/855
[943]: https://github.com/AUTO-MAS-Project/AUTO-MAS/pull/943
[973]: https://github.com/AUTO-MAS-Project/AUTO-MAS/pull/973
[993]: https://github.com/AUTO-MAS-Project/AUTO-MAS/pull/993
[997]: https://github.com/AUTO-MAS-Project/AUTO-MAS/pull/997
[1037]: https://github.com/AUTO-MAS-Project/AUTO-MAS/pull/1037
[1147]: https://github.com/AUTO-MAS-Project/AUTO-MAS/pull/1147
[1174]: https://github.com/AUTO-MAS-Project/AUTO-MAS/pull/1174
[1180]: https://github.com/AUTO-MAS-Project/AUTO-MAS/pull/1180
[1223]: https://github.com/AUTO-MAS-Project/AUTO-MAS/pull/1223
[1260]: https://github.com/AUTO-MAS-Project/AUTO-MAS/pull/1260
[1294]: https://github.com/AUTO-MAS-Project/AUTO-MAS/pull/1294
[1301]: https://github.com/AUTO-MAS-Project/AUTO-MAS/pull/1301
[1330]: https://github.com/AUTO-MAS-Project/AUTO-MAS/pull/1330
[1331]: https://github.com/AUTO-MAS-Project/AUTO-MAS/pull/1331
[1345]: https://github.com/AUTO-MAS-Project/AUTO-MAS/pull/1345
