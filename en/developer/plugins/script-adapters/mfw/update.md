# MFW Project Updates

`MaaFW/tools/embedded/view_update.py` orchestrates automatic and manual updates, while `MaaFW/api_service/update.py` and `update_job.py` provide the edit-page endpoints and background jobs. Updates build and precheck the new payload in a staging directory, then switch the triggering managed script and same-group views once registration succeeds.

## Update Trigger Paths

- **Pre-run update**: Checks project versions after pre-run checks finish and before account tasks begin. Progress is written to the managed script's task log, and user stops cancel cleanly between stages.
- **Manual check and update**: The edit page calls `POST /api/scripts/maafw/update`. Checks return available versions, while applying an update runs in the backend in the background. Applying updates is rejected while the managed script is running; update jobs perform view switching and environment confirmation under the view reservation. Manual updates can retry versions that previously failed precheck.
- **Reattaching to a manual update**: The edit page subscribes to progress pushes and calls `POST /api/scripts/maafw/update/job` to fetch the progress and result of the current or most recent job. Background updates continue even if the page closes or the request disconnects; job records stay available until the next manual update for that managed script or backend restart.

Run wrap-up can synchronize to an already-prepared group version and run environment confirmation after the summary report is sent. Time spent updating and preparing environments does not count toward account task time limits.

## Orchestration

`run_view_update()` coordinates group updates under the lineage lock. Automatic updates, manual updates, projection healing, and release verification pass parameters through `UpdateSession`.

1. **Sync group version**: Switches the triggering managed script to any available group version first, then checks for updates relative to that group version.
2. **Discover a new version**: MirrorChyan version queries omit the CDK. When a recorded precheck failure exists, retry eligibility is checked before downloading. When GitHub is selected as the source and the project declares a valid repository, versions can fall back to GitHub Releases if MirrorChyan is not configured or fails.
3. **Fetch the release package**: `download_resumable()` selects proxies, mirrors, and caches per configuration while propagating progress and cancellation tokens. GitHub sources can use range deltas.
4. **Build and precheck the payload**: Creates a staging directory from the current payload, applies the release package, verifies entries, and writes to disk. Confirms the new version's runtime environment works before committing shareable files to the blob store.
5. **Register the new payload**: Writes the manifest and advances the channel's group version. Any cancellation or failure before registration discards the staging result.
6. **Switch and synchronize**: Switches the triggering managed script first, then synchronizes idle managed scripts in the same group. Running managed scripts switch on their next pre-run check or normal wrap-up. Group sync status is logged, and environment confirmation runs as needed during switches.

## Download Sources, Credentials, and Proxies

