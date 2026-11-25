# Configuration Framework

AUTO-MAS uses its own configuration framework for declarations, validation, and YAML persistence. The backend uses transactions, business locks, and change signals to keep application state in sync with configuration. The frontend uses UI hints and change notifications to keep pages in sync with backend values.

The framework lives in `app/config/` and uses Pydantic models. Access application configuration with `from app.core import Config` and framework operations with `from app.config import config_manager`.

## Overview

### Source Layout

::: tip
This section is for contributors working on or studying the framework internals. First-time contributors can skip it.
:::

- `app/config/`

  - `core/` defines the framework manager, nodes, groups, entries, collections, and staging operations.
  - `fields/`, `types.py`, and `shortcuts.py` define references, encryption, UI hints, legacy field handling, and standard field types.
  - `signals.py` defines events for entries and collections.
  - `wire.py` defines Pydantic serialization and deserialization for API requests and responses.

- `app/core/config/`

  - `manager.py` provides the `Config` singleton, application access to configuration, and configuration lifecycle coordination.
  - `upgrade.py`, `game_upgrade.py`, and `plugin_upgrade.py` migrate configuration files between versions.
  - `backup.py` backs up backend configuration.

- `app/models/`

  - `config/` contains application configuration classes, field declarations, defaults, and validation rules.
  - `schema.py` contains shared API contracts. Endpoint-specific request and response models live at the top level of their `app/api/` module.

### Minimal Complete Example

This example creates, activates, updates, and saves a configuration entry backed by a file.

```python
import asyncio
from pathlib import Path
from pydantic import Field

from app.config import ConfigEntry, ConfigGroup, config_manager


class ExampleConfig(ConfigEntry):
    """Example configuration."""

    class InfoFields(ConfigGroup):
        Name: str = Field(default="Default name", description="Display name")
        Enabled: bool = Field(default=True, description="Whether it is enabled")

    Info: InfoFields = Field(default_factory=InfoFields)


async def main() -> None:

    # Register the persistence path with file=.
    config = ExampleConfig.build(file=Path("config/example.yaml"))
    # Activate the entry.
    await config.activate()

    # Stage the assignment.
    config.Info.Name = "New name"
    # Commit the operations staged on this node.
    await config.commit()

    assert config.Info.Name == "New name"

    # Bypass the save delay and write the configuration immediately.
    await config_manager.flush()
    # Unregister the persistence root.
    config_manager.unregister_root(config)


if __name__ == "__main__":
    asyncio.run(main())
```

## Configuration Structure

::: tip
This section explains how the project organizes configuration and how that structure is implemented.
:::

#### Logical Design

<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 880 390" role="img" aria-labelledby="config-tree-title config-tree-desc" style="display:block;width:100%;max-width:880px;margin:1rem auto">
  <title id="config-tree-title">Configuration Model</title>
  <desc id="config-tree-desc">Two examples of alternating collection and entry nodes. Collections have variable membership and entries have fixed fields.</desc>
  <defs>
    <marker id="config-tree-field-arrow" markerWidth="8" markerHeight="8" refX="7" refY="4" orient="auto">
      <path d="M0 0L8 4L0 8Z" fill="#28548c"/>
    </marker>
    <marker id="config-tree-member-arrow" markerWidth="8" markerHeight="8" refX="7" refY="4" orient="auto">
      <path d="M0 0L8 4L0 8Z" fill="#7c3a12"/>
    </marker>
  </defs>
  <rect width="880" height="390" rx="12" fill="#f8fafc"/>
  <path d="M450 22V306M24 318H856" fill="none" stroke="#cbd5e1"/>
  <g fill="none" stroke="#28548c" stroke-width="2" marker-end="url(#config-tree-field-arrow)">
    <path d="M100 135V174"/>
    <path d="M290 135V174"/>
    <path d="M665 62V80"/>
    <path d="M565 170V188"/>
    <path d="M765 170V188"/>
  </g>
  <g fill="none" stroke="#7c3a12" stroke-width="2" stroke-dasharray="5 4" marker-end="url(#config-tree-member-arrow)">
    <path d="M195 62C195 80 100 80 100 95"/>
    <path d="M195 62C195 80 290 80 290 95"/>
    <path d="M290 214C290 244 92 238 92 253"/>
    <path d="M290 214C290 235 217 235 217 253"/>
    <path d="M290 214C290 235 342 235 342 253"/>
    <path d="M665 116C665 126 565 126 565 134"/>
    <path d="M665 116C665 126 765 126 765 134"/>
  </g>
  <g fill="#fff9c4" stroke="#f9a825" stroke-width="2">
    <rect x="95" y="22" width="200" height="40" rx="20"/>
    <rect x="14" y="174" width="172" height="40" rx="5"/>
    <rect x="204" y="174" width="172" height="40" rx="5"/>
    <rect x="575" y="80" width="180" height="36" rx="5"/>
    <rect x="475" y="188" width="180" height="36" rx="5"/>
    <rect x="675" y="188" width="180" height="36" rx="5"/>
    <rect x="74" y="332" width="26" height="16" rx="4"/>
  </g>
  <g fill="#e1f5fe" stroke="#0288d1" stroke-width="2">
    <rect x="40" y="95" width="120" height="40" rx="5"/>
    <rect x="230" y="95" width="120" height="40" rx="5"/>
    <rect x="36" y="253" width="112" height="36" rx="5"/>
    <rect x="161" y="253" width="112" height="36" rx="5"/>
    <rect x="286" y="253" width="112" height="36" rx="5"/>
    <rect x="565" y="22" width="200" height="40" rx="20"/>
    <rect x="505" y="134" width="120" height="36" rx="5"/>
    <rect x="705" y="134" width="120" height="36" rx="5"/>
    <rect x="478" y="332" width="26" height="16" rx="4"/>
  </g>
  <g fill="#172033" font-size="17" text-anchor="middle" dominant-baseline="central">
    <text x="195" y="42">Collection · root</text>
    <text x="100" y="115">Entry A1</text>
    <text x="290" y="115">Entry A2</text>
    <text x="100" y="194">Collection · field B</text>
    <text x="290" y="194">Collection · field B</text>
    <text x="100" y="228" fill="#475569">Empty collection</text>
    <text x="92" y="271">Entry C1</text>
    <text x="217" y="271">Entry C2</text>
    <text x="342" y="271">Entry C3</text>
    <text x="429" y="271" font-size="22">…</text>
    <text x="217" y="303" font-size="22">⋮</text>
    <text x="665" y="42">Entry · root</text>
    <text x="665" y="98">Collection · field D</text>
    <text x="565" y="152">Entry E1</text>
    <text x="765" y="152">Entry E2</text>
    <text x="565" y="206">Collection · field F</text>
    <text x="765" y="206">Collection · field F</text>
    <text x="565" y="244" font-size="22">⋮</text>
    <text x="765" y="244" font-size="22">⋮</text>
  </g>
  <path d="M74 367H100" fill="none" stroke="#28548c" stroke-width="2" marker-end="url(#config-tree-field-arrow)"/>
  <path d="M478 367H504" fill="none" stroke="#7c3a12" stroke-width="2" stroke-dasharray="5 4" marker-end="url(#config-tree-member-arrow)"/>
  <g fill="#172033" font-size="16" dominant-baseline="central">
    <text x="114" y="340">Collection node</text>
    <text x="518" y="340">Entry node</text>
    <text x="114" y="367">Fixed fields</text>
    <text x="518" y="367">Add or remove members</text>
  </g>
