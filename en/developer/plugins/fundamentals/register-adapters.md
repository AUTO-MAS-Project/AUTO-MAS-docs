# Register Adapter Types

Adapter plugins declare a type key and a `decl` attribute on their plugin class. The host uses the type key to identify the adapter type and reads `decl` for the configuration classes and implementations it needs to manage the type.

## Game Adapters

Game adapter plugins provide installation discovery, game and device configuration, and device control. The host uses these capabilities to register installations and manage devices in game clients and emulators.

See [Declare an Adapter Type](../game-adapters.md#declare-an-adapter-type) for the integration steps.

```python
decl = GameTypeDecl(
    game_class=<game configuration class>,
    device_class=<device configuration class>,
    control_class=<device controller class>,
    search=<async installation discovery function>,
    resolve_installation=<legacy installation path resolver or None>,
)
```

## Managed Script Adapters

Managed script adapters provide configuration for managed scripts, accounts, and optional plan tables, along with a task expander. The host uses the declaration to create managed script configurations and expand tasks for the selected run mode during scheduling.

See the declaration example in [Standalone Adapters](../script-adapters/standalone.md#skeleton-of-the-plugin-repository).

```python
decl = ScriptTypeDecl(
    script_type=<managed script configuration class>,
    user_type=<account configuration class>,
    expander_class=<task expander class>,
    plan_type=<plan table configuration class or None>,
    supported_modes=<tuple of supported run modes>,
    expect_builder=<emulator requirements builder or None>,
    prefetch_check=<async download prefetch check or None>,
    activity_game=<game ID for activity display and reminders or None>,
    game_kinds=<tuple of compatible game target types>,
    supports_mod_avd=<whether ModAVD is supported>,
)
```
