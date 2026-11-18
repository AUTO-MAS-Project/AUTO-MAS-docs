# 前端与后端的对接

::: warning
本页以主程序 `dev` 分支 2026-11-18 的实现为准。动态前端上线后这里会变。
:::

主程序的前端**不加载插件自带的前端代码**：没有 iframe、远程模块，也没有动态页面加载器。插件影响界面只有四条路。

| 通道 | 插件提供什么 | 前端怎么用 |
| --- | --- | --- |
| 能力列表 | 启用后登记的托管类型 | 决定新建托管里出现哪些类型；MAA 的页面和计划表新建入口也按它开放 |
| 声明式设置 | 插件配置类上的界面提示 | 通用表单渲染器画出「插件设置」页 |
| 词条 | 包内 `i18n/*.json` | 插件名称、设置项标签和提示 |
| 专项页面 | 没有（页面在主仓） | 主仓里按类型静态登记，随桌面应用构建 |

## 能力列表

- `GET /api/scripts/types` 返回当前可用的托管类型键。插件启用后它的 `script_type_key` 出现在里面，停用后消失。
- 后端在插件启用、停用或启用失败后经 WebSocket 推送 `plugin.capabilities.updated`（路由 id `Plugin`），前端收到后重新取一次。WebSocket 重连后也会重取。
- 前端封装在 `frontend/src/composables/useScriptCapabilities.ts`。新建托管对话框拿能力列表和前端 `SCRIPT_METADATA` 里的类型求交集：**后端启用了、前端也登记了这个类型，类型才会出现。**
- 页面路由目前只对 MAA 做了拦截：后端没有 `MAA` 时，打开 MAA 页面会跳到插件市场（`frontend/src/router/index.ts`）。计划表的新建入口同样只按能力过滤 MAA 计划表。其他专项的编辑页路由不看能力列表。

## 插件中心的操作

启用、停用、重载、卸载、安装、升级都走同一个命令入口，不是各自的接口。

1. `POST /api/operations/start` 提交命令：

   ```json
   { "configUid": "<注册表或市场条目的 uid>", "group": "info", "field": "enable", "value": true }
   ```

   `field` 取 `enable`、`disable`、`reload`、`uninstall`（`configUid` 用注册表条目的 uid），或 `install`（用市场条目的 uid，升级也走它）。`value` 可以是布尔或字符串，插件中心固定传 `true`。
2. 用返回的 `operationId` 调 `POST /api/operations/get` 轮询：

   ```json
   { "operationId": "<start 返回的 operationId>" }
   ```

   请求被受理不等于完成。`state` 的中间态是 `queued`、`running`、`cancelling`，以 `succeeded`、`failed`、`cancelled` 三个终态为准。
3. 完成后前端重新读取列表。

前端封装在 `frontend/src/composables/usePluginApi.ts`。两点行为要知道：

- **按钮出不出现、能不能点由后端决定。** 注册表文档的界面提示里有对应字段，按钮才显示；字段的 `editable` 决定能不能点。前端不根据插件状态自己推断。
- **安装可能需要重启后端。** 已经加载的共享依赖要换版本时，安装会停在「等待重启」（`waiting_restart`）。前端反复调 `POST /api/plugin/installation/restart`，后端业务空闲后才同意，前端随即重启后端；新进程启动失败则再重启一次恢复旧环境。只有由前端管理后端进程时才走这条路。

读列表用 `POST /api/plugin_registry/get`（已安装）和 `POST /api/plugin_catalog/get`（市场）。停用前的确认内容来自注册表文档里的 `disableImpact`，它列出会被删除的配置领域（托管、游戏）。

## 声明式设置页

插件声明 `config_class` 后，插件中心的「设置」页由通用渲染器按后端下发的界面提示画出来，插件不用写前端。读写接口是 `POST /api/plugin_config/get` 和 `POST /api/plugin_config/update`，保存时一次提交全部值。

### 支持的控件

| `component` | 画成什么 |
| --- | --- |
| `bool`、`boolean`、`checkbox`、`switch`、`toggle-button` | 开关按钮 |
| `input`（默认，未知值也按它处理） | 文本框；`secret` 为真时是密码框 |
| `path` | 只读文本框加「选择」按钮；`path_kind` 为 `folder` 时选目录，否则选文件 |
| `number` | 数字框，支持 `min`、`max` |
| `select`、`ref-select` | 下拉框；`multiple` 为真时多选 |
| `date` | 日期选择，值为 `YYYY-MM-DD` |
| `button` | 按钮，点击触发一条命令，值为 `true` |
| `dropdown-button` | 下拉按钮，点选项触发一条命令，值为选中项；没有选项时禁用 |

