# 前端与后端的对接

宿主的 Vue 前端负责插件设置、插件中心和专项编辑页。插件提供配置声明、类型能力、语言文件和 HTTP 接口。定制编辑页由主程序编译，接入位置见本页末尾。

## 声明式设置页

插件声明 `config_class` 后，宿主通过 `POST /api/plugin_config/get` 读取配置及提示，并通过 `POST /api/plugin_config/update` 保存普通字段。前端使用 `frontend/src/components/DeclarativeControl.vue` 渲染字段。

- **控件**

  - `input`：文本输入，敏感提示使用密码显示。
  - `path`、`number`、`date`：路径、数字和日期控件。选择路径依赖桌面能力，数字和日期仍由后端校验。
  - `switch`、`select`：开关和下拉。下拉可以按字段提示启用多选。
  - `button`：配置触发器。按钮请求单独提交，字段提示提供可编辑状态。

- **使用字段提示**

  - 字段的可见、可编辑和忙状态根据后端快照展示。
  - 手写的复杂编辑交互由对应页面实现。设置页能呈现的字段以该渲染器实际使用的提示为准。

### 动态选项

`select(endpoint=...)` 可从宿主读取选项。依赖字段路径通过 `ui(deps=...)` 声明，见[配置声明](./schema.md#界面控件)。

- **选项入口**

  - `/api/plugin/options/games`：游戏。
  - `/api/plugin/options/devices`：所选游戏的设备，使用 `deps` 的第一个字段作为游戏标识。
  - `/api/plugin/options/plans`、`/api/info/combox/plan`：计划表选项。

- **空选择**

  - 选项值 `"-"` 表示不关联。
  - 游戏未选择时，设备下拉保留不关联选项。

## 配置触发器接口

触发命令由 `app/api/config_triggers.py` 提供。页面先保存操作参数，再独立调用触发器。

1. **写入命令**：向 `POST /api/config/triggers/write` 发送配置节点和字段。

   ```json
   {
     "configUid": "<配置节点 UUID>",
     "group": "<分组名>",
     "field": "<触发器字段名>",
     "value": true
   }
   ```

   - `configUid`：持有触发器的配置节点 UUID。
   - `value`：`true` 启动或复用，`false` 请求取消，`null` 跳过。响应 `data` 返回相关任务 UUID 或空值。配置锁等冲突会返回业务码 `409`。

2. **查询结果**：向 `POST /api/config/triggers/result` 发送 `{"taskId": "<任务 UUID>"}`。`running`、`cancelling` 为在途状态，`succeeded`、`failed`、`cancelled`、`timed_out` 为终态。取不到任务结果时返回业务码 `404`。
3. **恢复在途操作**：进入页面时通过 `GET /api/config/triggers/active` 查询在途任务。用配置节点、分组和字段定位控件，按后端提示展示忙状态和可编辑性。

## 配置界面的实时同步

配置快照使用 `uiSources` 列出它依赖的 Entry 和 Collection UUID。宿主在根事务成功后发送 `config.ui.changed`，在 `nodeUids` 中附上受影响的节点。

1. **登记来源**：页面使用 `useConfigQuery` 或 `useConfigLoader` 读取快照并登记依赖。
2. **刷新快照**：共享同步逻辑将匹配的查询标为过期，通过原 HTTP 接口重新读取值和提示。
3. **处理断线**：重连时刷新。断线期间，当前可见页面的活动配置查询每五秒补查。

## 插件中心的操作

插件中心通过 `frontend/src/composables/usePluginApi.ts` 调用宿主。

- **读取列表**

  - `POST /api/plugin_registry/get`：本机注册信息和 `disableImpact`。
  - `POST /api/plugin_catalog/get`：市场条目和可安装版本。

- **发起操作**

  - 注册条目的 `info.enable`、`info.disable`、`info.reload`、`info.uninstall` 是受管触发器。
  - 市场条目的 `info.install` 发起安装或升级。安装结果另行查询，等待重启时通过宿主的重启交接入口处理。

## 词条

插件包内的 `i18n/` 文件是扁平 JSON 对象。文件名支持 `zh_CN.json`、`en.json`、`ja.json` 及相应的区域别名。每个键只出现一次，并以 `plugins.<插件 ID>.` 开头。值为字符串。

```text
plugins.<插件 ID>.config.<分组>.label
plugins.<插件 ID>.config.<分组>.<字段>.label
plugins.<插件 ID>.config.<分组>.<字段>.tip
plugins.<插件 ID>.config.<分组>.<字段>.options.<选项值>
```

- **字段与选项**

  - 分组的 `label` 是分组标题，字段的 `label` 是控件标签。
  - `tip` 提供控件旁的说明。
  - `options.<选项值>` 提供选项标签。缺少字段或分组词条时，回退到相应名称。缺少选项词条时，回退到后端标签或选项值。

插件名称、简介和标签由 README 的市场声明提供，见[发布插件](./publish.md#readme-里的市场声明)。

## 插件的 HTTP 接口

接口由 `app/plugin/http.py` 管理。宿主在插件启用后开放注册路由，暂停调用时按受管调用的生命周期处理请求。

- **类型化接口**

  ```python
  from fastapi import APIRouter

  from auto_mas_core import BasePlugin

  <路由器> = APIRouter(prefix="<接口前缀>")

  @<路由器>.get("<查询路径>")
  async def <处理函数>() -> <响应类型>:
      return <响应结果>

  class PLUGIN(BasePlugin):
      api_router = <路由器>
  ```

  - `api_router`：按声明的路径挂载并进入后端 OpenAPI。
  - 从当前本地后端[生成前端 API](../../core/api.md)时，先启用对应插件。

- **通用网关**

  ```python
  from auto_mas_core import BasePlugin, plugin_route

  class PLUGIN(BasePlugin):
      @plugin_route("<相对路径>", methods=["GET"])
      async def <处理方法>(self) -> dict:
          return <响应数据>
  ```

  - 路径为 `/api/plugin/<插件 ID>/<路径>`。可以省略 `methods` 使用 `GET`。
  - 处理方法可接收一个 `Request` 参数，也可以无参。网关接口由前端按其契约调用，OpenAPI 生成使用上方的类型化接口。

## 专项页面的静态登记

主程序通过 `frontend/src/plugins/scriptAdapters.ts` 的 `ScriptFrontendAdapter` 登记专项页面。

- **类型及页面**

  - `pluginId`：所属插件 ID，用于将注册条目与托管、计划类型及图标关联。
  - `type`：后端的 `script_type_key`。
  - `metadata`、`editors`、`routes`：托管类型的展示信息、托管与账号编辑器、额外路由。
  - `plan`：专项实际支持计划表时声明，供主程序注册类型和表格。

- **可用范围**

  - 新建托管使用 `GET /api/scripts/types` 的能力与前端已登记类型的交集。
  - 宿主通过 `plugin.capabilities.updated` 通知重新查询。重连后也会重新读取。
  - 带编辑器和计划表的完整接入见[独立专项适配](../script-adapters/standalone.md)。
