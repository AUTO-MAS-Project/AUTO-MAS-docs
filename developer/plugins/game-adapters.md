# 游戏适配插件

游戏适配插件在独立仓库实现一种游戏客户端或模拟器的发现、枚举与操作。宿主在 `app/core/game_manager.py` 管理游戏与设备的配置身份，调度层在 `app/task/game.py` 处理绑定、占用和收尾。

## 声明游戏类型

在插件类的 `decl` 中登记配置和控制器，在包入口将该类导出为 `PLUGIN`。`GameAdapterPlugin` 和声明类由 `auto_mas_core.adapters.game` 导出。

```python
class Plugin(GameAdapterPlugin):
    game_type_key = <游戏类型键>
    decl = GameTypeDecl(
        game_class=<游戏配置类>,
        device_class=<设备配置类>,
        control_class=<控制器类>,
        search=<安装发现函数>,
        resolve_installation=<安装路径解析函数>,
    )
```

- `game_type_key`：持久化游戏类型的标识，例如 `"mumu"`。发版后保持稳定。
- `decl`：该类型的完整声明。
  - `game_class`：继承 `GameEntry` 的具体游戏配置类。模拟器可继承 `EmulatorEntry`。
  - `device_class`：继承 `GameDeviceEntry` 的具体设备配置类。模拟器可继承 `EmulatorDeviceEntry`。
  - `control_class`：继承 `GameControl` 的具体控制器。模拟器可使用 `EmulatorControl`。
  - `search`：异步返回安装候选配置。找不到时返回空序列。
  - `resolve_installation`：可选。接收一个路径并返回规范化的安装路径。无法识别时返回 `None`，主要用于配置升级。

## 发现与导入

1. **探测安装**：插件在 `search()` 中查找真实存在的安装。探测可以读取安装信息，创建设备、持久化配置和启动进程留到确认导入或操作阶段。将耗时的同步探测移出事件循环。
2. **整理候选**：宿主调用启用类型的 `search()`，过滤类别不符和已登记的安装，缓存剩余候选。候选过期后重新搜索。
3. **登记配置**：用户选中候选后，宿主按 `gameId` 导入。只支持手动配置的类型可以通过统一的创建入口登记。通用 Windows 进程适配会在 `initialize_devices()` 中为用户提供的可执行文件生成设备条目。

有安装但搜不到时，依次核对插件启用状态、安装路径能否被其识别、该路径是否已经登记。发现异常可查宿主日志。

## 控制设备

1. **建立设备条目**：控制器提供设备枚举、刷新和运行状态。保持同一设备的身份稳定，原有托管和账号才能在刷新后继续使用绑定。
2. **声明可操作项**：`GameDeviceEntry` 提供 `action.open`、`action.close`、`action.show`、`action.hide`、`action.minimize` 和 `action.maximize` 的处理器。插件实现支持的操作，通过[触发器字段](../core/config.md#触发器字段)与[界面元数据](../core/config.md#界面元数据)控制按钮。模拟器可扩展 ADB、维护和环境调整能力。
3. **校验操作请求**：手动触发器传入设备 UUID 和 `TRIGGER_TICKET`，调度任务传入设备订阅票。多个不同按钮可能操作同一设备，控制器负责判断各动作能否执行。
4. **刷新运行状态**：设备占用或运行状态改变后调用 `runtime_changed()`，使宿主重新读取配置快照。相关界面见[配置界面同步机制](../core/config.md#配置界面同步机制)。

### 调试宿主接口

`app/api/games.py` 提供游戏管理接口，标准设备操作走配置触发器。

- **读取配置**：`POST /api/games/types` 返回已注册类型，`POST /api/games/get` 返回游戏和设备快照。
- **添加安装**：`POST /api/games/search` 返回候选，`POST /api/games/import` 导入候选，`POST /api/games/create` 按类型手动创建。
- **检查绑定**：`POST /api/games/binding/options` 返回托管和账号可绑定的游戏与设备。
- **执行动作**：`POST /api/config/triggers/write` 接收设备条目 UUID 与字段路径，例如 `action.open`。

## 调度中的设备绑定

托管提供默认游戏和设备，账号可覆盖。读取绑定和回收资源由宿主负责，实际的资源申请、设备配置和恢复由注册的控制器完成。

1. **申请设备**：托管任务取得游戏票据，账号在执行期间借用。需要等待设备时，任务日志显示占用情况。
2. **准备并使用设备**：控制器通过订阅钩子申请、归还资源，并据票据识别使用者。按专项需要提供环境设置和设备操作。
3. **恢复并释放**：任务结束时先尝试恢复临时设备设置，再释放票据和配置锁。恢复失败时记录日志。适配器需要保留下一次恢复所需的信息。

## 默认安装和升级

全新配置首次启动时，宿主尝试安装 MuMu、雷电、通用模拟器和通用 Windows 进程适配。ModAVD 按需安装。初始化后可在插件中心管理这些插件。

旧配置的游戏绑定由 `app/core/config/game_upgrade.py` 升级。宿主按规范化路径创建或复用游戏对象，将原托管和账号引用映射到相应设备。多个安装路径可生成多个游戏对象，无法识别的路径由升级诊断报告。

## 验证适配

1. **验证注册与发现**：[从源码启动](./fundamentals/develop.md)，验证同一安装只登记一次，刷新或重载后原有设备绑定仍有效。
2. **验证交互与调度**：分别执行手动触发和托管运行，检查设备占用、取消、恢复和日志。
3. **验证发行包**：移出同 ID 的本地源码后按[发布检查](./fundamentals/publish.md)验证插件。涉及升级时使用旧配置副本，分别核对托管默认绑定和账号覆盖。
