# MFW Import and Payload Views

`MaaFW/tools/embedded/embedded_project.py` imports a project source into a payload and materializes a runtime view for each managed script. `MaaFW/tools/core/project_update/` handles projection, payload registration, and file sharing. Existing managed scripts receive new versions by switching views, keeping their project paths and identities stable.

## Import

Both manual imports and pre-run repairs operate on the managed script's view.

- **Locating the project root**: The effective project root comes from a single source: `embedded_project.resolve_maafw_project_root` always returns the view. New code must not read `Info.Path` directly, preventing copies and source directories from being mixed.
- **Handling architecture mismatches**: The sole exception is the pre-run architecture check—when the copy's bundled native library has the wrong architecture, it performs a read-only check on the source directory to decide whether to prompt "reimport" or "switch release package" ([#1024][1024]), giving users who picked the wrong architecture a viable recovery path.
- **Determining the view path**: The view directory name is the first 12 characters of the managed script's UUID with hyphens removed; it is not stored in config and cannot be edited manually. Because Python's bytecode prefix tree repeats the project root twice, longer paths would exceed Windows `MAX_PATH`.
- **Deciding when to import**: `ensure_embedded_copy` runs an import when the view does not exist or when `Info.Path` differs from the source recorded in the import report. If the source directory is gone while the view is healthy, it does nothing—fulfilling the promise that the source directory can be deleted.
- **Rebuilding a missing view**: When both the view and the source directory are missing, it looks up other managed scripts for the same project or registered sources and rebuilds from the latest version on this managed script's channel (`_rebuild_from_group`), likewise ensuring recoverability after source deletion.
- **Validating the source**: The source directory is validated before copying ([#1258][1258]), avoiding HTTP 500 errors and misaligned copy/source state caused by copying before writing config fields.
- **Displaying the copy path**: After import, the directory field locks and displays only the copy path. From that point on, the source path is just a record; create a new managed script to switch to a different project.
- **Reporting import progress**: Import progress is pushed over WebSocket by projected byte count so large projects do not appear frozen.
- **Detecting the managed type**: After import, the project determines the managed type (`flavor.decide_project_config_class`), switching automatically in both directions while keeping the UID unchanged.

## Projection

| Rule | Why |
| --- | --- |
| Allowlist = items declared in `interface.json` (`import`, `resource`, controller extra resources, language files, `agent` / `pretask` directories, dependency manifests, icons, welcome messages) + **remaining undeclared top-level entries up to 64 MB** (`projection.UNDECLARED_KEEP_LIMIT`) | Directories like MaaEnd's `data/` and `locales/` or M9A's `data/activity` are not declared in `interface.json` yet are read by agents at run time |
| Revision 2 rules: name-based exclusions cover only top-level GUI shells, Qt runtimes, and top-level runtime directories (`cache/`, `logs/`, `temp/`, `update/`, `backup/`, etc.); bundled Python, .NET managed libraries, and standard-layout native library directories are excluded only after **content** verification; at arbitrary depth only unambiguous caches and `.pyc/.log/.tmp` are dropped ([#1104][1104], [#1117][1117]) | Older rules excluded directory names recursively and accidentally deleted `battle/runtime/` inside one project's agent code and `UI/` images in another project's resources |
| `debug/` is neither excluded by name nor placed into the payload; on import, non-log state files up to 1 MB are copied from the source `debug/` as view-private files (`_runtime_state_seed`) | Some projects store persistent state they read back (such as account salts or timezone offsets) under `debug/` |
| Bundled native library directories and interpreter directories are carried **as-is**; classification rules do not apply inside them, and only `__pycache__` is stripped | Some agents require a specific Python version, and some native agents load strictly from `<project>/maafw`; CPython ships a `build` subdirectory whose removal breaks pip |
| Binary dependency packages scattered in the root by frozen Python shells are dropped | An agent's `PYTHONPATH` is the project root; partial packages turn into namespace packages that shadow real modules |
| Files for other architectures are never stripped; if none match the host architecture, a notice is shown at the top of the runtime environment log ([#1102][1102]) | Stripped files cannot be recovered; the notice is computed live from the current view so it never lingers after a version change |
| Mixed native libraries (`maafw/` coexisting with `runtimes/<rid>/native`) only trigger a warning and are not deleted ([#1144][1144]) | Cleanup happens during a full-package update based on the new release |

## Payloads and Views

Payloads and views on disk store separate responsibilities.

- **Payload directory**: Payloads live under `data/mfw/.payloads/<lineage>/<version>-<hash8>/` with a matching manifest and are immutable; views live under `data/mfw/<12hex>/` ([#973][973]). This enables single download, single extraction, and single precheck; running managed scripts remain undisturbed; delta baselines cannot be dirtied by agent hot-updates; and reusing a project never leaks runtime state.
- **View directory**: A view is a **complete hard-linked directory tree**, without directory-level junctions. Windows lacks overlayfs, and `data/`, `config/`, and `python/` mix shipped files with runtime writes. Agents resolve paths relative to their working directory or executable, so only a full tree behaves identically.
- **Payload ID**: Computed solely from the version number and file-content hashes, guaranteeing that identical trees always produce the same ID.
- **View path**: Never changes, so environment caches, isolated venvs, interface caches, backups, and runtime-pool reconciliation can all stably key by path.
- **View marker**: `.auto_mas_view.json` is the single source of truth for what is materialized; it is excluded from fingerprints and swapped in atomically with the directory. Because switching happens in a worker thread while config edits occur on the event loop, neither can block on the other; writing the marker into the staging directory first prevents a "new content + old marker" mismatch.
- **Config fields**: Lineage, payload, and version are not written to `ScriptConfig.json`; the only added config field is `Embedded.FollowSource`, keeping internal bookkeeping out of the UI.

## File Sharing and Write-Through Audit

| Design | Why |
| --- | --- |
| Single sharing predicate `projection.is_shared_path`: ≥ 64 KB, not under `config/ debug/ logs/ temp/ cache/ .pycache/`, extension not in `.lock .sha256 .pth .log .tmp`, not inside `*.dist-info`, and not listed among the lineage's private paths | Files modified in place at run time are under 64 KB or live in those directories |
| Matching files are stored in `data/maafw_blobs/<ab>/<sha256>` and NTFS hard-linked into payloads and views, falling back to copying if linking fails | Identical bytes are stored once |
| Shared files are created exclusively via `blob_store.place_fresh`; small files are not pooled; link failures fall back to copying | Hard links have no copy-on-write, so a write-through would corrupt every view |
| Read-only attributes are not used for protection | Read-only flags are shared across hard links and break deletion, replacement, and pip uninstall |
| Every run wrap-up performs a write-through audit (`view_audit.py`): only files with link count > 1 and mtime later than materialization are hashed; confirmed write-throughs are isolated, recorded as private paths, and mark the payload damaged (forcing a full package on the next update) | mtime acts as a cheap trigger and hashing confirms it, making the scan nearly instantaneous |

## Lineage and Groups

| Design | Why |
| --- | --- |
| Lineage key = `mirrorchyan_rid` > `github` > `name` (raw value) | Update discovery already starts from the MirrorChyan resource ID |
| Group = lineage + `Update.Channel`; each channel's latest version **only moves forward and never swaps IDs for the same version** | All members of a group share one payload; importing the same version from two sources never oscillates |
| Only two exceptions allow swapping IDs for the same version: the group's current bundled MaaFramework cannot load on this host while the newly registered one can ([#1024][1024]), or the new payload was built with a higher projection revision ([#1117][1117]) | Allows repairing broken payloads |
| Auto-update mode only decides who checks online: pre-run updates advance the whole group (legacy `AfterRun` config also runs pre-run); managed scripts set to "Do not update" never check on their own, but still switch when the group advances | No version pinning |
| Changing channels = changing groups; the next pre-run switches to the new channel's latest version, **allowing downgrades**; if the new channel has no version yet, the current version stays | Having two channels is the only way to hold one project at two versions |
| Local import: newer than group → advances the whole group; same or older → skipped from registration and the managed script immediately uses the group's current version | Groups do not allow members ahead of or behind the group |
| Remote channel rollbacks are not followed | Latest version only moves forward |

## Switching Views

Switching is handled by `embedded_project.switch_view()`. Upgrades, channel switches, and projection healing all share this entry point.

- **Materializing the new view**: `embedded_project.switch_view` rebuilds the link tree from the new payload in a staging directory → carries over view-private files → lets the new version win for locally modified managed files while archiving old content to `data/maafw_project_state/<view-hash>/local-modified/` → writes the marker into the staging directory first → swaps directories via two renames. The process is declarative, idempotent, direction-agnostic, and takes about 2 seconds.
- **Recovering interrupted switches**: Switch journals live in `data/mfw/.switch/`. At startup, `recover_switches` finishes according to on-disk state rather than rerunning the switch, avoiding misclassifying the entire new tree as local edits after a mid-switch crash.
- **Preserving runtime state**: Paths private to the old view that also exist in the new payload take the payload's copy, except `contracts.RUNTIME_STATE_FILES` (such as M9A's account records), which are carried over as-is so version changes do not reset account records to import day.
- **Handling version-bound state**: An exception to the exception is `contracts.VERSION_BOUND_STATE_FILES` (M9A's hot-update cache), which always takes the new payload's copy when switching payloads. The cache is paired with versioned data tables; keeping the old cache causes hot-updates to be skipped indefinitely.
- **Using the new interpreter**: The `maafw` binding inside a bundled interpreter is not treated as a managed file: it takes the new payload's version, is not archived, and is not carried over as a private file (`_is_maafw_binding_path`), because it was pinned during environment setup rather than edited by the user.
- **Filling missing files**: When the new payload is a strict superset of the old payload with identical shared files (produced by projection healing), only missing files are added and existing view files stay untouched, preventing projects from rerunning hot-updates.
- **Confirming the runtime environment**: Every switch is unconditionally followed by a runtime-environment confirmation; the marker records "confirmed for this payload" only upon success. This keeps multi-minute isolated venv rebuilds out of run time, where users would otherwise see tasks appear stuck.

## Reusing Imported Projects

| Design | Why |
| --- | --- |
| Entry point exists only in the **Create Managed Script dialog**: the first row is "Select local directory", followed by already imported projects matching the selected type | Reuse is a creation-time decision; after import, the directory field is display-only |
| `embedded_project.clone_embedded_copy` materializes a new view from the source managed script's **payload**, without reading the source view, requiring the source to be idle, or copying runtime state | Payloads are immutable |
| `Info.Path` and import records are inherited from the source; managed type follows the project | Pre-run checks compare them to decide whether to reimport |
| "Duplicate managed script" reuses the exact same function | One operation, one implementation |

## Developer Mode

`Embedded.FollowSource`, off by default ([#1294][1294]). Intended for developers editing files directly in a local release package or source repository to fix bugs.

| Behavior | Why |
| --- | --- |
| Switches the group to a private channel `dev:<view-name>` (`effective_channel`); every import advances, with payload versions carrying PEP 440 local segment `+dev` | Other managed scripts for the same project and official updates cannot interfere with it |
| Pre-run checks compute a source signature over the projection allowlist (path + size + mtime, plus content hash for files ≤ 1 MB) and reimport only on changes; unchanged content skips view rebuilds | Avoids full copies on every run |
| When the source cannot be read (locked files, temporarily missing `interface.json`, parse failure), **keeps running the last synced version** with a prominent warning log; import fails only if the view itself is missing | Developer source directories are often mid-edit |
| "Sync now" ignores signatures and forces a reimport; rejected immediately if running, updating, or source is missing | Catches changes whose mtime did not advance |
| Excluded from project updates: auto-updates skip it, and manual checks/updates are rejected | Source directory is the source of truth |
| Leaving a developer payload does not archive "locally modified" files | Those differences came from syncing the source |
| Turning the switch off returns to the original channel group's latest version; duplicating or reusing does not inherit the switch, and cloning a developer-mode script uses the original channel's release payload | Developer payloads do not spread |
| Developer payloads never propagate to normal channels | Same as above |

## Source-Repository Layout

Source-repository projects contain `assets/interface.json`, declare `"child_exec": "python", "child_args": ["-u", "./../agent/main.py"]`, and have `agent/` and `requirements.txt` at the root without `python/` or `maafw/` ([#1294][1294]).

| Design | Why |
| --- | --- |
| Views are materialized in the flat release layout: `assets/` is lifted to the root, while `agent/`, `requirements.txt`, etc. stay at the root | Matches the layout produced when the project packages a release |
| `interface.json` is **not rewritten**: out-of-root agent entry paths are handled by `flattened_entry_path` in `interface/agent_entry.py`, shared by projection and interpreter planning | One fallback fixes everything without maintaining a modified file per payload |
| Only out-of-bounds paths with a runtime mapping (the agent's first Python entry) are allowed; all other out-of-bounds paths are rejected at import time with an explanation | Prevents succeeding at import only to fail with path-traversal errors at run time |
| `child_exec: uv` is unsupported (`describe_uv_agent`) | `uv` creates an environment from the project's own lockfile, where `maafw` may not match the runtime MaaFramework version |
| Source-repository layouts always run in developer mode, with the UI toggle locked on | No official release channel exists |
| Release directories hot-updated from source by CFA (MFW-PyQt6) use the same entry fallback without rewriting `interface.json` on import | Matches CFA's own resolution rules; the first MAS update naturally restores release-package syntax ([#1157][1157]) |

## Migration and Garbage Collection

| Design | Why |
| --- | --- |
| One-time startup migration runs in a background task without blocking the main timer | Blocking the main timer silently skips minute-matched scheduled tasks |
| When adopting legacy copies, payload contents come from the view itself | "Older source + newer view" would register an incomplete payload |
| Latest versions for matching version numbers are chosen only after all copies are adopted, then each group unifies; migration unification does not run separate prechecks | Result is independent of managed-script order, and target payloads come from healthy local copies |
| GC runs only at startup: unreferenced payloads are deleted; projects with zero views left are cleaned up completely; payloads created after process startup are untouched; deletion re-verifies under the lineage lock | Referenced stays, unreferenced is removed—no "keep last N versions" retention |
| Next, shared blobs with link count 1 are deleted, and update download cache files unused for 7 days are pruned | Same as above |
| Data backups exclude `data/mfw`, `data/maafw_blobs`, and `data/maafw_update_cache` | All can be rebuilt, and archiving hard links by path duplicates data |

## Limitations

- In developer mode, files larger than 1 MB are compared only by size and mtime; source-repository layouts also carry up to 64 MB of undeclared files (such as tests and docs) into the view.
- There is no UI entry to "update an existing managed script from a newer local directory"; create a new managed script to import it (if newer than the group, it advances the whole group).

[973]: https://github.com/AUTO-MAS-Project/AUTO-MAS/pull/973
[1024]: https://github.com/AUTO-MAS-Project/AUTO-MAS/pull/1024
[1102]: https://github.com/AUTO-MAS-Project/AUTO-MAS/pull/1102
[1104]: https://github.com/AUTO-MAS-Project/AUTO-MAS/pull/1104
[1117]: https://github.com/AUTO-MAS-Project/AUTO-MAS/pull/1117
[1144]: https://github.com/AUTO-MAS-Project/AUTO-MAS/pull/1144
[1157]: https://github.com/AUTO-MAS-Project/AUTO-MAS/pull/1157
[1258]: https://github.com/AUTO-MAS-Project/AUTO-MAS/pull/1258
[1294]: https://github.com/AUTO-MAS-Project/AUTO-MAS/pull/1294
