# 独立专项适配

独立专项插件提供一种托管类型的后端配置、任务展开器和专用接口。插件放在独立仓库，主程序通过 SDK 加载。专项编辑页在主程序的 `frontend/src/views/` 编译和发布。基于 MaaFramework 项目接口的定制先参照[风味专项开发](./mfw/develop.md)。

## 建立插件项目

1. **确定插件身份**：按[开发与调试](../fundamentals/develop.md)配置发行包和导入包，调试时将源码放入主程序的 `plugins/<插件 ID>/`。以 MAA 为例，插件 ID 是 `maa_adapter`，PyPI 包名是 `auto-mas-maa-adapter`，托管类型键是 `MAA`。三者分别用于加载、分发和托管类型登记。
2. **导出插件类**：包入口导出 `PLUGIN` 与 SDK 的 `PLUGIN_API_VERSION`。宿主根据插件启用状态登记相应托管类型。
3. **选择参考实现**：MAA 声明计划表、预下载和关卡服务，奇想盒提供自动代理与配置模式。选与当前需求最接近的入口与调用方式。参见 [MAA 插件](https://github.com/AUTO-MAS-Project/AUTO-MAS-MAA-Adapter) 和[奇想盒插件](https://github.com/AUTO-MAS-Project/AUTO-MAS-Whimbox-Adapter)。

## 声明托管类型

从 `auto_mas_core` 导入 `ScriptAdapterPlugin`、`ScriptTypeDecl` 和宿主契约。每个插件声明一种托管类型。

```python
class Plugin(ScriptAdapterPlugin):
    script_type_key = <托管类型键>
    decl = ScriptTypeDecl(
        script_type=<托管配置类>,
        user_type=<账号配置类>,
        expander_class=<任务展开器类>,
        plan_type=<计划表配置类>,
        supported_modes=(<运行模式>,),
        expect_builder=<环境期望解析函数>,
        prefetch_check=<预下载检查函数>,
        activity_game=<活动游戏标识>,
        game_kinds=(<设备类别>,),
        supports_mod_avd=<是否支持 ModAVD>,
    )
```

- `script_type_key`：稳定的托管类型键，与前端声明的 `type` 一致。
- `decl`：交给宿主的类型和能力声明。
  - `script_type`：继承 `ScriptEntry`，保留 `info` 与 `users`。用 `collection(<账号配置类>)` 声明 `users`。
  - `user_type`：继承 `UserEntry`，保留 `info`，与 `users` 集合声明的类型一致。
  - `expander_class`：继承 `UserExpander`，向调度提供账号任务。执行器的职责见[调度逻辑](../../core/scheduling.md)。
  - `plan_type`：可选。声明计划表时，账号配置的 `info` 需要保留 `plan_id`。
  - `supported_modes`：非空元组。省略时支持以下三种模式，专项可只声明实际实现的模式。
    - `TaskMode.AUTO_PROXY`：运行自动代理。
    - `TaskMode.SCRIPT_VIEW`：查看脚本或其界面。
    - `TaskMode.SCRIPT_CONFIG`：打开脚本配置。
  - `expect_builder`：可选。从托管配置解析 `EmulatorExpect`。缺省时仅备份和恢复环境。
  - `prefetch_check`：可选。异步接收托管配置，返回 `DownloadRequest` 列表。返回空列表表示无需下载。
  - `activity_game`：可选。首页活动展示、活动提醒使用的游戏标识。
  - `game_kinds`：可选。限制游戏绑定的设备类别。`"emulator"` 对应模拟器，`"client"` 对应客户端。
  - `supports_mod_avd`：可选。声明此专项是否允许绑定 ModAVD。

## 实现配置、调度与接口

1. **声明配置字段**：复用 SDK 的配置节点和 UI 提示。托管、账号引用计划表等宿主配置集合时，在插件的 `pre_enable()` 中绑定配置类的 `related_config`。宿主随后激活领域配置并恢复引用。参见[配置声明](../fundamentals/schema.md)。
2. **接入任务展开**：按声明的模式展开账号和脚本任务。通过上游入口执行专项操作，在正常结束及异常路径恢复临时配置和资源。配置的生效顺序见[专项配置语义](./config-semantics.md)。
3. **挂载专用接口**：需要生成前端类型时，将 FastAPI 路由交给 `api_router`。只需通用网关的插件接口可以使用 `@plugin_route`。参见[插件的 HTTP 接口](../fundamentals/frontend.md#插件的-http-接口)。

## 前端页面

1. **声明页面入口**：在主程序 `frontend/src/views/<专项>/entry.ts` 提供 `ScriptFrontendAdapter`。`pluginId` 使用后端插件 ID，`type` 使用 `script_type_key`。在 `editors` 中登记托管和账号编辑页，按需提供 `routes` 与 `plan`。可参照 `frontend/src/views/maa/entry.ts`。
2. **登记专项**：将入口加入 `frontend/src/plugins/scriptAdapters.ts` 的 `adapters`，再从 `SCRIPT_PLUGIN_METADATA` 取该类型的元数据登记进 `frontend/src/utils/scriptMetadata.ts`。后端启用类型后，新建托管才会显示其入口。
3. **生成并验证 API 定义**：启用插件并启动后端，按[前端开发](../../core/api.md)生成接口定义。检查托管新建、编辑、账号编辑以及计划表配置。配置页的其他接入方式见[前端与后端的对接](../fundamentals/frontend.md)。

## 从内置专项迁出

这部分适用于已发布且有用户配置的内置专项。全新插件请从[建立插件项目](#建立插件项目)开始。

1. **确定发行边界**：明确目标主程序版本和实际发布的最低 SDK 版本，用对应发行包验证。插件从 SDK 导入宿主能力。
2. **保留配置身份**：保留用于持久化的类型名、UID、顺序和仍有效的字段。必须调整结构时在加载阶段转换，并让已有引用继续指向同一对象。
3. **登记升级依赖**：在主程序 `app/core/config/plugin_upgrade.py` 的 `_ADAPTER_PLUGINS` 中，将配置索引中的现有托管、计划表类型名映射到插件 ID 和发行包名。支持早期目录格式时同步更新 `required_adapter_plugins()` 的探测范围。宿主根据这些信息在领域配置激活前加载插件。
4. **切换宿主入口**：移除内置类型注册和重复的专项实现，把调用改到插件声明或[服务](../fundamentals/service.md)。更新静态前端登记，并重新生成 API 定义。
5. **按顺序验证与发布**：先交付插件 wheel，再交付移除内置实现的主程序版本。使用脱敏旧配置副本，并移出同 ID 的本地源码，检查启动时能否加载发行包，以及重启前后托管、账号和计划表的 UID、顺序、设置与引用是否均正确。

## 开发验证

- **插件仓库**：按插件自己的测试入口验证配置与任务。例如 MAA 测试通过 `AUTO_MAS_HOST_SOURCE` 选定主程序源码，再运行 `python -m pytest tests -q`。
- **主程序集成**：`tests/requirements-plugins.txt` 锁定插件 wheel 的版本和 SHA256，`scripts/prepare_test_plugins.py` 将依赖装入 `.test-plugins`。升级锁定的 wheel 时同步更新版本和哈希。
- **启停影响**：普通禁用会删除该类型的托管、账号和所声明的计划表，重新启用后需从备份恢复。卸载还会删除 `config/plugins/<插件 ID>.yaml`。请在插件 README 中说明备份与恢复方式。
