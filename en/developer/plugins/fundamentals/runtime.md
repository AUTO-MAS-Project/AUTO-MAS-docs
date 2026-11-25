# Plugin Host and Runtime

A package declares its capabilities. The host decides when to load them, make them callable, and withdraw them. Start with the [minimal plugin](./index.md), then use this page to understand how source projects, distributions, installation environments, and runtime instances fit together.

## From project to runtime instance

In a source checkout the host scans `plugins/<id>/`; with Runtime, user source plugins stay in the installation root’s `plugins/` and installed distributions are under `plugins/pypi/site-packages/`. The SDK comes from the running backend's source tree (`repo/plugins/auto_mas_core/` under Runtime). This is why switching backend versions does not replace the user’s plugin directory.

| Identity | Example | Purpose |
| --- | --- | --- |
| Plugin ID | `mumu_adapter` | Registry identity, translation namespace, and candidate selection |
| PyPI distribution name | `auto-mas-mumu-adapter` | Installation, versions, and dependency resolution |
| Python import package | `mumu_adapter` | The package the host imports to read `PLUGIN` |
| Game type key | `mumu` | Identifies saved game configurations of that type |

1. The host scans source projects and installed distributions. A source project needs `pyproject.toml` and `src/` with a single regular top-level package. Distribution discovery reads the `auto-mas-plugin` and `auto-mas-id:<id>` metadata markers. Do not infer the import name from the distribution name.
2. Discovery updates the registry from metadata before plugin code is loaded. A source project takes precedence over a distribution with the same ID. When verifying a wheel, move the source project out of the scan paths, restart, and confirm that the registry selects the installed package.
3. On load, the host imports the package and reads the class exported as `PLUGIN`. The plugin distribution version, declared SDK dependency range, and host `PLUGIN_API_VERSION` are separate. Follow the [development guide](./develop.md) for declarations and package resources, then enable the plugin in Plugin Center.

## Enable, disable, and reload

The host binds infrastructure and performs prechecks, enables required service providers, and activates configuration before running the plugin’s enable hook. Adapter types must be registered before their instances are activated so the configuration framework can use the concrete classes. Services and endpoints become available once enabling succeeds. See [Core Capabilities](./index.md) and [Services](./service.md).

On teardown, the host rejects new managed calls and waits for in-flight work to settle before removing resources. HTTP endpoints, services, and managed adapter calls share this lifecycle. A synchronous call remains tracked until its worker returns.

Put environment checks in `pre_enable` and allocate business resources in `on_enable`, covering failure cleanup, disable, and process exit. If a managed call needs its own plugin to stop, arrange a separate lifecycle request so the call does not wait for itself to drain.

The call gate tracks in-flight calls rather than scheduling the plugin's business work. Blocking synchronous HTTP handlers run in a worker thread. Cancelling an HTTP request does not release its registration until that worker exits. In a supported self-disable path the gate excludes the initiating call from the drain and still waits for other calls; an independently spawned task has its own identity.

Reload coordinates affected plugins in dependency order and registers replacement types while preserving config identity. Ordinary disable and uninstall may remove configurations owned by an adapter type. Plugin Center shows the impact first, and the host rejects removal when affected configs are locked. Use a disposable profile to verify enable, reload, disable, and uninstall. See [Configuration](./schema.md) and [Events](./event.md) for committed changes and UI refreshes.

| Operation | What the host changes | What the plugin author implements |
| --- | --- | --- |
| Load | Imports the package, validates the exported class and reads its declarations. | Keep module import limited to declarations. Acquire resources in lifecycle hooks. |
| Enable | Resolves required services, activates config, then exposes services and HTTP routes. | Check requirements in `pre_enable`; start business resources in `on_enable`. |
| Disable | Drains calls and tasks, unregisters capabilities, and releases the running instance. | Close resources in `on_disable`. |
| Unload | Runs the relevant teardown, then removes the loaded class and module references. | Use `pre_unload` and `on_unload` for unload-specific checks and cleanup. |
| Reload | Coordinates downstream teardown, loads the replacement class, and restores the dependency tree. | Validate and clean up the old instance in the reload hooks. Recreate runtime resources on enable. |

