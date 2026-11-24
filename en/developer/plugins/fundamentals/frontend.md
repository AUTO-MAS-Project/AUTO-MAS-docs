# Frontend and Backend Integration

::: warning
This guide describes the host contract after the Game Management migration. Check the SDK that ships with your target AUTO-MAS version.
:::

The main program's frontend **does not load frontend code shipped with a plugin**: there are no iframes, no remote modules, and no dynamic page loader. A plugin can affect the UI through only four channels.

| Channel | What the plugin provides | How the frontend uses it |
| --- | --- | --- |
| Capability list | The managed types registered after enable | Decides which types appear in "new managed script"; the entries for MAA pages and for creating a MAA plan table are also opened according to it |
| Declarative settings | UI hints on the plugin's config class | A generic form renderer draws the "Plugin Settings" page |
| Strings | `i18n/*.json` inside the package | The plugin name, and the labels and tooltips of settings |
| Adapter pages | None (the pages are in the main repository) | Registered statically by type in the main repository and built with the desktop app |

## Capability List

- `GET /api/scripts/types` returns the managed type keys currently available. After a plugin is enabled, its `script_type_key` appears in it, and after it is disabled it disappears.
- After a plugin is enabled, disabled, or fails to enable, the backend pushes `plugin.capabilities.updated` over WebSocket (route id `Plugin`), and the frontend fetches it again on receiving it. It also fetches again after a WebSocket reconnect.
- The frontend wrapper is `frontend/src/composables/useScriptCapabilities.ts`. The new-managed-script dialog intersects the capability list with the types in the frontend's `SCRIPT_METADATA`: **a type appears only if the backend has it enabled and the frontend has registered it.**
- Page routes are currently intercepted only for MAA: when the backend does not have `MAA`, opening the MAA page redirects to the plugin marketplace (`frontend/src/router/index.ts`). The entry for creating a plan table likewise filters only the MAA plan table by capability. The edit page routes of other adapters do not look at the capability list.

## Configuration Trigger API

1. Save any action inputs through the ordinary configuration endpoint. Send a separate `POST /api/config/triggers/write` to invoke the action.

   ```json
   { "configUid": "<entry UUID>", "group": "action", "field": "send_test", "value": true }
   ```

   Send `false` to request cancellation or `null` to skip. The response envelope’s `data` contains the affected task UUID or `null`. A rejected invocation can return business code `409`, with a reason such as `trigger_target_locked`.
2. Use `POST /api/config/triggers/result` with `{ "taskId": "<UUID>" }` to fetch the current state. `running` and `cancelling` are active. `succeeded`, `failed`, `cancelled`, and `timed_out` are terminal. The backend retains the latest 200 results in memory. A missing or expired task returns business code `404`.
3. When entering a screen, read its configuration and UI hints. Use `GET /api/config/triggers/active` when the screen needs to recover in-flight commands, matching `configUid`, `group`, and `field` to the corresponding control. The UUID locates the task, while the UI hint’s `state` (`idle` or `busy`) controls the busy display. Apply backend visibility and `editable` per field.

## Configuration UI Synchronization

A configuration snapshot includes `uiSources` with the UUIDs of the entries and collections it reads. On a successful root transaction, the backend broadcasts `config.ui.changed` with the affected UUIDs in `nodeUids`. The event invalidates snapshots. The page then reloads values and hints from their normal HTTP endpoints. The same path handles trigger completion and changes to device availability.

For host pages, use `useConfigQuery` or `useConfigLoader` to collect the snapshot sources and subscribe the active query. Refresh visible queries on reconnect. While disconnected, the shared handler checks visible pages every five seconds. Use the committed response to update each button. For example, a device’s start button can be busy while a separate close button is disabled by the controller’s admission rules.

## Plugin Center Actions

Use `POST /api/plugin_registry/get` for installed plugins and `POST /api/plugin_catalog/get` for the marketplace. The registry entry UUID is the target for the `info.enable`, `info.disable`, `info.reload`, and `info.uninstall` triggers. Marketplace install and upgrade use `info.install` on the catalog entry UUID. The registry document’s `disableImpact` informs the disable confirmation and lists affected configuration domains.

The frontend integration lives in `frontend/src/composables/usePluginApi.ts`. When a shared dependency is loaded, installation can pause in `waiting_restart`. If the frontend owns the backend process, it calls `POST /api/plugin/installation/restart` until the idle backend accepts the handoff, then restarts the process. A boot failure can require another restart to restore the previous environment. Track installation state separately from the trigger outcome and refresh the lists after completion.

## Declarative Settings Page

After a plugin declares `config_class`, the "Settings" (「设置」) page in the Plugin Center is drawn by a generic renderer from the UI hints the backend sends down; the plugin writes no frontend. The read and write endpoints are `POST /api/plugin_config/get` and `POST /api/plugin_config/update`, and saving submits all values at once.

### Supported Controls

| `component` | Rendered as |
| --- | --- |
| `bool`, `boolean`, `checkbox`, `switch`, `toggle-button` | Switch |
| `input` (default; unknown values are also treated as it) | Text box; a password box when `secret` is true |
| `path` | Read-only text box plus a "Select" button; selects a directory when `path_kind` is `folder`, otherwise a file |
| `number` | Number box, supports `min` and `max` |
| `select`, `ref-select` | Dropdown; multi-select when `multiple` is true |
| `date` | Date picker, value `YYYY-MM-DD` |
| `button` | Button; a click triggers a command with value `true` |

