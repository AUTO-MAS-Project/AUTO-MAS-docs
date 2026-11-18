# Frontend and Backend Integration

::: warning
This page follows the `dev` branch implementation as of 2026-11-18. It will change once the dynamic frontend ships.
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

## Plugin Center Operations

Enable, disable, reload, uninstall, install, and upgrade all go through the same command entry; they are not separate endpoints.

1. `POST /api/operations/start` submits the command:

   ```json
   { "configUid": "<注册表或市场条目的 uid>", "group": "info", "field": "enable", "value": true }
   ```

   `field` is one of `enable`, `disable`, `reload`, `uninstall` (with `configUid` being the uid of the registry entry), or `install` (with the uid of the marketplace entry; upgrade also goes through it). `value` can be a boolean or a string, and the Plugin Center always passes `true`.
2. Poll with the returned `operationId` using `POST /api/operations/get`:

   ```json
   { "operationId": "<start 返回的 operationId>" }
   ```

   The request being accepted does not mean it is done. The intermediate `state` values are `queued`, `running`, and `cancelling`; go by the three final states `succeeded`, `failed`, and `cancelled`.
3. When done, the frontend re-reads the list.

The frontend wrapper is `frontend/src/composables/usePluginApi.ts`. Two behaviors to know:

- **Whether a button appears and whether it can be clicked is decided by the backend.** The button is shown only if the registry document's UI hints have the matching field, and the field's `editable` decides whether it can be clicked. The frontend does not infer it from the plugin state on its own.
- **Installing may require restarting the backend.** When an already-loaded shared dependency needs a different version, the install stops at "waiting for restart" (`waiting_restart`). The frontend repeatedly calls `POST /api/plugin/installation/restart`; the backend agrees only once its business is idle, and the frontend then restarts the backend. If the new process fails to start, it restarts once more to restore the old environment. This path is only taken when the frontend manages the backend process.

To read lists, use `POST /api/plugin_registry/get` (installed) and `POST /api/plugin_catalog/get` (marketplace). The confirmation content before disabling comes from `disableImpact` in the registry document, which lists the configuration domains that will be deleted (managed scripts, games).

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
| `dropdown-button` | Dropdown button; clicking an option triggers a command with the selected item as value; disabled when there are no options |

Button-type fields always read as the idle value, and "Save" does not trigger them. A click sends a separate command (`POST /api/operations/start`): `configUid` is the uid of the plugin config, and `group` and `field` are the group and field name that contain the field. The button responds only when `editable` in the UI hints is `true`.

### Field metadata the frontend currently recognizes

`field`, `component`, `secret`, `editable`, `path_kind`, `min`, `max`, `options`, `multiple`, `endpoint`, `deps`.

The backend's UI hints also have `masked`, `format`, `path_extensions`, `path_must_exist`, `ordered`, `widget`, and table definitions, which **the frontend does not use yet**; declaring them has no effect.

### Dynamic Options

A dropdown's options can point to an endpoint with `endpoint`. The frontend recognizes only the ones below; any other reports "option endpoint not wired up yet: &lt;endpoint&gt;" (「尚未接入选项接口：<端点>」):

- `/api/plugin/options/games`
- `/api/plugin/options/devices` (takes the value of the first item of `deps` as the game ID)
- `/api/plugin/options/plans`
- `/api/info/combox/emulator`
- `/api/info/combox/emulator/devices` (takes the value of the first item of `deps` as the emulator ID)
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

A plugin's display name, description, and tags come from the `auto-mas:i18n` block at the start of the README; see [Publishing Plugins](/en/plugin/start/publish).

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

For a complete example, see [Standalone Adapters](./standalone).

## What You Cannot Do Yet

- A plugin carrying its own pages or components.
- A brand-new managed type whose page the frontend has not compiled: the backend can register it, but the frontend cannot draw an edit page.
- A generic message channel for a plugin to drive the frontend UI. A plugin backend can send WebSocket messages through the SDK's `Publisher`, but they are received only if a page compiled in the main repository subscribes by `id` / `type`.
- Plugin icons. The plugin metadata (`pyproject.toml`, and the `auto-mas:i18n` block of the README) has no icon field, and the Plugin Center does not show plugin icons. The icon of a managed type is the image referenced by `metadata.logo` in the main repository's `entry.ts`, built with the desktop app.
