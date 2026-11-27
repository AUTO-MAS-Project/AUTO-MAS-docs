# Plugin Lifecycle

The host coordinates plugin loading, enablement, disabling, reload, uninstall, and shutdown in `app/core/plugin/manager.py`. Plugin classes override asynchronous hooks defined by `BasePlugin` as needed. The host calls the implementations on the plugin instance.

This page uses plugins installed from the market as its example. To remove a local source plugin, manage its source project on disk. Move that project out of the host's scan paths before using the market to install or update a distribution with the same plugin ID.

- **Lifecycle hooks.** The `BasePlugin` implementations are empty. A plugin can override an empty hook directly.
- **Upstream and downstream services.** Upstream providers supply the services listed in the current plugin's `requires`. Downstream dependents list services supplied by the current plugin in their own `requires`.

## Load

The host uses `app/plugin/runtime/loader.py` to import the plugin package and obtain the class it will instantiate during enablement.

1. **Import the plugin package.** The host uses the import package name and source in the registry entry to locate the top-level Python package and execute its `__init__.py`. The plugin exports its class as `PLUGIN` in that file.
2. **Validate the plugin.** The host checks that `PLUGIN` is a subclass of `BasePlugin`. If the plugin exports `PLUGIN_API_VERSION`, the host checks that the major protocol version matches.

- **Load results**
  - `disabled` indicates a successful load. The plugin can be enabled, reloaded, or uninstalled. The host retains the plugin class and records the service names declared in `requires` and `wants`.
  - `fault_load` indicates an import or validation failure. The plugin can be uninstalled. The host records the error for display in the Plugin Center.

## Enable

The host creates an instance of the loaded plugin class and registers the capabilities the plugin provides.

1. **Create the instance.** The host checks that the upstream providers are loaded. If the plugin declares `config_class`, the host builds the plugin configuration before activation. It passes the registry entry and configuration to the constructor, with `None` as the configuration argument when the class declares no configuration. It then binds the service resolver and background task group.
2. **Prepare to enable.** The host calls `pre_enable()` on the plugin instance. The plugin can check its runtime prerequisites, such as external programs or environment settings. Put work that needs required services or active plugin configuration in `on_enable()`.
3. **Enable upstream plugins.** The host recursively enables providers for the services listed in `requires` and reuses upstream plugins that are already enabled. It then activates the current plugin's configuration if the plugin declares one.
4. **Enable the plugin.** The host calls `on_enable()`. The plugin can read its configuration, call required upstream services, create clients, or subscribe to signals.
5. **Register capabilities.** The host registers the plugin's services, HTTP routes, and adapter types.

- **Enable results**
  - `enabled` indicates successful enablement. The plugin can be disabled, reloaded, or uninstalled. The host retains the instance, and callers can use its registered capabilities.
  - `fault_runtime` indicates an enablement failure after `pre_enable()` succeeds. You can retry enablement, reload, or uninstall. The host withdraws registrations made during the operation, cleans up the instance, and records the error.
  - **Preparation failure.** If checking dependencies, creating the instance, or calling `pre_enable()` fails, the host reports the error and retains the previous state. You can retry enablement after fixing the error.

## Disable

The host withdraws the plugin's capabilities and releases its instance. The loaded class is available for the next enable.

1. **Prepare to disable.** The host pauses new calls to the current plugin and its downstream dependents and waits for in-flight work. It checks whether any adapter configuration about to be released is in use. The host then calls `pre_disable()`, where the plugin can check that operations such as configuration sessions have finished.
2. **Disable downstream plugins.** The host recursively disables enabled downstream dependents along the required service relationships.
3. **Disable the plugin.** The host removes the instance's background task group and calls `on_disable()`. The plugin can close clients, external processes, or subscriptions established during enablement.
4. **Unregister capabilities.** The host saves and releases the plugin configuration, unregisters services, HTTP routes, and adapter types, and releases the instance.

- **Disable results**
  - `disabled` indicates successful disabling. You can enable, reload, or uninstall the plugin.
  - `fault_runtime` indicates a failure during disabling. You can retry enablement, reload, or uninstall. The host runs failure cleanup and records the error.
  - **Preparation failure.** If the adapter configuration check or `pre_disable()` fails, the host retains the previous state. You can retry disabling after fixing the error.

## Reload

