# 插件基础

本章节讲述了插件的工作机制与开发方法，讲解了插件开发需要的所有信息。

## 术语解释

### 宿主

运行和管理插件的 AUTO-MAS 后端。宿主维护本机插件注册表和插件市场目录，负责发现、加载插件并编排插件生命周期，也通过 SDK 向插件开放主程序能力。

### 插件注册项 | PluginRecord

宿主管理插件的入口，除市场操作外的所有插件相关内容均可以通过插件注册表访问。每个插件对应一条注册项记录，注册项记录插件身份、来源、版本等插件信息，通过触发器提供所有生命周期操作的入口，持有宿主加载的插件类和启用时创建的插件实例。

### 插件市场条目 | PluginMeta

宿主展示插件发行信息和发起市场插件安装或更新的入口。每条市场条目记录一个可安装的发行版，包含插件 ID、发行包名、版本、名称和简介。用户通过条目的 `info.install` 触发器请求安装或更新，由插件管理器按条目指定的发行版执行操作。

### 插件基类 | BasePlugin

SDK 提供给所有插件类继承的公共基类。`BasePlugin` 定义插件的配置、服务依赖等声明入口，以及由插件按需覆盖的生命周期钩子。游戏适配和托管适配分别通过 `GameAdapterPlugin` 和 `ScriptAdapterPlugin` 继承这套接口。

### 插件类

开发者提供给宿主的具体实现类，直接或间接继承 `BasePlugin`。插件类通过类属性声明配置、依赖和对外能力，通过方法提供业务实现。宿主加载时从顶层包的 `__init__.py` 读取导出的 `PLUGIN`，将该类保存在插件注册项中，供启用时创建实例。

### 插件实例

宿主启用插件时，根据已加载的插件类创建的运行对象。宿主向实例传入注册项和插件配置，并为它绑定服务查询入口和后台任务组。宿主通过实例调用生命周期钩子和插件声明的服务方法。插件停用后，宿主释放实例，再次启用时创建新实例。

### 插件配置 | PluginConfig

插件保存自身设置的配置对象，类型由开发者继承 `PluginConfig` 定义，并通过插件类的 `config_class` 声明。宿主启用插件时构建并激活配置，将配置绑定到 `config/plugins/<插件 ID>.yaml`。插件实例通过 `self.config` 访问配置，未声明配置类时该属性为 `None`。

## 插件类字段与方法

插件通过类属性声明配置、服务和 HTTP 路由，通过生命周期钩子实现各阶段的业务操作。

### 类声明

```python
from auto_mas_core import BasePlugin


class <插件类>(BasePlugin):
    config_class = <插件配置类>
    requires = ("<必需服务名>",)
    wants = ("<可选服务名>",)
    provides_services = {"<提供的服务名>": "<服务方法名>"}
    api_router = <FastAPI 路由对象>


PLUGIN = <插件类>
```

- `<插件配置类>`：继承 `PluginConfig` 的类，默认值为 `None`。宿主启用插件时用它构建插件配置。
- `<必需服务名>`：插件必须使用的服务。宿主在启用插件前会自动级联启用对应服务提供方。
- `<可选服务名>`：插件按需使用的服务。此类服务缺失时，插件应当仍能正常工作。
- `<提供的服务名>: <服务方法名>`：插件提供的服务声明，服务方法应该是插件类的成员方法。
- `<FastAPI 路由对象>`：`APIRouter` 实例，提供插件所有 HTTP 接口。

### 生命周期钩子

`BasePlugin` 的生命周期钩子是异步空方法。插件类可以按需覆盖，宿主调用插件实例上的实现。各阶段的完整调用顺序见[插件生命周期](./runtime.md)。

```python
class <插件类>(BasePlugin):
    async def pre_enable(self) -> None:
        ...

    async def on_enable(self) -> None:
        ...

    async def pre_disable(self) -> None:
        ...

    async def on_disable(self) -> None:
        ...

    async def pre_reload(self) -> None:
        ...

    async def on_reload(self) -> None:
        ...

    async def pre_unload(self) -> None:
        ...

    async def on_unload(self) -> None:
        ...

    async def on_fail_cleanup(self, during, lifecycle_context, exc) -> None:
        ...

    async def on_teardown(self) -> None:
        ...
```

- **启用**：插件检查运行条件并建立业务资源。
  - `pre_enable()`：宿主创建实例后调用，插件可检查外部程序或运行环境。
  - `on_enable()`：宿主启用必需服务的提供方并激活插件配置后调用，插件可创建客户端或订阅信号。
- **停用**：插件检查能否结束当前工作，再释放启用时取得的资源。
  - `pre_disable()`：宿主处理停用前调用，插件可检查配置会话等业务操作是否已结束。
  - `on_disable()`：宿主在撤销服务和路由登记前调用，插件可关闭客户端、进程或订阅。
- **重载**：插件可以在旧实例上检查重载条件并保存跨重载所需的状态。
  - `pre_reload()`：宿主在旧实例上调用，插件可检查当前业务是否允许重载。
  - `on_reload()`：宿主在停用旧实例前调用，插件可保存新实例需要恢复的业务状态。
- **卸载**：宿主对仍有实例的插件调用卸载钩子。
  - `pre_unload()`：宿主在卸载前调用，插件可检查自己管理的文件是否允许清理。
  - `on_unload()`：宿主在插件停用后、移除发行包前调用，插件可清理卸载时才需移除的资源。
- **失败与退出**：插件在对应阶段释放业务资源。
  - `on_fail_cleanup(during, lifecycle_context, exc)`：宿主进入插件失败清理流程时调用，插件可使用出错阶段 `during`、操作上下文 `lifecycle_context` 和原异常 `exc` 清理已部分初始化的资源。
  - `on_teardown()`：宿主退出时对已启用的插件调用，插件可关闭外部进程、连接或订阅。