</svg>

The framework uses a **recursive tree model** in which `Entry` and `Collection` nodes alternate.

- **Entry**

  An entry holds data. Read its fields through the entry. Its field and child-collection declarations are fixed at runtime.

- **Collection**

  A collection organizes child entries. Read its members through the collection. Members can be added or removed at runtime.

#### Implementation

The business-facing configuration model has three levels.

- **L3 Configuration group.** Groups organize related fields in an `Entry`. For example, a managed script's `Info` group holds its name and enabled state. Its `Run` group holds execution settings such as retry limits.
- **L2 Configuration entry.** An `Entry` represents one configuration object. It can have multiple groups and collections. For example, a managed script has groups such as `Info` and `Run`, plus a `UserData` collection for its accounts.
- **L1 Configuration collection.** A `Collection` holds entries of registered types and indexes them by UUID. A list of managed scripts is one example.

```text
ScriptConfig (L1)
└─ <managed script> (L2)
   ├─ Info (L3): Name, Status
   ├─ Run (L3): Mode, RetryCount, Tasks
   └─ UserData (L1)
      └─ <account> (L2)
         ├─ Info (L3): Name, ID
         └─ Data (L3): LastRunTime
```

### Usage

#### Configuration Node | Node {#config-node}

`ConfigNode` is the common parent of configuration entries (`ConfigEntry`) and collections (`ConfigCollection`). It defines the operations they share.

Configuration nodes are Pydantic models. You can construct configuration instances from them and use them as type declarations in FastAPI.

- **Properties**

  - `uid` is the node's UUID.
  - `parent` is the direct parent. It is `None` for a root.
  - `root` is the root of the node's configuration tree. A root returns itself.
  - `is_root` indicates whether this is a root node.
  - `persist_path` is the bound YAML path. It is `None` for an unregistered root and all its descendants.
  - `activation_state` is `INACTIVE` for a cold node, `INITIALIZING` during initialization, and `ACTIVE` for a hot node.
  - `deleted` indicates whether the node has been soft-deleted.
  - `is_locked` indicates whether the node has a shared or exclusive lock. The context holding the matching exclusive lock ticket can still commit changes.

- **Construction**

  Construction returns a cold instance. It has standard Pydantic behavior and can serve as a FastAPI return value. Framework-specific behavior becomes available after activation.

  - From a dictionary

    Validate the supplied field values immediately. They can be read from the cold instance.

    ```python
    <instance> = <configuration node class>.model_validate(<configuration dictionary>)
    ```

  - Full construction

    Pass the full set of node parameters. Payload values are validated during activation and become readable afterward.

    ```python
    <instance> = <configuration node class>.build(
        parent=<parent node>,
        uid=<node UUID>,
        payload=<staged configuration dictionary>,
        file=<bound YAML path>,
        **<other options>,
    )
    ```

    - `parent=` defaults to `None` for an independent root. The framework uses this parameter to attach nodes.
    - `uid=` defaults to `None`, so the framework generates a UUID.
    - `payload=` supplies the configuration dictionary for activation. Its values are unavailable on the cold instance. Use either `payload=` or `file=`.
    - `file=` binds a YAML persistence path and is available only for a root. Activation loads the file, and later configuration changes are saved automatically.