按钮类字段读出来恒为闲置值，「保存」不会触发它。点击时单独发一条命令（`POST /api/operations/start`）：`configUid` 是插件配置的 uid，`group` 和 `field` 是这个字段所在的分组和字段名。只有界面提示里 `editable` 为 `true` 时按钮才响应。

### 前端目前认的字段元数据

`field`、`component`、`secret`、`editable`、`path_kind`、`min`、`max`、`options`、`multiple`、`endpoint`、`deps`。

后端的界面提示里还有 `masked`、`format`、`path_extensions`、`path_must_exist`、`ordered`、`widget` 和表格定义，**前端现在还没用上**，声明了也不会有效果。

### 动态选项

下拉框的选项可以用 `endpoint` 指向一个接口，前端只认下面这几个，写别的会报「尚未接入选项接口：<端点>」：

- `/api/plugin/options/games`
- `/api/plugin/options/devices`（取 `deps` 第一项的值作游戏 ID）
- `/api/plugin/options/plans`
- `/api/info/combox/emulator`
- `/api/info/combox/emulator/devices`（取 `deps` 第一项的值作模拟器 ID）
- `/api/info/combox/plan`

`deps` 里每项是 `<分组>.<字段>` 形式的点分路径，从当前表单值里取。依赖值为空或 `-` 时设备列表为空。动态选项最前面总会加一个值为 `-` 的「不关联」选项。

## 词条

插件在包目录里带 `i18n/zh_CN.json`、`i18n/en.json`、`i18n/ja.json`（也认 `zh-CN.json`、`en-US.json`、`ja-JP.json`）。文件是一层扁平对象，每个键都以 `plugins.<插件ID>.` 开头，值都是字符串：

```json
{
  "plugins.my_adapter.config.Cache.label": "缓存",
  "plugins.my_adapter.config.Cache.Enabled.label": "启用缓存",
  "plugins.my_adapter.config.Cache.Enabled.tip": "关闭后每次都重新下载"
}
```

::: warning
有一个键不带这个前缀、值不是字符串或键重复，宿主读语言资源就会失败。
:::

设置页按这个规则取词：

```text
plugins.<插件ID>.config.<分组>.label            分组标题
plugins.<插件ID>.config.<分组>.<字段>.label     字段标签
plugins.<插件ID>.config.<分组>.<字段>.tip       字段提示
plugins.<插件ID>.config.<分组>.<字段>.options.<值>  选项文案
```

缺词条时：分组标题和标签显示分组名、字段名，没有提示，选项显示后端给的 `label`（没有就显示值）。

插件的显示名、简介、标签来自 README 开头的 `auto-mas:i18n` 块，见[发布插件](../start/publish)。

主程序目前只有中文、英文、日文三种界面语言，宿主只读上面这几个文件名，插件带了别的语言资源也不会多出语言选项。

## 插件的 HTTP 接口

- 插件用 `api_router` 交出的 FastAPI 路由按自己声明的路径挂进宿主，并进入 OpenAPI。
- 用 `@plugin_route` 声明的接口走通用网关 `/api/plugin/<插件ID>/<路径>`，不进 OpenAPI。
- 前端的接口客户端由 OpenAPI 生成（`frontend/src/api/`）。**运行 `yarn openapi` 时插件必须处于启用状态**，否则生成结果里没有这个插件的接口和配置类型。

## 专项页面的静态登记

专项的托管编辑页、账号编辑页、计划表写在主仓，用 `ScriptFrontendAdapter` 声明：

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

登记在 `frontend/src/plugins/scriptAdapters.ts` 的 `adapters` 列表里，同时在 `frontend/src/utils/scriptMetadata.ts` 的 `SCRIPT_METADATA` 里加 `<类型>: SCRIPT_PLUGIN_METADATA.<类型>`。登记后：

- 编辑页路由由 `frontend/src/router/scriptEditorRoutes.ts` 自动生成：`/scripts/:id/edit/<后缀>`、`/scripts/:scriptId/users/add/<后缀>`、`/scripts/:scriptId/users/:userId/edit/<后缀>`。`routes` 里的额外路由原样加进路由表。
- 有 `plan` 的专项，它的计划表类型自动进入计划表注册表。
- 新建托管的卡片按能力列表显示或隐藏。

完整例子见[独立专项适配](./standalone)。

## 现在做不到的

- 插件带自己的页面或组件。
- 一个前端没有编译对应页面的全新托管类型：后端能登记，前端画不出编辑页。
- 插件驱动前端界面的通用消息通道。插件后端可以经 SDK 的 `Publisher` 发 WebSocket 消息，但只有主仓里编译的页面按 `id` / `type` 订阅了才收得到。
- 插件图标。插件元数据（`pyproject.toml`、README 的 `auto-mas:i18n` 块）里没有图标字段，插件中心也不显示插件图标。托管类型的图标是主仓 `entry.ts` 里 `metadata.logo` 引用的图片，随桌面应用构建。
