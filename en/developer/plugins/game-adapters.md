# Game Adapters

A game adapter connects one kind of device or client to Game Management. The plugin owns discovery, enumeration, and device actions. The host owns config identity, task bindings, device admission, task logs, and the UI. A script uses the binding without parsing a vendor’s private settings or command line.

## Declare an adapter type

In an independent plugin project, define concrete instance and device entries derived from `GameEntry` and `GameDeviceEntry`. Emulator adapters can extend `EmulatorEntry` and `EmulatorDeviceEntry`. Group the two config classes, the controller, and discovery under `GameAdapterPlugin.decl`. The stable `game_type_key` identifies saved instances; changing it requires a config migration.

The following excerpt takes the type declaration from the MuMu adapter and omits its lifecycle hooks. Export the plugin class as `PLUGIN` in the package’s `__init__.py`.

```python
from auto_mas_core.adapters.game import GameAdapterPlugin, GameTypeDecl

from .config import MuMuDevice, MuMuGame
from .control import Control, search
from .discovery import resolve_installation


class Plugin(GameAdapterPlugin):
    game_type_key = "mumu"
    decl = GameTypeDecl(
        game_class=MuMuGame,
        device_class=MuMuDevice,
        control_class=Control,
        search=search,
        resolve_installation=resolve_installation,
    )
```

Place config and field hints in `config.py`, instance operations in `control.py`, and installation probing in `discovery.py`, as appropriate for the plugin. Keep vendor drivers within their adapter. MuMu and LDPlayer each implement the applicable capabilities formerly grouped under Emulator 2.

## Discover and import installations

Make `search()` return an async result containing only candidate entries of its own type. Return an empty sequence if it finds none. Probe the installation without persisting entries or starting a device. Move synchronous blocking probes off the event loop. The host calls each enabled adapter, filters mismatched and already registered installations, then caches candidates for the user to add. Repeat discovery if a candidate expires. An optional `resolve_installation(path)` resolves an old path to a recognized installation, primarily for configuration upgrade. Return `None` for paths it cannot resolve.

If MuMu is installed but discovery is empty, check whether the adapter is installed, enabled, and registered. Check for an existing entry or a path the current discovery code cannot detect. The host logs discovery failures separately from empty results. If a path cannot be found automatically, add it manually according to the adapter’s configuration requirements.

## Implement the controller and actions

- Implement refresh and device actions with `GameControl`. For emulators, use `EmulatorControl` for ADB, settings, and desired device state. Implement vendor maintenance, updates, recovery, and process operations within the adapter. Use controller subscription hooks to acquire and release resources; validate device use against the runtime ticket.
- Give each device action its own [trigger](./fundamentals/schema.md#triggers), and expose availability through backend hints. A trigger deduplicates the same field; the controller still checks admission for different actions on one physical device. Call the controller’s `runtime_changed()` when device usage or availability changes.
- The host renders tabs and device lists by type. Expose initialization, settings, maintenance, or window controls as applicable. The frontend renders the supported controls from their individual field hints. See [Configuration UI Synchronization](./fundamentals/frontend.md#configuration-ui-synchronization).

`GameDeviceEntry` already provides `action.open`, `close`, `show`, `hide`, `minimize`, and `maximize`. Their handlers call the controller with the entry UUID and `TRIGGER_TICKET`. A plugin can inherit this wiring and implement the controller methods. For example, the Windows Process adapter inherits those actions and uses `initialize_devices()` to supply a stable device entry for the configured executable. It only accepts manual registration because there is no reliable list of arbitrary installed games to discover. A manual action uses the special ticket and does not require a managed task subscription. The controller still enforces its own action limits; a task holding a subscription is not by itself a reason for the base UI to disable all buttons.

### Host integration and debugging

All game adapters use the same host API. When maintaining a host page or tracing a request, use these endpoints:

| Request | Responsibility |
| --- | --- |
| `POST /api/games/types`, `POST /api/games/get` | Read registered types and game, device, and UI snapshots. |
| `POST /api/games/search`, `POST /api/games/import` | Discover candidates and persist the selected candidate by `gameId`. |
| `POST /api/games/create` | Create a manually configured game of the selected type. |
| `POST /api/games/binding/options` | Read candidates for managed task and account bindings. |
| `POST /api/config/triggers/write` | Invoke a device action. Use the device entry UUID and its exact field, such as `action.open`. |

The host flow runs from `app/api/games.py` through `app/core/game_manager.py` into the registered adapter. Task binding and finalization live in `app/task/game.py`. Use the shared API for management UI instead of creating vendor-specific routes for standard actions.

## Bind, reserve, and restore devices

A managed task may bind a default game and device. Each account may override it. The managed task owns the game ticket, and its accounts borrow it. The host and controller check device admission before use; the task log reports a wait for availability. When work ends, try restoring temporary settings before releasing the ticket and configuration locks. Log recovery failures and keep the required material for a later retry. The managed task may report failure while completed accounts keep their own successful results and counts. Use the shared subscription boundary for device exclusivity and cleanup on plugin unload or reload.

## Default installation, upgrade, and verification

On the first launch with a fresh configuration, the host attempts to install MuMu, LDPlayer, General Emulator, and Windows Process adapters once. Users can later uninstall them. ModAVD is installed on demand. An existing configuration uses its own upgrade flow: prepare needed plugins, create or reuse entries by normalized installation path, then map old managed tasks and accounts to their game and device. Several paths may require several game entries. Handle unrecognized paths through upgrade diagnostics and user selection.

The one-time attempt is decided before the data store is initialized: the data database must not exist and the configuration directory must be empty. The data database created during startup prevents another automatic attempt on later launches. The four attempts share a 90-second startup budget; if an adapter is missing afterwards, install it from the plugin market. Upgrades use a separate migration record (`.game-migration-v1.json`) and may install ModAVD if old data requires it.

Start with [source development](./fundamentals/develop.md). Verify discovery, import, refresh, device contention, cancellation, and recovery; the same installation should not create duplicates and reload should preserve bindings. Move the same-ID source out of the scan path when testing the [published package](./fundamentals/publish.md). Use a copy of config and data to check upgrades with multiple script paths, managed defaults, and account overrides.