A trigger button reads as idle or carries a task UUID. Clicking it sends a trigger command separately from saving ordinary fields.

### Field metadata the frontend currently recognizes

`field`, `component`, `secret`, `editable`, `path_kind`, `min`, `max`, `options`, `multiple`, `endpoint`, `deps`.

The backend's UI hints also have `masked`, `format`, `path_extensions`, `path_must_exist`, `ordered`, `widget`, and table definitions, which **the frontend does not use yet**; declaring them has no effect.

### Dynamic Options

A dropdown's options can point to an endpoint with `endpoint`. The frontend recognizes only the ones below; any other reports "option endpoint not wired up yet: &lt;endpoint&gt;" (「尚未接入选项接口：<端点>」):

- `/api/plugin/options/games`
- `/api/plugin/options/devices` (takes the value of the first item of `deps` as the game ID)
- `/api/plugin/options/plans`
- `/api/info/combox/plan`

Each item in `deps` is a dotted path of the form `<group>.<field>`, taken from the current form values. When the dependency value is empty or `-`, the device list is empty. Dynamic options always get a "Not linked" (「不关联」) option with value `-` at the front.

## Strings

A plugin carries `i18n/zh_CN.json`, `i18n/en.json`, and `i18n/ja.json` in its package directory (`zh-CN.json`, `en-US.json`, and `ja-JP.json` are also accepted). A file is a flat object. Every key starts with `plugins.<plugin ID>.`, and every value is a string:

```json
{
  "plugins.my_adapter.config.Cache.label": "缓存",
  "plugins.my_adapter.config.Cache.Enabled.label": "启用缓存",
  "plugins.my_adapter.config.Cache.Enabled.tip": "关闭后每次都重新下载"
}
```

::: warning
If any key lacks this prefix, any value is not a string, or a key is duplicated, the host fails to read the language resources.
:::

The settings page looks up strings by this rule:

```text
plugins.<插件ID>.config.<分组>.label            分组标题
plugins.<插件ID>.config.<分组>.<字段>.label     字段标签
plugins.<插件ID>.config.<分组>.<字段>.tip       字段提示
plugins.<插件ID>.config.<分组>.<字段>.options.<值>  选项文案
```

When strings are missing: the group title and label show the group name and field name, there is no tooltip, and options show the `label` the backend provides (or the value if there is none).

A plugin's display name, description, and tags come from the `auto-mas:i18n` block at the start of the README; see [Publishing Plugins](/en/developer/plugins/fundamentals/publish).

The main program currently has only three UI languages: Chinese, English, and Japanese. The host reads only the file names above, so language resources in other languages shipped by a plugin do not add language options.

## A Plugin's HTTP API

- Routes a plugin hands over via `api_router` (FastAPI) are mounted into the host at the paths they declare and enter OpenAPI.
- Endpoints declared with `@plugin_route` go through the generic gateway `/api/plugin/<plugin ID>/<path>` and do not enter OpenAPI.
- The frontend's API client is generated from OpenAPI (`frontend/src/api/`). **The plugin must be enabled when you run `yarn openapi`**; otherwise the output has neither this plugin's endpoints nor its config types.

## Static Registration of Adapter Pages

An adapter's managed-script edit page, account edit page, and plan table are written in the main repository and declared with `ScriptFrontendAdapter`:

```ts
export interface ScriptFrontendAdapter {
  metadata: ScriptMetadata // 图标、标签色、路由后缀、新建卡片
  type: string // 与后端 script_type_key 一致
  editors: { script: LazyPage; user: LazyPage }
  routes: RouteRecordRaw[] // 专项额外的路由
  plan?: {
    configType: string
    displayNameKey: string
    defaultName: string
    selectorTag: string
    reloadAfterSave: boolean
    supportsSimpleView: boolean
    tableComponent: Component
  }
}
```

Register it in the `adapters` list in `frontend/src/plugins/scriptAdapters.ts`, and also add `<type>: SCRIPT_PLUGIN_METADATA.<type>` to `SCRIPT_METADATA` in `frontend/src/utils/scriptMetadata.ts`. After registration:

- Edit page routes are generated automatically by `frontend/src/router/scriptEditorRoutes.ts`: `/scripts/:id/edit/<suffix>`, `/scripts/:scriptId/users/add/<suffix>`, and `/scripts/:scriptId/users/:userId/edit/<suffix>`. Extra routes in `routes` are added to the route table as they are.
- For an adapter with `plan`, its plan table type enters the plan table registry automatically.
- The card for creating a managed script is shown or hidden by the capability list.

For a complete example, see [Standalone Adapters](../script-adapters/standalone.md).

## What You Cannot Do Yet

- A plugin carrying its own pages or components.
- A brand-new managed type whose page the frontend has not compiled: the backend can register it, but the frontend cannot draw an edit page.
- A generic message channel for a plugin to drive the frontend UI. A plugin backend can send WebSocket messages through the SDK's `Publisher`, but they are received only if a page compiled in the main repository subscribes by `id` / `type`.
- Plugin icons. The plugin metadata (`pyproject.toml`, and the `auto-mas:i18n` block of the README) has no icon field, and the Plugin Center does not show plugin icons. The icon of a managed type is the image referenced by `metadata.logo` in the main repository's `entry.ts`, built with the desktop app.
