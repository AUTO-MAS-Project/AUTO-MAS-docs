# MFW Shell Import and Issue Reports

`MaaFW/api_service/shell_instances.py` reads shell configurations from the project source directory, and `MaaFW/tools/embedded/shell_instances.py` converts them into account task snapshots. Issue reports are exported by `frontend/electron/services/maafwIssueReportService.ts`, packaging run logs and redacted configurations for the selected managed script.

## Shell Configuration Import

Import entry points live in the new-managed-script onboarding wizard and the account edit page. Shell files are opened read-only, and generated data is saved into MAS account and managed-script configs.

- **Supported shells**: Supports MFAAvalonia (`config/instances/*.json`, reading last-used instance from `appsettings.json`), MXU (`config/mxu-<project>.json`, including pre-tasks and global options), and MFW-PyQt6 (`config/multi_config.json` + `config/configs/*.json`). Shells without verified real-world samples (such as MaaPiCli) are unsupported ([#997][997]); only shells verified against real config shapes are supported.
- **Reading available configs**: When a project contains configs from multiple shells, all of them are listed; unreadable files are skipped individually with a log message so one bad file never blocks the rest.
- **Converting option values**: Shell configs storing case **indexes** are mapped back to case names; boolean toggles normalize to `Yes` / `No`; sub-options are flattened; controller and resource fields are not written onto the account, ensuring the generated snapshot matches what the account page saves.
- **Creating accounts**: In onboarding, all discovered configs are selected by default, creating one account per config named after the config (with numeric suffixes for duplicates). If listing fails or finds nothing, the section is hidden; partial failures report errors while successes proceed; total failure reports an error and falls back to "Create first account"; partial imports list skipped items per account. Overall, non-disruptive issues stay quiet while impactful errors are surfaced without blocking the flow.
- **Overwriting an existing account**: The "Shell" tab in the account page's "Import config" dialog calls `/api/scripts/maafw/shell-instances/apply` to overwrite the current account with one shell config; the computed snapshot is returned in the response so the UI replaces local state directly ([#1195][1195]), avoiding re-fetching account config and clobbering other unsaved edits.
- **Importing hotkeys**: Instance listings include hotkeys and the source directory; when importing accounts in onboarding, the option to also import managed-script hotkeys is checked by default and writes only keys differing from defaults ([#1174][1174]), since hotkeys are managed-script level.
- **Handling missing sources**: When the source directory is unset or deleted, MAS prompts the user to select the project directory on the managed-script page first; the hotkey dialog also provides "Choose another directory…" as a one-time read, since the source directory is no longer updated after import.

## Issue Reports

Issue reports collect artifacts per managed script and apply redaction rules before writing the archive.

- **Export scope**: Settings → Advanced → "Export MFW issue report" (dropdown picking a managed script, including flavors) and "Export M9A issue report" (one-click collection of all M9A managed scripts) ([#1039][1039]), because standard "Export logs" excludes run history.
- **Selecting run records**: Collects history from the 5 most recent runs plus the 3 most recent failed runs (`.log`, `.json`, `.worker.log`, `.maafw.log`, `.project.log`, and failure screenshots), project `debug/` agent logs not present in history, redacted managed-script config and project version, MAS logs, and a `manifest.json` summarizing what was collected and truncated (including runtime check results)—gathering everything needed for troubleshooting in one bundle.
- **Matching artifact ownership**: Because history directories are organized by account display name, MAS inspects the view path recorded at the top of `.worker.log` to claim artifacts for the right managed script, preventing identically named accounts from different managed scripts from mixing.
- **Filtering logs for projects with passwords**: Top-level `debug/maafw.log` contains plaintext passwords and is never collected; for projects with password inputs, `.worker.log` writes a redaction marker (`option_secrets.LOG_REDACTION_MARKER`) on its first line, and only runs carrying that line are collected. MAS already redacts when writing history, and this marker confirms which logs were redacted.
- **Redacting config snapshots**: Because snapshots are JSON strings, they are parsed into objects and redacted recursively, covering both `globalOptions` and `taskOptions` so embedded secrets are not missed by simple prefix matching.
- **Limiting archive size**: Export size is capped; oversized text files are read from the tail starting at a complete line before redaction, and export yields periodically to the main process so tens of megabytes of native logs do not freeze the UI.
- **Recording runtime environment**: The runtime check selects the native library using the exact same criteria as the backend health check and runs one `import` without writing bytecode, replacing paths with `<view>`, `<install_dir>`, and `<HOME>` ([#1088][1088]), so troubleshooting "project Python unavailable" does not require users to run manual commands.

## Limitations

- Task display names are exported verbatim in issue reports.
- Logs written directly by project agents are outside MAS's control; redaction using all account passwords for that managed script applies only when saved into `.project.log`.
- MFAAvalonia resource-level options and MFW-PyQt6 checkboxes and global options lack verified real-world samples; mismatched shapes are skipped cleanly rather than written incorrectly.

[997]: https://github.com/AUTO-MAS-Project/AUTO-MAS/pull/997
[1039]: https://github.com/AUTO-MAS-Project/AUTO-MAS/pull/1039
[1088]: https://github.com/AUTO-MAS-Project/AUTO-MAS/pull/1088
[1174]: https://github.com/AUTO-MAS-Project/AUTO-MAS/pull/1174
[1195]: https://github.com/AUTO-MAS-Project/AUTO-MAS/pull/1195
