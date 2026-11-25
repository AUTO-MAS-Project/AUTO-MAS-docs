# MFW Architecture

MFW lives under `app/task/MaaFW/` and imports, updates, and runs MaaFramework projects according to their project interface. The managed types `MaaFWConfig`, `M9AConfig`, and `MSSConfig` are each registered in `run_types` inside `MaaFW/registration.py` and share `MaaFWEmbeddedManager`. The task queue saved on the account page serves as the task source at run time.

## Code Responsibilities

1. **Handling API requests**: `MaaFW/api.py` defines the routes, mounted uniformly by `app/api/scripts/__init__.py`. Endpoints parse inputs, call service modules, and build responses. View import, preview, updates, environment preparation, and shell-config import are implemented in modules under `MaaFW/api_service/`.
2. **Host integration**: `MaaFW/embedded_manager.py` and `MaaFW/tools/embedded/` handle config locks, managed views, scheduling, notifications, and run history. `MaaFW/tools/restore_service.py` integrates with config backup and restore. Project-specific behavior is invoked through the [Flavor](./flavor.md) mechanism defined in `tools/embedded/flavor.py`.
3. **Core engine logic**: `MaaFW/tools/core/` handles interface parsing, task snapshots, run plans, interpreter management, project payloads, and updates. The worker runs inside an isolated virtual environment whose `app.*` import closure is restricted to `tools/core/`, and whose third-party dependencies are limited to `maafw` and `BASE_RUNTIME_PACKAGES`. These constraints are enforced by `tests/task/test_maafw_core.py`.
4. **User interface**: The MFW managed-script page, account page, and flavor pages under `frontend/src/views/EditView/` present import, configuration, and run status. Flavors are registered in `frontend/src/composables/useMaaFWFlavor.ts`. Electron issue reports are exported by `frontend/electron/services/maafwIssueReportService.ts`.

API services depend unidirectionally on the engine and host integration modules. The updater runs in the host process, while core engine code used by the worker stays self-contained. The worker's working directory is the application root directory; `PYTHONSAFEPATH` is set only for the worker, so project agents still resolve sibling modules according to project settings.

## Single Import Flow

1. **Validate the source directory**: The managed-script page calls `/api/scripts/maafw/embedded/reimport`. The host first validates the source directory using the `Info.Path` validation rules.
2. **Project files**: Files are selected according to the project interface and projection rules, filtering out recognizable shells and placing required runtime state files into the view's private area. Source-repository layouts use a local private channel instead.
3. **Register payload and switch view**: Files are committed into a payload, the current managed script's view switches to the group version, and idle managed scripts in the same group synchronize. Import progress is pushed over WebSocket.
4. **Detect managed type**: The first flavor in registration order that claims the project is selected; otherwise generic MFW is used. When the type changes, it is swapped in place while preserving the managed script's UID.

See [MFW Import and Payload Views](./import-and-payload.md) for projection rules, on-disk layout, and reuse flows.

## Single Run Flow

1. **Prepare the managed script**: `MaaFWEmbeddedManager` reserves the view, ensures the project is imported, and runs group synchronization and projection healing. After locking the managed-script and account collections, it accesses runtime configs through a lease ticket.
2. **Prepare version and environment**: Checks for updates according to managed-script settings. If switching versions or environments requires re-confirmation, that confirmation runs before account execution. See [MFW Project Updates](./update.md).
3. **Execute account tasks**: `tools/embedded/runner_task.py` builds the run plan from the account snapshot, prepares the device, and starts the worker. The worker connects to the controller, dispatches tasks one by one, and reports progress and results. See [MFW Runs](./run.md) for flavor hooks, time limits, and retry rules.
4. **Persist results and release resources**: Records account status, logs, failure screenshots, and history entries. The executor saves runtime state and releases locks, then runs view maintenance and summary reporting. Normal wrap-up may also advance to a ready group version.

## Concurrency and Locking

- **View reservation**: `try_reserve_project_path()` in `tools/embedded/project_path.py` provides in-process exclusive access to a single view. Running, updating, environment preparation, and switching are all governed by this reservation; callers skip or warn when acquisition fails.
- **Lineage lock**: `DurableFileLock` in `tools/core/project_update/state.py` serializes updates, registration, and garbage-collection checks for the same project across processes.
- **Config lock**: Locks the managed-script and account configurations while a managed script is running so UI edits cannot overwrite runtime state. It is released under the scheduler's ticket rules after resources are restored.

Each managed script owns its own view, isolating private runtime files written by projects. Hard links are used only for files proven shareable; see [MFW Design Tradeoffs](./tradeoffs.md) for write-through auditing.