Inspect the registry state and error after failure. A load fault points to package structure, SDK compatibility, or import. A runtime fault points to enable, disable, or reload. Use `on_fail_cleanup` for the business resources you already acquired. The host rolls back its own registrations. Process exit calls `on_teardown` to close resources while keeping the configured enable state for the next launch.

## Market, installation, and updates

The market catalog is independent of the local registry. It tries the data site, official PyPI, then mirror indexes. The first complete result replaces the catalog. If they all fail, the catalog stays unchanged. Catalog changes do not install, disable, or downgrade a local package. If a release is missing, check its metadata and whether the index has picked it up. See [Publishing Plugins](./publish.md).

The installer resolves the plugin environment’s dependencies together. When loaded dependencies need changes, the host stages a candidate environment and waits for a backend restart. It completes the handoff after the new process confirms successful loading and can recover the old environment on failure.

In Runtime mode, the SDK follows the backend source, while installed user plugins live at the installation root and survive backend updates. Desktop application updates, Runtime backend updates, and plugin installation have separate lifecycles. Do not replace host files or change the shared installation environment from plugin code. See [Plugin Center Actions](./frontend.md#plugin-center-actions) for status and restart coordination.

### Compatibility checks and startup after an update

- **Plugin updates:** resolve against the current host and its locked dependencies, preserving constraints from installed plugins, including disabled ones. The plugin environment must not overwrite host dependencies or the host SDK.
- **Application updates:** preflight uses the target version's `pyproject.toml`, `uv.lock`, and `.python-version`. A compatible combination may include plugin upgrades. On startup, the host scans metadata without importing old plugin code, prepares and validates the joint environment, then loads and enables plugins according to their saved intent. If preparation fails, preserve the old environment and config for recovery; a successful preflight alone does not mean the new host has started.
- **Runtime updates:** replacing the Runtime executable has its own version and integrity flow and can run without the backend. Runtime supervises the backend; plugin dependency resolution belongs to the backend host.

Under Runtime, core health and business readiness are separate: health can report the process before plugin and configuration preparation finishes. The frontend waits for `businessStatus` before activating business features.

## Background tasks, downloads, and prefetch

Use `self.background_tasks` for plugin-owned asynchronous work. Use [configuration triggers](./schema.md#triggers) for work initiated by settings buttons; the configuration manager owns that execution. Keep blocking work off the event loop. Use the task handle described in [Core Capabilities](./index.md) to wait for or take a plugin-owned result. Keep long-lived tasks within the host lifecycle.

Use `auto_mas_core.services.Downloads` for shared file downloads: construct `DownloadRequest(url=...)`, acquire a reference with `Downloads.request(request, owner=...)`, read the completed snapshot with `await Downloads.wait(request)`, and release the reference with `await Downloads.release(request.cache_key, owner=...)`.

The default resource key is the original URL; use a stable, plugin-namespaced `key` when a signed URL changes. Content identity is resolved in order of `rid`, then `content_hash`, then none—supply a distinct identity when changing the content behind one key. Set `downloader` when a resource needs a specific host downloader. Only call `wait` if `request` accepted the reference, and check the snapshot’s state before using its path.

A script adapter can declare an async `ScriptTypeDecl.prefetch_check` that computes a `list[DownloadRequest]` from script configuration. Return an empty list when there is nothing to prefetch. L2 manages the references. L3 checks the requirements again under the config lock and can read the resource-key-to-path mapping from the task’s `prefetch_paths` argument. The mapping is not automatically forwarded to L4. See [Standalone Adapters](../script-adapters/standalone.md) for script integration.