The host imports the current plugin class again. Enabled downstream dependents are disabled during the reload and then enabled again in dependency order.

1. **Prepare to reload.** The host pauses calls to the current plugin and its downstream dependents, waits for in-flight work, and checks whether adapter configuration is in use. It calls `pre_reload()` on an existing instance so the plugin can check whether its current work permits a reload.
2. **Disable downstream plugins.** The host recursively disables enabled downstream dependents and records the affected plugins and their dependency relationships.
3. **Reload the plugin.** If an instance exists, the host calls `on_reload()` on it so the plugin can save state across the reload. If the plugin was enabled before the reload, the host disables it. The host then reimports the package, validates the entry point, and records the newly loaded class. It reenables the plugin if it was enabled before the reload.
4. **Restore downstream plugins.** The host restores the downstream plugins disabled by this operation in upstream-to-downstream order. If a downstream plugin fails to recover, the host marks it as `fault_runtime` and continues with the other branches.

- **Reload results**
  - `enabled` indicates a successful reload that restored the plugin to an enabled state. You can disable, reload, or uninstall it.
  - `disabled` indicates a successful reload with the plugin in a disabled state. You can enable, reload, or uninstall it.
  - `fault_runtime` indicates a failure during reload. You can retry enablement, reload, or uninstall. The host runs failure cleanup and records the error.
  - **Preparation failure.** If the adapter configuration check or `pre_reload()` fails, the host retains the previous state. You can retry reloading after fixing the error.

## Uninstall

The host releases plugin runtime resources and removes the distribution, plugin configuration file, and registry entry. The Plugin Center shows which adapter configurations may be affected.

1. **Prepare to uninstall.** The host pauses calls to the current plugin and its downstream dependents and waits for in-flight work. It calls `pre_unload()` on any existing instance so the plugin can check that files it manages can be cleaned up.
2. **Unload downstream plugins.** The host follows required service relationships to recursively unload downstream plugins, releasing their runtime resources and removing their registry entries. Distribution and configuration file removal applies to the selected plugin.
3. **Uninstall the selected plugin.** The host retains a reference to any instance present at the start of the operation. If the plugin was enabled, the host disables it first. If the host retained an instance, it then calls `on_unload()` on it. The plugin can remove resources needed only while it is installed, such as its own caches. Finally, the host releases the plugin class, removes the distribution and configuration file, and removes the registry entry.

- **Uninstall results**
  - **Success.** The plugin can be installed again from the market.
  - `fault_load` indicates an error releasing runtime state or removing the distribution or configuration file. The host records the error in the retained registry entry. You can retry uninstalling.
  - **Preparation failure.** If `pre_unload()` fails, the host reports the error and retains the previous state. You can retry uninstalling after fixing the error.

## Failure Cleanup

When a lifecycle operation enters failure cleanup, the host rolls back registrations and releases resources for the affected plugin.

1. **Roll back host registrations.** The host unregisters services and HTTP routes, releases plugin configuration, and removes the instance's background task group.
2. **Clean up plugin resources.** When cleanup requires the original instance, the host rolls back any adapter registrations, then calls `on_fail_cleanup(during, lifecycle_context, exc)`. The plugin can release resources it acquired before the operation failed.
   - `during` identifies the lifecycle stage that failed.
   - `lifecycle_context` describes the operation's lifecycle context.
   - `exc` is the exception that triggered cleanup.
3. **Release the instance and record the error.** The host releases the plugin instance and writes the error to its registry entry.

- **Cleanup result.** The host records the fault state for the operation. See each operation's results for the available next actions.

## Shutdown

The host shuts down enabled plugins in dependency order and saves the configuration for its next startup.

1. **Stop accepting calls.** The host closes service and HTTP call entry points and waits for in-flight calls to finish.
2. **Shut down downstream plugins first.** The host processes downstream dependents before upstream providers. For each enabled plugin instance, it removes the background task group, then calls `on_teardown()`. The plugin can close external processes, connections, and subscriptions.
3. **Save configuration.** The host saves configuration as part of backend shutdown and preserves the desired enabled state in each plugin's registry entry.

- **Shutdown results**
  - **Cleanup completed.** At the next startup, the host loads plugins again and restores the saved enabled selections.
  - **Cleanup failed.** If a plugin's `on_teardown()` raises an error, the host logs it and continues with the other plugins.