- **Activation**

  Activation turns a cold instance into a hot instance, loads the configuration, and enables framework behavior.

  Normally, activate the root and let the framework activate its child nodes recursively. Activation loads values in this order of precedence.

  1. `payload=`
  2. `file=`
  3. Existing values on the cold instance

  ```python
  await <node>.activate(
      regenerate_uuids=<regenerate child node UUIDs>,
      fast_init=<attempt fast initialization>,
  )
  ```

  - `regenerate_uuids=` is typically used when copying a configuration. Reference fields retain their input values even when member UUIDs change. To point a reference at a member in the copy, remap its UUID explicitly.
  - `fast_init=` groups the current entry's field initialization in a local transaction to reduce per-field transaction overhead. If a failure rolls back that attempt, initialization falls back to the regular per-field path.

  See [Initialization Order](#initialization-order) for initialization failures and reference dependencies.

- **Commit**

  Assignments to ordinary fields and changes to collections on hot nodes are staged. They take effect when `commit()` applies them within the applicable transaction boundary.

  - Default transaction boundary

    Each operation and the changes it directly triggers form one transaction. Within one `commit()`, **successful operations can take effect even if another operation fails**.

    ```python
    await <node>.commit()
    ```

  - Explicit transaction boundary

    Use `transaction()` to group changes and their effects. **An exception that escapes the block rolls back all changes in its boundary.** Commit every staged operation in that transaction before leaving it. Pending operations at exit cause an error and rollback.

    ```python
    async with config_manager.transaction():
        <node A>.<group>.<field> = <new value>
        await <node A>.commit()
        <node B>.<group>.<field> = <new value>
        await <node B>.commit()
    ```

  See [Transaction Mechanics](#transaction-mechanics) for workspaces, nesting, and concurrency.

- **Business locks**

  Hot nodes can be locked. Reading ordinary fields and iterating over collections require no ticket. Locks govern writes to ordinary fields and collections.

  - **S lock (shared)** allows other S locks and rejects ordinary writes within its scope.
  - **X lock (exclusive)** excludes other S or X locks. Only the context holding the matching ticket can stage and commit changes.

  Lock acquisition does not wait. On a conflict, `lock_s()` and `lock_x()` raise `ValueError`, while `try_lock_x()` returns `None`. Release a lock on the node that issued its ticket.

  ```python
  <shared lock ticket> = await <node>.lock_s(cascade=<lock descendants>)
  <exclusive lock ticket> = await <node>.lock_x(cascade=<lock descendants>)
  <exclusive lock ticket or None> = await <node>.try_lock_x(cascade=<lock descendants>)
  await <node>.unlock(<original lock ticket>)
  ```

- **Export**

  - Pydantic export

    ```python
    <configuration dictionary> = <node>.model_dump(
        mode="json", context=ExportContext(audience=<export audience>)
    )
    ```

    - `mode=` selects the representation.
      - `"json"` converts fields to JSON-compatible values.
      - `"python"` retains native Python types.
    - `audience=` selects the purpose of JSON export.
      - `"api"` is the default. It includes virtual and trigger fields, applies visibility and masking rules, and uses plaintext for encrypted fields when exported.
      - `"persist"` is for persistence. It omits virtual and trigger fields and retains encrypted values.
      - `"export"` is for backups and migration. It omits virtual and trigger fields and exports encrypted fields as plaintext.

  - Persistence export

    Equivalent to `.model_dump(mode="json", context=ExportContext(audience="persist"))`.

    ```python
    <persistence dictionary> = await <node>.to_dict()
    ```

#### Configuration Declarations

The tree has two node types. Configuration entries inherit from `ConfigEntry`, and collections use `ConfigCollection[T]`. Both share the `ConfigNode` base class. Configuration groups inherit from `ConfigGroup` and organize fields inside an entry.

- **Groups.** Declare fields as in Pydantic. Use `Field(...)` for defaults, constraints, and descriptions. Define the group independently or inside the entry class.
- **Entries.** Use type annotations for group fields and `Field(default_factory=...)` to construct a fresh group for each instance.
- **Nested collections.** Declare collection fields with `collection(...)`. The `ConfigCollection[...]` annotation gives static type checkers the member type. Arguments passed to `collection(...)` determine the member types initially accepted at runtime.

```python
from pydantic import Field

from app.config import ConfigCollection, ConfigEntry, ConfigGroup, collection


class <entry class>(ConfigEntry):
    class <group class>(ConfigGroup):
        <field name>: <field type> = Field(default=<default value>, description=<field description>)

    <group field name>: <group class> = Field(default_factory=<group class>)

    <collection field name>: ConfigCollection[<member type A> | <member type B> | ...] = collection(
        <member type A>,
        <member type B>,
        ...
    )
```

For collections that accept more than one member type, use a union such as `ConfigCollection[<member type A> | <member type B>]` and pass each class separately to `collection(<member type A>, <member type B>)`.

Entry subclasses can override group or collection fields. Narrowing mutable fields can trigger a static type error. When a local suppression is needed, add it to the declaration and explain why.

#### Configuration Entry | Entry

- **Construction**

  In addition to the methods under [Configuration Node](#config-node), construct an entry by passing group field names directly.

  ```python
  <instance> = <entry class>(<group field name>=<group instance>)
  ```

- **Read and write fields**

  Read or change ordinary fields through their groups. An assignment to a cold instance validates and applies immediately. An assignment to a hot instance is staged, then validated and applied on commit.

  To edit a mutable field such as a list or dictionary on a hot instance, copy the value, edit the copy, and assign the entire new value back.

  ```python
  <configuration value> = <entry>.<group name>.<field name>
  <entry>.<group name>.<field name> = <new value>
  await <entry>.commit()
  ```

- **Batch updates**

  `update()` applies and commits the explicitly supplied ordinary fields from a cold instance of the same class, then handles trigger commands. It skips virtual fields and collection fields. Use it for configuration updates received through the API.

  Outside an explicit transaction, a batch update commits automatically under the default per-operation boundary. Individual changes can succeed or fail independently.

  ```python
  await <entry>.update(<cold entry of the same class>, check_visibility=<apply UI hints>)
  ```

  - `check_visibility=` defaults to `True`. It skips ordinary fields that the UI hides or disables.

- **Invoke a trigger**

  A trigger field controls a task. Outside a transaction, send a trigger command to the configuration instance to control the task directly.

  ```python
  <task UUID or None> = <entry>.write_trigger(
      group=<group name>,
      field=<field name>,
      value=<command value>,
  )
  ```

  - `value=` selects the command.
    - `None` performs no operation and returns `None`.
    - `True` starts the task and returns its UUID. If it is already running, it returns the existing UUID.
    - `False` requests cancellation and returns the current task UUID, or `None` if idle.

#### Configuration Collection | Collection

- **Construction**

  In addition to the [Configuration Node](#config-node) parameters, full collection construction accepts the following.

  ```python
  <instance> = ConfigCollection[<member type>].build(
      entry_types=<member type or list of types>,
      name=<reference name>,
      order=<ordered index>,
      data=<member mapping>,
      **<other options>,
  )
  ```

  - `entry_types=` registers one entry class or a list of classes. If omitted, the collection subclass's default types are used.
  - `name=` registers a unique lookup name for a root collection. `ref(...)` uses this name independently of the YAML filename.
  - `order=` is the cold collection's ordered list of member UUIDs and type names.
  - `data=` maps member UUIDs to cold member configuration.

- **Add a member**

  `add()` stages a new member and immediately returns its allocated UUID. The UUID becomes readable from the collection after a successful commit.

  ```python
  <member UUID> = <collection>.add(
      entry_type=<member type>,
      payload=<initial configuration dictionary>,
  )
  await <collection>.commit()
  ```

  - `entry_type=` accepts a registered member class or its name as a string.
  - `payload=` supplies the new member's initial dictionary. The framework activates the member with it at commit.
  - `uid=` allows specifying the member UUID. The framework generates one by default.
  - `regenerate_uuids=` defaults to `False`. Set it to regenerate UUIDs in nested collections during member activation, typically when copying a configuration. Remap references separately when they should point to the copied nodes.

- **Read members**

  Treat a collection as an ordered dictionary keyed by UUID. Use `get()`, `keys()`, `values()`, and `items()` to access its members.

  ```python
  <entry> = <collection>[<member UUID>]
  <collection> = <entry>.<collection field name>
  ```

- **Remove a member**

  ```python
  <collection>.remove(uid=<member UUID>)
  await <collection>.commit()
  ```

- **Reorder members**

  ```python
  <collection>.set_order(order=<member order>)
  await <collection>.commit()
  ```

  - `order=` is a sequence of UUID objects or `CollectionOrderItem` values in the target order. Include every member exactly once.

- **Change member types at runtime**

  A collection supports adding, removing, or reloading registered member types. `T` in `ConfigCollection[T]` still determines static type hints.

  If `T` in `ConfigCollection[T].build(...)` is one `ConfigEntry` class, a type registered dynamically must be `T` or a subclass.

  A successful **type reload** or **type replacement** soft-deletes old instances and creates new ones. Reading fields or modifying a soft-deleted instance through the framework raises `DeletedNodeError`. After a reload, resolve instances by UUID and bind their instance subscriptions again. Old instance subscriptions are not migrated to the new instances.

  - **Register a member type**

    The collection registers types by class name. Passing the same registered class skips registration. Passing a different class with the same name raises an error.

    ```python
    <collection>.add_type(entry_cls=<new member type>)
    await <collection>.commit()
    ```

  - **Unregister a member type**

    Removing a type removes all its members in the same transaction. The entire operation fails if any member is locked or a deletion guard rejects removal.

    ```python
    <collection>.remove_type(entry_type=<registered member class or name>)
    await <collection>.commit()
    ```

  - **Reload a member type**

    Rebuild all members of a type from their existing configuration and retain their UUIDs and order.

    ```python
    <collection>.reload_type(entry_type=<replacement class with the same name or registered type name>)
    await <collection>.commit()
    ```

    - `entry_type=` accepts a new class with the registered name to update the registration and rebuild members, or the registered name as a string to rebuild members with the current class. Passing the registered class itself skips the operation.

  - **Replace one member's type**

    Rebuild one member from a target class and supplied dictionary. The member keeps its UUID and position, and nested members retain the UUIDs in the supplied payload.

    ```python
    <collection>.replace_type(
        uid=<member UUID>,
        entry_type=<target member type>,
        payload=<converted configuration dictionary>,
    )
    await <collection>.commit()
    ```

## Extended Configuration Fields

In addition to standard Pydantic fields, the framework provides extensions for common configuration needs.

_Read and write refer to direct access to field values. Export refers to producing API responses or persisted data._

### String Extensions

```python
<field name>: <string extension type> = <default string>
```

| Type                    | Input handling                                                                                                                                                                                                                                                                                                                              |
| ----------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `JsonDictString`        | Keeps a valid JSON object string. Other input falls back to `"{ }"`.                                                                                                                                                                                                                                                                        |
| `JsonListString`        | Keeps a valid JSON array string. Other input falls back to `"[ ]"`.                                                                                                                                                                                                                                                                         |
| `UrlString`             | Keeps a URL with a scheme and network location. Other input is cleared.                                                                                                                                                                                                                                                                     |
| `KeyboardKeyString`     | Lowercases key names. Invalid input falls back to the field default. Declare a valid key as the default.                                                                                                                                                                                                                                    |
| `WindowsNameString`     | Removes invalid Windows name characters and trims surrounding whitespace and periods. Limits the result to 255 characters. Empty or reserved names fall back to `"默认用户名"`.                                                                                                                                                             |
| `CliArgumentString`     | Validates command-line syntax. Keeps valid input as written and clears invalid input.                                                                                                                                                                                                                                                       |
| `CliArgumentListString` | Stores several sets of arguments in one field as `<relative program path 1>%<arguments 1>\|<relative program path 2>%<arguments 2>\|...`. A pipe separates sets. Each set may omit the `<relative program path>%` prefix. Anything after its first `%` is treated as arguments. Validation skips empty segments and checks argument syntax. |

### Path Fields

Path fields normalize input to an absolute `Path`. Empty or invalid paths become `None`. Normalization expands environment variables and the home directory and resolves symbolic links and Windows shortcuts.

- **Built-in path types**

  ```python
  <field name>: <path type> = None
  ```

  | Type             | Accepted path                                                                                                                      |
  | ---------------- | ---------------------------------------------------------------------------------------------------------------------------------- |
  | `FilePath`       | Existing file. Excludes drive roots, protected system paths, and the AUTO-MAS working directory and its ancestors and descendants. |
  | `FolderPath`     | Existing directory with the same exclusions as `FilePath`.                                                                         |
  | `ScriptRootPath` | Existing script directory. Excludes drive roots and protected system paths.                                                        |
  | `ExecutablePath` | Existing `.exe`, `.bat`, `.cmd`, or `.com` file with the same exclusions as `FilePath`.                                            |
  | `LoosePath`      | File or directory, including paths that do not yet exist.                                                                          |

- **Custom path policy**

  ```python
  <field name>: Annotated[
      Path | None,
      path_policy(
          kind=<path kind>,
          extensions=<sequence of extensions>,
          must_exist=<require an existing path>,
          blocked_dirs=<sequence of blocked directories>,
      ),
  ] = None
  ```

  - `kind=` accepts `"file"`, `"folder"`, or `"any"` for files, directories, or both.
  - `extensions=` is an optional list or tuple of file suffixes. It is unavailable for directory-only fields. A leading dot is optional and matching ignores case. With `kind="any"`, the restriction applies to paths identified as files.
  - `must_exist=` defaults to `False`. When existence is required and directories are accepted, a value that is not a directory can fall back to its parent if the parent exists.
  - `blocked_dirs=` is a list or tuple of directory paths or `PathBlockPreset` members. Omitting it adds no extra exclusions.

    - A directory path excludes itself and its descendants.
    - `PathBlockPreset.ROOTS` excludes filesystem roots.
    - `PathBlockPreset.SYSTEM` excludes Windows system directories and their ancestors and descendants.
    - `PathBlockPreset.PROGRAM_FILES_ROOTS` excludes `Program Files` and `Program Files (x86)`.
    - `PathBlockPreset.APP` excludes the AUTO-MAS working directory and its ancestors and descendants.

    `SYSTEM_BLOCKED_DIRS` supplies the `ROOTS`, `SYSTEM`, and `PROGRAM_FILES_ROOTS` presets. `STANDARD_BLOCKED_DIRS` also includes `APP`.

### Date and Time Fields

The framework handles time zones for `datetime` fields, including different rollover times by server. For example, if a server changes dates at 4:00 a.m. Beijing time, use `UTC+4` so that reading the date gives the server's day.

- **Write.** Convert timezone-aware input to UTC before storage. Interpret naive input as UTC.
- **Read.** Convert to the field's configured time zone.
- **Export.** Use the stored UTC value.

```python
<field name>: Annotated[datetime, tz(<time zone or resolver>)] = <default datetime>
```

- `tz(<time zone or resolver>)` defaults to `UTC+8` when omitted. Pass a `datetime.timezone` for a fixed offset or a synchronous callback for dynamic resolution. The callback receives the owning `ConfigGroup` instance and returns a `datetime.timezone`. Keep it read-only and avoid reading the same field recursively.

### Encrypted Fields

Place `encrypted()` **last** in a string field's `Annotated` metadata to enable encrypted storage.

- **Write.** After validation, encrypt nonempty plaintext. Store empty strings and existing ciphertext directly.
- **Read.** Decrypt automatically. A decryption failure returns the original ciphertext.
- **Export.** Retain ciphertext for persistence. API and backup exports attempt decryption.

The encryption backend can vary by platform.

- Windows uses device-based Windows DPAPI. Different users on the same device can decrypt the value.
- Other operating systems do not yet have an implementation.

```python
<field name>: Annotated[str, encrypted()] = <default string>
```

### Reference Fields

A reference field links to one member of a registered collection, much like a database foreign key.

```python
<field name>: Annotated[
    UUID | str,
    ref(
        target=<target collection reference name>,
        default=<fallback for an invalid reference>,
        allow_values=<allowed special values>,
        entry_types=<allowed target types>,
        display_field=<target display field path>,
        on_delete=<invalid reference policy>,
        on_delete_callback=<custom handler>,
    ),
] = <field default>
```

- `target=` is the collection's registered [reference name](#configuration-collection-collection), set with `name=`. Register the target before activating the reference.
- `default=` sets the string fallback and field default. The default is `"-"`.
- `allow_values=` is a tuple of special strings that may be stored directly. It is empty by default. Add a value here to make it a valid unselected option.
- `entry_types=` is a tuple of accepted entry classes. Subclasses also match. An omitted or empty tuple accepts any member type.
- `display_field=` is a field path such as `"Info.Name"` for the UI label. The option value remains the member UUID.
- `on_delete=` determines what happens when a target is removed or a reload or replacement invalidates its type. The target change and associated updates use one transaction.

  - `RefDeleteAction.SET_DEFAULT` is the default. It resets the reference to `default=`.
  - `RefDeleteAction.RESTRICT` rejects the operation that would invalidate the reference.
  - `RefDeleteAction.CASCADE` deletes the referencing entry. That entry must be a collection member.
  - `RefDeleteAction.CUSTOM` calls `on_delete_callback=`.

- `on_delete_callback=` is required only for `RefDeleteAction.CUSTOM`. Use a synchronous or asynchronous handler that receives the referencing entry and collection event, or pass the name of a method on that entry.

### Virtual Fields

A virtual field exposes a synchronous method on its owning entry as a read-only field, like a Python property.

- **Write.** Virtual fields are read-only and skipped by batch updates.
- **Read.** Call the bound synchronous method and return its result.
- **Export.** API output includes the result. Persistence and backup exports omit it.

```python
class <entry class>(ConfigEntry):
    class <group class>(ConfigGroup):
        <field name>: Virtual[<return type>] = None

    <group name>: <group class>

    @virtual_field("<group name>.<field name>")
    def <getter>(self) -> <return type>:
        return <computed result>
```

### Trigger Fields

A trigger field exposes task control as a configuration field. Write it to start or stop a task and read it to check the task's status.

Assignments inside an explicit transaction submit the command only after the root transaction succeeds. A rollback discards the command. If the command itself fails to be accepted, previously committed ordinary fields remain in effect.

- **Write.** `True` starts a task or uses the task already running. `False` requests cancellation. `None` skips the operation.
- **Read.** Return the running task UUID, or `None` when idle. After a cancellation request, the UUID remains until the task finishes.
- **Export.** API output matches the read value. Persistence and backup exports omit triggers.

```python
class <entry class>(ConfigEntry):
    class <group class>(ConfigGroup):
        <field name>: Trigger = trigger(
            timeout=<time limit>,
            description=<field description>,
        )

    <group name>: <group class>

    @trigger_field("<group name>.<field name>", finish_on_cancel=<run to completion on cancellation>)
    async def <handler>(self):
        <operation>
```

- `timeout=` is a finite, positive time limit in seconds. It defaults to 900 seconds.
- `finish_on_cancel=` defaults to `False`. Set it to `True` to let a handler finish after a cancellation request when cleanup must run to completion.

### UI Metadata

The framework derives UI hints from field declarations. Use `ui()` to override its widget, format, or visibility. Use `select()` to describe option sources and ordering.

- **UI hints**

  ```python
  <field name>: Annotated[
      <field type>,
      ui(
          widget=<widget name>,
          format=<display format>,
          deps=<dependent fields>,
          secret=<mark as sensitive>,
          visibility=<visibility policy>,
      ),
  ] = <default value>
  ```

  - `widget=` selects a control. The framework infers it when omitted.
  - `format=` selects a display format, for example `"hm"` for a time control.
  - `deps=` lists field names the UI needs to track together.
  - `secret=` marks sensitive content when `True`. Encrypted fields use it by default.
  - `visibility=` accepts a `UiVisibility` value or a read-only synchronous callback that receives the owning entry and returns one.
    - `UiVisibility.SHOW` displays the field and allows updates.
    - `HIDE` hides the field and skips it during default updates.
    - `DISABLE` displays the field and skips it during default updates.
    - `MASKED` displays a placeholder and allows replacement. Assigning the placeholder preserves the original value.

  A decorator can bind a dynamic visibility method.

  ```python
  @ui_visibility("<group name>.<field name>")
  def <visibility handler>(self) -> UiVisibility:
      return <visibility state>
  ```

  Apply `ui(visibility=...)` to a group annotation on the entry to set visibility for the entire group. Groups do not accept `MASKED`.

- **Select options**

  ```python
  <field name>: Annotated[
      <field type>,
      select(endpoint=<options endpoint>, ordered=<options are ordered>),
  ] = <default value>
  ```

  - `endpoint=` provides a dynamic options route. The framework infers static option types without a route.
  - `ordered=` indicates whether the selection order matters for a multiselect field. It defaults to `False`.

- **Table layout hints**

  Declare several groups derived from the same `ConfigTableColumn` subclass to arrange them as columns of one table. Group fields become rows. Read field hints through `entry.ui.hints` and table layouts through `entry.ui.tables`.

### Legacy Fields

When moving or renaming a field, use `legacy()` on the destination field to identify its old location. Cold construction or activation reads the old field only when the destination is omitted. An explicit `None` counts as a supplied value.

```python
<new field name>: Annotated[
    <field type>,
    legacy(group=<old group name>, name=<old field name>, convert=<value converter>),
] = <default value>
```

- `group=` is the old group name.
- `name=` is the old field name.
- `convert=` is an optional function that takes the old value and returns the value for validation by the new field.

## Configuration Signals and Events

Configuration change signals let application logic respond to data changes. Like a PostgreSQL `AFTER` trigger, a callback runs after its change and inside the transaction that triggered it.

### Event Types

- **Field change | `FieldChangeEvent`**

  - `kind` identifies the change.
    - `init_set` is emitted during initialization, even when the field value stays the same.
    - `set` is emitted at runtime when the field value changes.
  - `node` is the entry that owns the field and is also the callback's `sender`. `config` is its compatibility alias, and `config_uid` returns its UUID.
  - `group` is the field's group name.
  - `field` is the name of the changed field.
  - `value` is the validated, normalized new value. Encrypted fields pass plaintext.
  - `old_value` is the previous value, usually the default during initialization. Encrypted fields pass plaintext.

- **Collection change | `CollectionChangeEvent`**

  - `kind` identifies the change.
    - `init_add` adds a member during initialization.
    - `add` adds a member at runtime.
    - `remove` removes a member.
    - `set_order` changes member order.
    - `add_type` registers a member type.
    - `remove_type` unregisters a member type.
    - `reload_type` reloads a member type.
    - `replace_type` replaces the type of one member.
  - `collection` is the changed collection instance and is also the callback's `sender`.
  - `uid` is the member UUID for `init_add`, `add`, `remove`, or `replace_type`. Replacement retains that UUID.
  - `entry` is the member instance identified by `uid`. `init_add`, `add`, and `replace_type` supply the new instance. `remove` supplies the soft-deleted instance.
  - `old_order` is the order before `set_order`.
  - `order` is the order after `set_order`.
  - `entry_type` is the class involved. Type events supply the newly registered class, removed class, reloaded class, or new instance's class, respectively.
  - `reloaded_uids` is a tuple of rebuilt member UUIDs for `reload_type`, in collection order. It is empty if there were no members to rebuild.

### Subscribe and Unsubscribe

- **Class subscriptions** receive events from all instances of the class. Subclasses have their own subscriptions.
- **Instance subscriptions** receive events from a particular instance.

A callback is a synchronous or asynchronous function that supports attached attributes and accepts `sender` and `event`. Public subscriptions hold weak references and expire when the callback is collected.

```python
<class or instance>.connect(
    receiver=<callback>,
    phase=<event phase>,
    kind=<event kind>,
    group=<group name>,
    field=<field name>,
)
<class or instance>.disconnect(
    receiver=<original callback>,
    phase=<event phase>,
    kind=<event kind>,
    group=<group name>,
    field=<field name>,
)

@<class or instance>.connect(
    phase=<event phase>,
    kind=<event kind>,
    group=<group name>,
    field=<field name>,
)
def <callback>(sender, event):
    <handler logic>

@<class or instance>.disconnect(
    phase=<event phase>,
    kind=<event kind>,
    group=<group name>,
    field=<field name>,
)
def <callback>(sender, event):
    <handler logic>
```

- `receiver=` is the callback function.
- `phase=` selects event phases.
  - `"runtime"` receives runtime events. This is the default.
  - `"init"` receives initialization events.
  - `"both"` receives both.
- `kind=` filters by event kind. `None`, the default, accepts all kinds in the selected phase. For initialization subscriptions, `"set"` matches `init_set` and `"add"` matches `init_add`.
- `group=` filters field events by group. `None`, the default, accepts any group. Omit it for collection events.
- `field=` filters field events by name. `None`, the default, accepts any field. Omit it for collection events.

Unsubscribe on the original class or instance with the original function object and the same filters.

Redefining a function with the same name creates a new subscription when the new function is connected. While the old function is still referenced, both callbacks can receive events. Disconnect the original function before rebinding its name. Applying `@disconnect(...)` to the redefined function attempts to disconnect the new object.

### Signal Transactions

- **Callbacks run before the root transaction commits.** The framework applies the change, emits the event, and waits for callbacks. Callbacks see the changed configuration.
- **Commit callback changes before returning.** An `asyncio.create_task()` child has a separate transaction identity. Waiting for it to commit from inside the callback can leave both tasks blocked. See [Queuing, Reentrancy, and Background Tasks](#queuing-reentrancy-and-background-tasks).
- **An exception that escapes a transaction boundary rolls back its changes together.** This covers the triggering change and related changes committed by callbacks.
- **Subscription changes apply to later events in the same transaction.** Other transactions see them after the root commits. A rollback discards subscriptions and unsubscriptions in the affected boundary. See [Signal Workspaces](#signal-workspaces).

## Automatic Persistence and Export

A root bound with `file=` saves changes to its YAML file automatically. The framework waits briefly after a change takes effect so that several changes can share one write.

I/O errors are retried up to a limit. Other errors are recorded without a retry. Persistence errors are available in `config_manager.persistence_errors` and are not raised automatically.

- **Write to disk immediately**

  Call `flush()` outside a transaction when committed values need to be written to YAML. It bypasses the save delay and attempts to save all eligible roots.

  ```python
  await config_manager.flush()
  ```

- **Unregister the persistence root**

  Call `flush()` first if the latest changes should be saved.

  ```python
  config_manager.unregister_root(node=<registered root node>)
  ```

- **Export manually**

  Export registered configuration to a destination directory while preserving the relative directory layout. Export uses `audience="export"` and writes encrypted fields as plaintext.

  ```python
  await config_manager.export(target=<export directory>)
  ```

## Use the Configuration Framework with FastAPI

The configuration framework uses Pydantic models, which can serve as FastAPI contracts.

### Declare Request and Response Models

Derive the request or response `data` model from an existing entry class.

- **Regular request | `ConfigInput`**

  Accept field updates and ignore unknown fields or groups.

  ```python
  from pydantic import BaseModel

  from app.config.wire import ConfigInput


  class <update request model>(BaseModel):
      data: ConfigInput[<configuration class>]
  ```

- **Strict request | `StrictConfigInput`**

  Accept field updates and reject unknown fields or groups.

  ```python
  from pydantic import BaseModel

  from app.config.wire import StrictConfigInput


  class <update request model>(BaseModel):
      data: StrictConfigInput[<configuration class>]
  ```

- **Response | `ConfigPayload`**

  Return queried fields without child nodes.

  ```python
  from app.config.wire import ConfigPayload
  from app.models.schema import ConfigSnapshotOut

  <configuration response data model> = ConfigPayload[<configuration class>]


  class <query response model>(ConfigSnapshotOut):
      data: <configuration response data model>
  ```

- **Response with nested collections | `config_contract()`**

  Return queried fields and their child nodes.

  ```python
  from app.config.wire import config_contract
  from app.models.schema import ConfigSnapshotOut

  <configuration response data model> = config_contract(<configuration class>, include_collections=True)


  class <query response model>(ConfigSnapshotOut):
      data: <configuration response data model>
  ```

### Query Configuration

Have the application method read the current configuration and return an API-ready dictionary. Use the dictionary to build the route's response `data`. When a page needs to refresh after changes, include the UUIDs of the entries and collections it depends on in `uiSources`. See [Configuration UI Synchronization](#configuration-ui-synchronization).

```python
@router.post("/<query path>", response_model=<query response model>)
async def <query route>() -> <query response model>:
    data = await <configuration service>.<query method>()
    return <query response model>(
        data=<configuration response data model>(**data),
        uiSources=[<configuration node UUID>],
    )
```

For a collection, `order` lists member UUIDs and types in order. `data` maps each member UUID to its configuration.

### Update Configuration

Use `model_dump(exclude_unset=True)` so that the patch contains only explicitly provided fields.

```python
@router.post("/<update path>", response_model=OutBase)
async def <update route>(body: <update request model>) -> OutBase:
    changes = body.data.model_dump(exclude_unset=True)
    await <configuration service>.<update method>(changes)
    return OutBase()
```

## Configuration UI Synchronization

Configuration responses identify the nodes a page depends on. When those nodes change, the backend notifies the frontend over WebSocket so it can read them again.

If the connection drops, the frontend refreshes configuration used by the visible page every five seconds. It stops polling when the connection is restored.

The response's `uiSources` lists those configuration node UUIDs. For an individual configuration, use the entry's UUID. For a collection query, use the collection's UUID. If the result depends on multiple nodes, include all of their UUIDs.

## Advanced Mechanics

### Initialization Order

When an entry enters `INITIALIZING`, it initializes ordinary fields, recursively activates nested collections, and then establishes reference subscriptions. It becomes `ACTIVE` after initialization. Other execution contexts see the activation result after the root transaction commits.

#### Per-field and Fast Initialization

The default path validates and commits each field or member and emits its initialization signal within its own local boundary. If one field fails, successful operations can be retained and failures are collected in `ConfigAggregateError`. When root activation opens its own ordinary transaction, successful work is committed to live state, the node becomes `ACTIVE`, and the aggregate error is then raised.

Use `activate(fast_init=True)` to try grouping **the current entry's entire field loop** in one local transaction. This reduces the cost of creating and merging separate workspaces.

1. Assign and validate fields in their original order, emitting each initialization signal immediately.
2. Roll back the whole fast transaction if any field, direct initialization signal chain, or associated framework-managed update fails.
3. Once rollback finishes, restart from the first field using the regular per-field path. Successful work from the fast attempt is repeated.
4. Callbacks and third-party services may run once in the fast attempt and once during fallback. The framework cannot detect, deduplicate, or roll back external effects.

Each nested entry handles its own field boundary and fallback. Collection member creation, reload, and replacement retain member-level boundaries. Within an existing transaction, successful work merges only into the parent workspace. If the parent later fails, rollback restores the node's cold state and pending activation payload.

`fast_init` applies to that activation call. Ordinary runtime `commit()` keeps its usual boundary. Without the option, activation uses the per-field path.

#### Activation Markers and Hot-State Restrictions

A node's `NodeState` records its lifecycle. A separate marker identifies the recursive activation operation as a whole. A node inherits the marker from its direct parent at construction. `build()` and recursive `activate()` calls do not need to pass it explicitly. An independent entry point creates a marker if none is active.

While initialization is incomplete, hot-state operations such as locks, trigger commands, public instance subscriptions, deletion guards, and file or name registration are restricted within the activation tree. Stable nodes outside it retain their hot-state behavior. Cold field assignment and reference subscriptions created by activation continue to work. Even after a node reaches `ACTIVE`, other execution contexts rely on the root transaction to publish its stable state.

#### Reference Targets and Startup Dependencies

At startup, register reference targets and activate their members before activating configuration that depends on them. Use `name=` for root collections and `config_manager.register_collection()` for nested collections that must be reference targets. `app/core/config/manager.py` loads global configuration and the plugin registry, waits for plugin-provided configuration types, then activates game configuration and the business configuration that references games.

### Signal Workspaces

Configuration signals use Blinker. The framework calls receiver wrappers synchronously, then awaits the returned awaitables one at a time. Blinker determines receiver order. To guarantee a sequence of operations, call them in order within a single receiver.

When a transaction first changes a node class's subscriptions, the framework copies its effective signal into a workspace for the current boundary. Emission looks outward from that boundary for the nearest workspace and falls back to live signals. A child boundary merges its workspace into the parent when it commits. A root commit publishes the workspace to live signals. Rollback discards the workspace for the affected boundary.

Blinker has no public interface for listing all subscriptions. To create a workspace, the framework rebuilds the subscription index from receivers, senders, and their indexes while preserving weak references and cleanup behavior. Application code uses the public `connect()` and `disconnect()` methods.

### Transaction Mechanics

#### Default Boundary for One Operation and Its Signal Chain

Assigning a field on a hot node stages an operation in the current owner's queue. The effective value is unchanged. Once `commit()` obtains execution rights, it snapshots the queue for that scope and processes operations in order.

1. Acquire the global execution gate before the node's commit lock.
2. If there is no effective transaction, create a context for the operation. Create the node's workspace on its first change.
3. Apply the field or collection operation in the workspace, run validation, and emit the change signal.
4. Keep ordinary callbacks, direct calls, and directly awaited operations in the same task and transaction. Related nodes receive their own workspaces.
5. Merge all related workspaces after callbacks finish with no uncommitted staged changes. Discard them if an exception escapes the boundary.
6. Close the context, clean up the scope, and mark any registered roots that need to be saved.

By default, **each operation in a `commit()` batch has its own transaction**. Each succeeds or rolls back independently. The framework processes later operations and reports failures together in `ConfigAggregateError`. The global gate and node lock cover the whole batch. Other owners commit after the batch ends. `fast_init` adds a local boundary for an entry's field loop during activation. Ordinary runtime commits retain their own rules.

For example, if one commit applies staged operations A and B, a successful A can remain applied even when B fails. Use an explicit transaction when the operations need to succeed or roll back together.

#### Explicit Transactions and Nesting

Nested `transaction()` calls in the same task reuse the existing transaction. Use `transaction(new_boundary=True)` for a separate rollback boundary. When `commit()` returns inside a transaction, staged operations have run in the workspace. Other owners see those changes after the root commits.

Initialization and runtime updates use the same transaction context. The context tracks the boundary, parent chain, actual owner, and participating nodes. Node instances hold data workspaces and node classes hold signal workspaces. A successful child boundary merges into its parent's workspace. The root publishes live state, updates generations, and requests persistence.

::: info Exceptions and Rollback Boundaries
An exception that escapes the root rolls back the entire transaction. If it escapes only a `new_boundary=True` child and the outer scope catches it, only that child rolls back. When an exception is caught inside the same boundary and the transaction commits normally, workspace changes still take effect.
:::

Transactions protect framework-managed configuration and subscription workspaces. Already sent messages, device operations, database writes, and external file changes have separate lifecycles and cannot be rolled back by this framework.

#### How Workspace Reads Are Resolved

A transaction context checks its lifecycle and actual owner. It checks the task object when one exists and the thread object otherwise. Inheriting a `ContextVar` alone does not grant access to a transaction.

Ordinary attribute lookup checks the effective initialization workspace first, then the effective regular workspace, then live state. Nodes without a workspace read live state directly. Collection identities and application references point to framework-managed live nodes. Workspaces are temporary and end on commit or rollback.

`node.effective` returns the effective node in the current context. Access such as `config.Info.Name` uses its effective field values. **Calling `model_dump()` on the original live object exports stored live values** for persistence or FastAPI. Use `node.effective.model_dump()` to export the current workspace. Its values remain pending until the outermost transaction commits.

Datetime fields export the stored UTC value directly, without the time zone conversion used by attribute access. Virtual fields, UI hints, and other computed API values follow their own declarations.

#### Queuing, Reentrancy, and Background Tasks

A single global transaction gate coordinates commits, explicit transactions, autosave, and `flush()`. Lock order is always the global gate followed by the node lock. Related changes use the same order on reentry, preventing a cycle where each executor holds a node lock while waiting for the gate.

The same actual owner can reenter. Other owners obtain execution rights through the wait queue. Asynchronous callers suspend, and synchronous calls from worker threads block. If `transaction_sync()` runs on the event loop thread while another owner holds the gate, it raises to keep the loop responsive.

An independent `asyncio.create_task()` has its own transaction identity. It reads live state and queues for permission to write. If the parent holds the gate and waits for the child's commit, the child waits for the parent and neither can complete. Direct calls or awaits in the same task include related changes in the parent transaction. Independent tasks started after the transaction ends commit in their own boundary.

Owners without workspace access read live state and cannot observe another transaction's staged values. Ordinary attribute reads reflect the effective state at each access. The manager's export entry point uses the global gate to obtain a stable export across roots.

#### Staging Ownership, Cancellation, and Cleanup

Staging queues are isolated by actual owner and by ordinary or initialization scope. They live on live nodes, stay outside workspaces and persistence, and never transfer to another task.

- Assign and commit in the same scope. Outside-transaction commits read outside-transaction staging. Commits inside an explicit transaction read staging from that transaction.
- If the current scope is empty and the same owner has staged operations elsewhere, `commit()` raises and leaves the other queues intact.
- A signal callback commits the changes it stages. Uncommitted staging for that chain at callback return or transaction exit causes an error and rollback.
- Rollback cleans up the current scope, leaving queues owned by other tasks intact. When a task ends, any staging it still owns is cleared.
- Cancellation while waiting removes the queue entry or returns an execution grant that already changed hands. Cancellation during execution rolls back the current operation and returns rolled-back and unexecuted operations to their queues. Application changes are not automatically retried. Closing the transaction still cleans up its remaining staging according to the scope rules.