| Design | Why |
| --- | --- |
| Download source, CDK, channel, and timing read only managed-script `Update.*` with no global fallback (`update_credentials.py`); global CDK and channel serve MAS's own self-updates | Each project manages its own credentials and settings |
| When the edit page loads with an empty managed-script CDK, the frontend prefills and saves the global CDK, while the backend still reads only managed-script level ([#843][843]) | Convenient to fill in without blurring backend boundaries |
| Version checks do not validate the CDK; remaining days and CDK status under the edit page's CDK input use a dedicated `/api/scripts/maafw/mirror-cdk` endpoint (`app.services.update_transport.query_mirror_cdk_status()`): first queries the latest version without a CDK, then queries again with the CDK setting `current_version` to that latest version ([#1330][1330]) | When already up to date, the server validates the CDK without issuing a download URL or deducting daily quota; the second call's `current_version` must not be reverted to the local project version. CDKs travel only in the request body, and logs record only whether one is configured |
| Proxy `Update.ProxyAddress` follows global when empty and overrides it when set, covering both update downloads and runtime-environment installation; logs print only "script-level / global / not configured" and never the address ([#943][943]) | Proxy URLs may contain usernames and passwords |
| Only stable and beta channels are exposed; MirrorChyan's alpha channel is intentionally excluded | — |
| GitHub acceleration mirror has only a global `Update.GitHubMirror` toggle (auto / off) sharing its list with the frontend mirror service; applies only to GitHub Release downloads; **never uses mirrors without a SHA-256 digest**; detects mirror error pages as unusable before touching resume state ([#945][945]) | Mirror prefixes break MirrorChyan's signed URLs, and third-party relayed bytes must be verifiable |
| Direct connections retry 5 times with 1 / 3 / 9 / 27 s backoff; no overall timeout, only a 30-second per-read timeout | Slow downloads that are still making progress should not be aborted |

## Full vs. Incremental Packages

| Design | Why |
| --- | --- |
| Only payloads produced by previous updates request incremental packages (manifests record per-file package hashes and payloads are immutable); locally imported payloads, older projection revisions, damaged payloads, and payloads still carrying legacy native-library layouts always request full packages | Delta baselines must be trustworthy |
| Disk entries are enumerated solely through `apply.build_package_plan` (filtered by the projection allowlist); the only bypass is range delta, so changing enumeration rules requires syncing `range_delta._check_layout` | Single place controlling which files hit disk |
| Old files removed by a full package = files from the previous package absent in the new one ∪ imported files inside the new interface's resource directories absent in the new package ∪ shell native-library locations superseded by the new package ∪ files absent in directories taken over wholesale by the new package (native library dirs, `MaaAgentBinary/`, bundled interpreter dir) ([#659][659], [#1144][1144], [#1157][1157]) | Prevents mixing old and new versions while preserving user files outside resource directories (such as presets in `preset/`) |
| Directories equal to, containing, or inside resource directories or agent code directories are not treated as wholesale takeover directories | An agent interpreter may live inside `agent/` |
| **Tradeoff**: packages manually installed by users into a bundled interpreter at import time are removed on the first full-package update, and MAS does not reinstall them | Matches a clean install of the new version |
| Managed files edited by project hot-updates: the new version wins and old contents are archived rather than rejecting the update ([#810][810]) | Some projects rewrite their own data files on every run; failing closed previously trapped them on old versions forever |

## GitHub Range Delta

| Design | Why |
| --- | --- |
| When a full package is needed (always true for GitHub sources), MAS reads the release archive's central directory via HTTP Range, compares entries by CRC32 and size against the current payload, fetches only changed entries, copies the rest locally, and builds a virtual full archive to apply with full-package semantics; output is byte-identical with the same payload ID ([#1157][1157]) | Minor updates often transfer only a few MB instead of hundreds of MB |
| Does not use "source hot-updates" from Git tag source archives | Tested as incompatible with releases: line endings differ, native libraries stay outdated, and `interface.json` would have to be assembled manually |
| Connects directly to `github.com` only, never via mirrors | Range reads have only per-entry CRC32 checks, not a whole-archive digest |
| Any failure (server lacks Range support, CRC mismatch, timeout, or changed entries exceeding 40% of asset size or 3,000 entries) discards the batch and falls back to downloading the full archive (mirrors allowed) in the same update | Conservative by default |
| When mirrors are available as fallback, applies a direct-speed guard: abandons range delta if estimated network time exceeds 120 s or elapsed time exceeds 180 s (`range_delta.RangeSpeedGuard`) | Direct GitHub connections in mainland China are often slower than fetching the full archive through a mirror |
| Environment variable `AUTO_MAS_MAAFW_FORCE_FULL_PACKAGE=1` always forces full-package downloads; intentionally omitted from config and UI | For verification and emergency fallback |
| Core reports `delta`, which the host normalizes externally to `incremental` (displayed as "Incremental" in UI) | Frontend recognizes only full and incremental modes |

## Precheck and Memoization

| Design | Why |
| --- | --- |
| Before committing an update, MAS **actually builds** the runtime environment in the new version's staging directory (not a dry run); if building fails, the new version is discarded and the old version keeps running ([#901][901]) | Prevents "update succeeded, environment fails to build, every subsequent run breaks" |
| Prechecks use a separate isolated venv root, never write to the environment cache, and never run preparation directly against a payload directory | Preparation would otherwise rebuild the single active agent venv |
| Prechecks also run static import checks on the agent: using the project's own interpreter and `ast` to parse unconditional startup imports and verify project-owned modules exist, without executing any project code; third-party packages and conditional imports only warn—**preferring false negatives over false rejections** (`agent_env/import_check.py`, [#1117][1117]) | Previously, missing agent modules in a new release were caught only at run time |
| Precheck memos are keyed by **lineage + target version** and shared across the group: once one managed script fails precheck on a version, no group member auto-downloads it again (`project_update/precheck_memo.py`) | Avoids redundant downloads |
| Failures caused by a missing binding version on PyPI run a lightweight probe before each run and resume updating once available; failures for other reasons are not probed; manual updates always force a retry | — |

## Projection Revisions and Healing

| Design | Why |
| --- | --- |
| Whenever projection rule changes would leave already-registered payloads missing files, `PROJECTION_REVISION` is incremented and a replica of the previous rule set is kept (`projection_legacy.py`) | Already-installed projects do not self-heal because identical versions are not rebuilt |
| During pre-run checks and manual update checks, MAS computes gaps ("kept by new rules, dropped by old rules, absent in payload"), fetches only the missing files from the source directory, download cache, or GitHub Releases (via Range), registers a superset payload of the same version, and switches the whole group (`view_heal.py`, `projection_heal.py`, [#1110][1110], [#1117][1117]) | Restores missing files automatically without manual user patching |
| Checked once per payload; transient failures retry up to 3 times at 6-hour intervals; capped at 45 seconds total, running normally on timeout | Never blocks execution |

## Update Panel

| Design | Why |
| --- | --- |
| Left column shows "Current version" and "GitHub" fields; right side shows the "Update progress" panel with "Check for updates / Update" buttons in the panel header ([#843][843], [#943][943]) | — |
| **The conclusion is the last line of the log in one accent color**, without extra status badges, progress bars, or popups | One place to read the outcome |
| "Check for updates" and "Update" are mutually exclusive | Checking during an update would overwrite the panel |
| Choosing "Do not update" in the auto-update dropdown only stops this managed script from checking online; it still switches when the group advances (UI wording unchanged) | No version pinning; keeps existing UI consistent |
| Every stage (extract, diff, copy, apply, precheck, store, manifest) logs start and end lines plus a total elapsed summary at the end ([#1106][1106]) | Long silent stretches look frozen |
| Busy reasons distinguish "Project is updating", "Script is running", and "Project is in use" | — |

## Limitations

- **Version discovery**: Projects must declare their current version. MirrorChyan updates also require `mirrorchyan_rid`. When using GitHub as the source, releases can also be checked via the project's declared `github` repository.
- **Stopping manual updates**: Background manual updates have no user-initiated stop button, though you can reopen the edit page to view progress.

[659]: https://github.com/AUTO-MAS-Project/AUTO-MAS/pull/659
[810]: https://github.com/AUTO-MAS-Project/AUTO-MAS/pull/810
[843]: https://github.com/AUTO-MAS-Project/AUTO-MAS/pull/843
[901]: https://github.com/AUTO-MAS-Project/AUTO-MAS/pull/901
[943]: https://github.com/AUTO-MAS-Project/AUTO-MAS/pull/943
[945]: https://github.com/AUTO-MAS-Project/AUTO-MAS/pull/945
[1106]: https://github.com/AUTO-MAS-Project/AUTO-MAS/pull/1106
[1110]: https://github.com/AUTO-MAS-Project/AUTO-MAS/pull/1110
[1117]: https://github.com/AUTO-MAS-Project/AUTO-MAS/pull/1117
[1144]: https://github.com/AUTO-MAS-Project/AUTO-MAS/pull/1144
[1157]: https://github.com/AUTO-MAS-Project/AUTO-MAS/pull/1157
[1330]: https://github.com/AUTO-MAS-Project/AUTO-MAS/pull/1330
