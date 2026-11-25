# MFW Design Tradeoffs

MFW isolates project state with a dedicated view per managed script and coordinates versions by project lineage and update channel. The constraints below guide maintenance across import, updates, execution, and flavor extensions. When changing any of these behaviors, document the problem being solved and the affected scenarios in the change description.

## Import, Copies, and Projection

- **Runtime views**: Every managed script gets its own directory named after the first 12 characters of its UID, reducing the risk of hitting Windows path-length limits on nested paths. After import, accounts and tasks run strictly against this view while the source directory remains under user control. See `embedded_project.resolve_maafw_project_root()`.
- **Shared files**: Payloads and views create hard links only for files matching the sharing rules. Potentially writable files keep private copies, and any write-through that does occur is caught by the wrap-up audit. Shared files remain replaceable and reclaimable. See `projection.is_shared_path()` and `view_audit.py`.
- **File projection**: Project-declared resources and dependencies are prioritized, alongside bounded undeclared files that agents may read at run time. Only verified shells and their runtimes are excluded, while bundled Python and MaaFramework libraries are preserved. See [MFW Import and Payload Views](./import-and-payload.md).

## Version Synchronization and Updates

- **Consistent group versions**: Managed scripts sharing a lineage and channel use the same registered version. "Do not update" means a managed script does not check online itself, yet it still synchronizes when the group advances. Switching channels (such as returning to stable) may move to an older version.
- **Stage before commit**: Downloading, extraction, and environment prechecks happen in a staging area, and managed scripts adopt the new version only after registration completes. Versions that fail precheck are memoized to avoid repeated downloads. See [MFW Project Updates](./update.md).
- **Verifiable downloads**: Range deltas reconstruct the release archive's exact contents, falling back to full downloads whenever verification fails or fetching the full archive is faster. Third-party mirrors participate only when a full-package digest is available.
- **Garbage collection and recovery**: GC relies on active references and re-verifies at startup. View switches preserve account runtime state while refreshing version-bound caches from the new version. Switch journals allow startup recovery to finish interrupted directory swaps.

## Tasks and Devices

- **Task instances**: Account snapshots preserve queue order, enabled state, and per-instance options. The same project task can have multiple instances, while periodic completion flags are tracked by task name. Tasks removed from the project remain in the queue until deleted by the user, and are skipped and logged at run time.
- **Runtime signals**: Whole-account and per-task time limits are handled cooperatively by the host and worker. Experimental loop-guard detection relies on actual node and action telemetry. Maintenance and client-update signals come from public declarations in the project.
- **Retries after failure**: Non-periodic tasks rerun by default; flavors declare tasks that have already succeeded and can safely be skipped on retry. Each attempt keeps its own log.
- **Client lifecycle**: `DirectExe` and `URL` close clients launched by the current run. `AttachOnly` fallback shutdown runs only on final wrap-up and is constrained by `Game.CloseMode` and process start times. See [MFW Runs](./run.md#pc-game-launch-and-shutdown).

## UI and Flavors

- **Shared pages**: Managed-script and account pages share import, queue, and progress panels. Flavors start with diff configuration and slots, and preserve shared prop and event contracts when replacing sections. See [MFW Frontend and Interaction](./frontend.md).
- **Execution semantics**: Flavor hooks handle project recognition and project-specific tasks or options, while the generic engine continues to provide scheduling, notifications, updates, and wrap-up. See [Flavor](./flavor.md).
- **Secret protection**: The editor UI, logs, and issue reports each handle password fields according to their respective data flows. Projects with password inputs export only run logs carrying the redaction marker. See [MFW Shell Import and Issue Reports](./shell-import-and-issue-report.md).
