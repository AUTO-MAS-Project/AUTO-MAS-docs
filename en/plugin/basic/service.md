# Service System

The service system provides low-coupling capability sharing between plugins. Plugins cooperate through service names instead of hard-coding references to specific plugin classes.

This document explains the current AUTO-MAS service mechanism, lifecycle behavior, and minimal usable patterns.

## Service Model

### Core Concepts

- Service name: a string key, such as `clock`, `store`, or `auth`.
- Service declaration: declares "I provide this service name", but the service may not be ready immediately.
- Service assignment: writes the service instance into the registry, making the service readable.
- Hard dependency (`needs`): if missing, the instance will not start.
- Soft dependency (`wants`): if missing, startup is still allowed.

### Declaration and Assignment Are Separate

Service registration uses two steps:

1. Declaration: `provide(name)` only registers the service slot and provider.
2. Assignment: `set(name, value)` writes the instance value and triggers dependency recalculation.

This avoids timing issues where a service is used before it is ready.

## Plugin Entry Points

Plugins can use service features directly through `ctx`, or through the equivalent `ctx.service` interface.

### Quick Methods

- `ctx.provide(name)`: declare a service.
- `ctx.set(name, value)`: set a service value.
- `ctx.get(name, default=None)`: read a service value.
- `ctx.inject(needs=None, wants=None, ready=None)`: dynamically inject dependencies and trigger a callback when they are satisfied.

### Service Methods

- `ctx.service.provide(name)`
- `ctx.service.set(name, value)`
- `ctx.service.get(name, default=None)`
- `ctx.service.inject(needs=None, wants=None, ready=None)`
- `ctx.service.miss()`: returns the current instance's missing hard dependencies.

## Static Dependency Declarations

Plugin classes support static declaration fields:

- `provides`: services provided by the current instance.
- `needs`: services required for startup.
- `wants`: optional services.

Declaration values support two forms, which are normalized internally:

- String: `provides = "clock"`
- List: `provides = ["clock", "ticker"]`

Example:

```python
class Plugin:
    provides = "clock"
    needs = ["store"]
    wants = "notify"

    def __init__(self, ctx):
        self.ctx = ctx
```

Notes:

- The current development version does not provide compatibility for the old static `inject` layer.
- Static declarations only recognize `provides`, `needs`, and `wants`.

## Startup Order and Dependency Policy

### Load Order

The loader first scans enabled instances and parses `provides/needs/wants`, then builds a dependency graph and performs topological sorting.

::: warning What happens if service names duplicate?

Only one provider is allowed for the same service name.

- If another instance tries to declare an existing service name, that instance startup is rejected and an error is logged.
- This error does not interrupt the entire plugin system startup process, but the instance enters a failed state.

:::

Sorting goals:

- Providers start before consumers.
- If a dependency cycle is detected, a warning is logged, and the remaining instances continue loading in their original order.

### Missing Hard Dependencies

When an instance has missing `needs`:

- The instance is skipped and marked as failed.
- Other instances continue starting. The entire plugin system is not interrupted.

This follows the policy that one failed instance should not bring down the whole system.

## Service Changes and Automatic Recalculation

The registry triggers two-phase notifications on `set` / `drop`:

- `before`: tells dependents to prepare for deactivation.
- `after`: tells dependents to try reloading.

The loader listens to both phases and performs recalculation:

1. Collect affected instances that are currently active.
2. Unload those instances first.
3. Try to reload them according to recorded configuration and dependency declarations.

## Undeclared Access Warnings

When `ctx.get(name)` reads a service, it checks whether the current instance has declared that service through `provides/needs/wants` or dynamic `inject`.

- Declared: read normally.
- Undeclared: log a warning, but do not throw immediately.

This is a progressive constraint strategy that helps old plugins migrate smoothly.

## Minimal Examples

### 1) Plugin Providing a Service

```python
class Plugin:
    provides = {"clock"}

    def __init__(self, ctx):
        self.ctx = ctx

    async def on_start(self):
        self.ctx.set("clock", {"now": "2026-04-25T12:00:00+08:00"})

    async def on_stop(self, reason: str):
        _ = reason
```

### 2) Plugin Depending on a Service

```python
class Plugin:
    needs = {"clock"}

    def __init__(self, ctx):
        self.ctx = ctx

    async def on_start(self):
        clock = self.ctx.get("clock", {})
        self.ctx.logger.info(f"clock={clock}")

    async def on_stop(self, reason: str):
        _ = reason
```

### 3) Dynamic Injection Callback

```python
class Plugin:
    def __init__(self, ctx):
        self.ctx = ctx

    async def on_start(self):
        def ready(ctx):
            data = ctx.get("clock", {})
            ctx.logger.info(f"inject ready: {data}")

        self.ctx.inject(needs={"clock"}, wants={"notify"}, ready=ready)

    async def on_stop(self, reason: str):
        _ = reason
```
