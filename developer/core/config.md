# 配置框架

AUTO-MAS 主程序使用自研的配置框架，统一处理配置声明、校验和 YAML 持久化。框架为后端提供事务、业务锁和配置变更信号等机制，保证后端业务状态与配置字段实时同步；为前端提供界面提示字段与页面变更通知，支持前端界面与后端字段值实时同步。

配置框架源代码位于 `app/config/`，以 Pydantic 模型声明配置。主程序通过 `from app.core import Config` 获取配置项管理入口，配置框架操作通过 `from app.config import config_manager` 调用。

## 导览

### 源码文件结构

::: tip 提示
仅开发或研究配置框架具体实现方法时阅读，首次贡献者建议直接跳过本段。
:::

- `app/config/`

  - `core/`：核心配置模型与机制的定义，包括配置框架管理器、配置节点、配置分组、配置对象、配置集合、暂存操作。
  - `fields/`、`types.py`、`shortcuts.py`：基础字段与机制定义，包括引用字段、加密、UI 页面提示字段、历史字段兼容及标准字段类型。
  - `signals.py`：配置对象与配置集合的信号事件。
  - `wire.py`：定义 API 请求和响应的 Pydantic 序列化与反序列化规则。

- `app/core/config/`

  - `manager.py`：提供 `Config` 单例的类，负责提供配置项访问入口与组织配置相关生命周期任务。
  - `upgrade.py`、`game_upgrade.py`、`plugin_upgrade.py`：配置文件版本升级的迁移方法。
  - `backup.py`：后端配置信息备份方法。

- `app/models/`

  - `config/`：各业务域的配置基类及其字段具体声明，是主程序业务字段、默认值和校验规则的主要声明位置。
  - `schema.py`：跨模块公共接口契约。端点专用请求、响应模型位于对应 `app/api/` 模块顶层。

### 最小完整示例

以下示例展示一个关联文件的配置对象从构造、激活到修改和保存的完整过程。

```python
import asyncio
from pathlib import Path
from pydantic import Field

from app.config import ConfigEntry, ConfigGroup, config_manager


class ExampleConfig(ConfigEntry):
    """示例配置"""

    class InfoFields(ConfigGroup):
        Name: str = Field(default="默认名称", description="显示名称")
        Enabled: bool = Field(default=True, description="是否启用")

    Info: InfoFields = Field(default_factory=InfoFields)


async def main() -> None:

    # `file=` 登记持久化文件路径
    config = ExampleConfig.build(file=Path("config/example.yaml"))
    # 激活配置对象
    await config.activate()

    # 将赋值操作暂存
    config.Info.Name = "新的名称"
    # 提交 config 节点暂存的所有操作
    await config.commit()

    assert config.Info.Name == "新的名称"

    # 停止持久化防抖，立即将配置落盘
    await config_manager.flush()
    # 注销持久化信息的登记
    config_manager.unregister_root(config)


if __name__ == "__main__":
    asyncio.run(main())
```

## 配置信息组织结构

::: tip 导读
本段描述了在本项目中，配置信息是如何被组织的，以及相关的代码实现。
:::

#### 逻辑设计

<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 880 390" role="img" aria-labelledby="config-tree-title config-tree-desc" style="display:block;width:100%;max-width:880px;margin:1rem auto">
  <title id="config-tree-title">配置模型</title>
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
    <text x="195" y="42">配置集合 · 根</text>
    <text x="100" y="115">配置对象 A1</text>
    <text x="290" y="115">配置对象 A2</text>
    <text x="100" y="194">配置集合 · 字段 B</text>
    <text x="290" y="194">配置集合 · 字段 B</text>
    <text x="100" y="228" fill="#475569">空集合</text>
    <text x="92" y="271">配置对象 C1</text>
    <text x="217" y="271">配置对象 C2</text>
    <text x="342" y="271">配置对象 C3</text>
    <text x="429" y="271" font-size="22">…</text>
    <text x="217" y="303" font-size="22">⋮</text>
    <text x="665" y="42">配置对象 · 根</text>
    <text x="665" y="98">配置集合 · 字段 D</text>
    <text x="565" y="152">配置对象 E1</text>
    <text x="765" y="152">配置对象 E2</text>
    <text x="565" y="206">配置集合 · 字段 F</text>
    <text x="765" y="206">配置集合 · 字段 F</text>
    <text x="565" y="244" font-size="22">⋮</text>
    <text x="765" y="244" font-size="22">⋮</text>
  </g>
  <path d="M74 367H100" fill="none" stroke="#28548c" stroke-width="2" marker-end="url(#config-tree-field-arrow)"/>
  <path d="M478 367H504" fill="none" stroke="#7c3a12" stroke-width="2" stroke-dasharray="5 4" marker-end="url(#config-tree-member-arrow)"/>
  <g fill="#172033" font-size="16" dominant-baseline="central">
    <text x="114" y="340">Collection 节点</text>
    <text x="518" y="340">Entry 节点</text>
    <text x="114" y="367">固定字段不可增减</text>
    <text x="518" y="367">成员可增减</text>
  </g>
</svg>

配置框架的底层是一种**递归定义的树形数据模型**，由 `Entry` 与 `Collection` 两类节点交替嵌套构成。

- **Entry**

  承载数据的节点，可以通过该节点访问到具体的数据信息，该节点的下属数据的结构与下属节点的结构必须固定，无法在软件运行时随时增删。

- **Collection**

  仅负责组织配置树结构的节点，仅可以通过该节点访问下属节点，下属节点在运行时可自由增删。

#### 具体实现

为实现该数据模型，框架在业务层面抽象出了三个逻辑层级。

- **L3 配置分组**：对应树结构中 `Entry` 节点中保存的具体数据信息，组织 `Entry` 中用途相关的字段。例如，托管的 `Info` 分组保存托管名称、启用状态等基础信息，`Run` 分组保存重试次数上限等运行设置。
- **L2 配置对象**：对应树结构中的 `Entry` 节点，表示一项独立配置，一个配置对象包含多个配置分组与多个配置集合。例如，一个托管脚本设置就是一个配置对象，不仅包含 `Info`、`Run` 等具体配置信息，还可以包含下属用户集合 `UserData`。
- **L1 配置集合**：对应树结构中的 `Collection` 节点，一个集合包含多个特定类型的配置对象，使用 UUID 索引成员。例如，多个托管脚本组成的列表就是一个配置集合。

```text
ScriptConfig（L1）
└─ <一项托管脚本>（L2）
   ├─ Info（L3）：Name、Status
   ├─ Run（L3）：Mode、RetryCount、Tasks
   └─ UserData（L1）
      └─ <一个用户>（L2）
         ├─ Info（L3）：Name、ID
         └─ Data（L3）：LastRunTime
```

### 使用方法

#### 配置节点 | Node {#config-node}

`ConfigNode` 是配置对象（`ConfigEntry`）和配置集合（`ConfigCollection`）的父类，定义了配置节点通用的方法。

配置节点类基于 `Pydantic` 实现，不仅可以用于构造配置实例，还可以用作 `FastAPI` 接口的类型声明。

- **属性**

  - `uid`：节点的 UUID，用于标识该节点。
  - `parent`：直接父节点，根节点的值为 `None`。
  - `root`：所在配置树的根节点。当前节点是根节点时，值为自身。
  - `is_root`：当前节点是否为根节点。
  - `persist_path`：绑定的 YAML 文件路径。未登记文件的根节点及其所有子节点的值均为 `None`。
  - `activation_state`：节点的激活状态。`INACTIVE` 表示冷态，`INITIALIZING` 表示正在初始化，`ACTIVE` 表示已激活。
  - `deleted`：节点是否已被软删除。
  - `is_locked`：节点是否持有共享锁或独占锁。持有对应独占锁票据的上下文仍可提交修改。

- **构造**

  配置节点构造后返回的是冷态配置实例，冷态配置实例仅具有 Pydantic 实例的基本能力，配置框架相关能力都无法使用，可用作 `FastAPI` 返回值。

  - 从字典构造

    使用包含配置字段值的字典构造，传参时立即校验配置值，可以直接在冷态对象中读到传入的配置值。

    ```python
    <实例名> = <配置节点类>.model_validate(<配置字典>)
    ```

  - 完整构造

    使用配置节点的完整参数构造，激活时校验，激活后才能直接读到传入的配置值。

    ```python
    <实例名> = <配置节点类>.build(
        parent=<父节点>,
        uid=<节点 UUID>,
        payload=<暂存的配置字典>,
        file=<绑定的 YAML 文件路径>,
        ...,
    )
    ```

    - `parent=`：默认 `None`，表示独立的根节点。该字段用于在框架内建立节点关联，通常无需手动传入。
    - `uid=`：默认 `None`，表示自动生成 UUID。通常无需手动传入。
    - `payload=`：激活时加载的配置字典。不会直接写入到冷态实例，激活前无法直接读取。与 `file=` 互斥。
    - `file=`：绑定的配置文件路径，仅根节点可用。激活时从该路径加载配置值，绑定后配置更改会自动持久化到该文件。与 `payload=` 互斥。

- **激活**

  冷态配置实例通过异步的激活方法转为热态，正式加载相关配置内容，激活配置框架相关能力，承载配置信息与基于配置变动的业务逻辑。

  激活操作仅冷态配置实例可用，一般只需要激活根节点，子节点会自动递归激活。激活时加载配置的来源优先级为：

  1. `payload=`
  2. `file=`
  3. 冷态实例当前的配置值

  ```python
  await <节点>.activate(
      regenerate_uuids=<是否为所有子配置节点重新生成 UUID>,
      fast_init=<是否尝试快速激活>,
  )
  ```

  - `regenerate_uuids=`：一般在复制配置实例时启用。即使成员 UUID 重新生成，引用字段仍保留输入值。需要引用副本中的成员时，调用方需按 UUID 映射转换引用值。
  - `fast_init=`：启用后，对本次激活涉及的 Entry，尝试在一个局部事务里直接完整初始化所有配置字段，以减小频繁创建事务的开销。事务中存在字段异常报错回退后，恢复原有逐字段建立事务的初始化方法，集合成员仍各自使用独立的初始化边界。

  初始化失败的处理方式和引用依赖的加载顺序见[初始化顺序](#初始化顺序)。

- **提交**

  热态配置实例的普通字段赋值和集合修改操作都会先暂存，再通过 `commit()` 应用，提交前暂存的修改不生效。配置框架提供了事务担保，同一事务内操作具有原子性。

  - 默认事务边界

    直接提交当前节点的暂存操作，会使用默认事务边界。每项操作及其直接触发的关联配置修改构成一个事务。一次 `commit()` 中，**某项操作失败时，其他成功的操作仍会正常生效**。

    ```python
    await <节点>.commit()
    ```

  - 显式事务边界

    使用 `transaction()` 块显式指定事务边界。整个 `transaction()` 块及其直接触发的关联配置修改构成一个事务。`transaction()` 块中，**异常传出事务边界时，本次事务内的配置修改全部回退**。事务结束前需要保证**本事务内暂存的修改已提交**，否则会报错并触发事务回退。

    ```python
    async with config_manager.transaction():
        <节点 A>.<分组>.<字段> = <新值>
        await <节点 A>.commit()
        <节点 B>.<分组>.<字段> = <新值>
        await <节点 B>.commit()
    ```

  工作区、嵌套事务和并发行为见[事务机制](#事务机制)。

- **业务锁**

  热态配置节点实例可以加锁。普通字段读取和集合遍历无需持有票据，加锁状态仍允许读取。锁仅影响普通字段和集合操作的写入权限。

  - **S 锁（共享锁）** 允许多个 S 锁共存，锁定范围内的普通写入一律拒绝。
  - **X 锁（独占锁）** 与其他 S 锁和 X 锁互斥。只有持有对应票据的执行上下文可以暂存并提交修改。

  获取锁时不会等待，遇到冲突时 `lock_s()` 和 `lock_x()` 抛出 `ValueError`，`try_lock_x()` 返回 `None`。取锁成功后，用原票据在签发节点上解锁。

  ```python
  <共享锁票据> = await <节点>.lock_s(cascade=<是否锁定子节点>)
  <独占锁票据> = await <节点>.lock_x(cascade=<是否锁定子节点>)
  <独占锁票据或 None> = await <节点>.try_lock_x(cascade=<是否锁定子节点>)
  await <节点>.unlock(<原锁票据>)
  ```

- **导出配置**

  - Pydantic 导出

    ```python
    <配置字典> = <节点>.model_dump(
        mode="json", context=ExportContext(audience=<导出用途>)
    )
    ```

    - `mode=`：指定导出模式：
      - `"json"`：将所有字段转为 JSON 文件支持的表示方式。
      - `"python"`：保留 Python 原生类型。
    - `audience=`：指定 JSON 模式的导出用途：
      - `"api"` 是默认值，用于接口输出。包含虚拟字段和触发器字段，按界面规则隐藏或掩码字段，加密字段以明文导出。
      - `"persist"` 用于持久化。不包含虚拟字段或触发器字段，加密字段保持密文。
      - `"export"` 用于备份和迁移。不包含虚拟字段或触发器字段，加密字段以明文导出。

  - 持久化导出

    等效于 `.model_dump(mode="json", context=ExportContext(audience="persist"))`。

    ```python
    <持久化字典> = await <节点>.to_dict()
    ```

#### 配置声明

配置树有两类节点。配置对象继承 `ConfigEntry`，配置集合使用 `ConfigCollection[T]`，两者共用 `ConfigNode` 基类。配置分组继承 `ConfigGroup`，用于组织配置对象中的字段。

- **配置分组**中的字段按 Pydantic 的方式声明，可以用 `Field(...)` 指定默认值、约束和描述。分组类可以单独定义，也可以定义在 Entry 类体内。
- **配置对象**通过类型注解声明分组字段，可以用 `Field(default_factory=...)` 为每个实例创建分组。
- **嵌套集合**作为 Entry 的字段，必须用 `collection(...)` 声明。字段注解中的 `ConfigCollection[...]` 为类型检查器提供成员类型，`collection(...)` 传入的类型参数决定运行时初始允许的成员类型。

```python
from pydantic import Field

from app.config import ConfigCollection, ConfigEntry, ConfigGroup, collection


class <配置对象类>(ConfigEntry):
    class <分组类>(ConfigGroup):
        <字段名>: <字段类型> = Field(default=<默认值>, description=<字段描述>)

    <分组字段名>: <分组类> = Field(default_factory=<分组类>)

    <集合字段名>: ConfigCollection[<成员类型 A> | <成员类型 B> | ...] = collection(
        <成员类型 A>,
        <成员类型 B>,
        ...
    )
```

如果集合允许多个成员类型，使用 `ConfigCollection[<成员类型 A> | <成员类型 B>]` 注解，并将每个成员类型分别传给 `collection(<成员类型 A>, <成员类型 B>)`。

配置对象类的子类可以覆写父类的分组或集合字段。收窄可变字段的类型时，静态检查器可能报错，可以在对应声明处加上豁免注释并说明原因。

#### 配置对象 | Entry

- **构造**

  除[配置节点 | Node](#config-node)中说明的构造方法外，Entry 还可以按分组字段名直接传参构造。

  ```python
  <实例名> = <配置对象类>(<分组字段名>=<分组实例>, ...)
  ```

- **读取与修改字段**

  通过配置分组读取或修改普通字段。对冷态实例的赋值会立即校验并生效，对热态实例的赋值会暂存，提交时验证并生效。

  修改热态实例的列表、字典等可变类型字段时，先复制实例再修改，修改完成后将整个实例赋回字段。

  ```python
  <配置值> = <配置对象>.<分组名>.<字段名>
  <配置对象>.<分组名>.<字段名> = <新值>
  await <配置对象>.commit()
  ```

- **批量修改**

  一次性批量提交字段修改的方法，会将同类冷态实例显式提供的普通字段值赋值并提交，然后处理触发器激发，不会处理虚拟字段或配置集合字段。通常用于处理接口传入的配置更新请求。

  注意，在显式事务外，批量修改采用的是默认事务边界且会自动提交，该操作**不具备原子性**。

  ```python
  await <配置对象>.update(<同类冷态对象>, check_visibility=<是否遵循界面提示>)
  ```

  - `check_visibility=`：是否遵循界面提示信息的指引，跳过界面上隐藏或禁用的普通字段，默认为 `True`。

- **激发触发器**

  配置字段中有一类触发器字段，可以通过更新该字段的值，触发一段任务操作。你可以在事务外向配置实例发送触发器命令直接触发对应任务。

  ```python
  <任务 UUID 或 None> = <配置对象>.write_trigger(
      group=<分组名>,
      field=<字段名>,
      value=<命令值>,
  )
  ```

  - `value=`：不同的值对应不同的命令
    - `None` 跳过执行，返回 `None`
    - `True` 启动任务并返回任务 UUID，已有任务仍在运行时返回已有任务 UUID
    - `False` 请求取消并返回当前任务 UUID，空闲时返回 `None`

#### 配置集合 | Collection

- **构造**

  除[配置节点 | Node](#config-node)中说明的构造参数外，集合的完整构造还接受以下参数：

  ```python
  <实例名> = ConfigCollection[<成员类型>].build(
      entry_types=<成员类型或类型列表>,
      name=<引用名称>,
      order=<顺序列表>,
      data=<成员映射>,
      ...,
  )
  ```

  - `entry_types=`：构造时初始登记的成员类型，可以传入一个具体 Entry 类或由多个 Entry 类组成的列表。省略时使用集合子类预设的类型。
  - `name=`：为根集合登记唯一的引用名称，供 `ref(...)` 查找。引用名称与 YAML 文件名独立。
  - `order=`：冷态集合的顺序列表，每项包含成员的 UUID 和类型名。
  - `data=`：冷态集合的成员映射，以 UUID 为键。

- **添加成员**

  通过 `add()` 暂存新增操作，立即取得预分配的成员 UUID。集合提交成功后，才可以用该 UUID 读取新成员。

  ```python
  <成员 UUID> = <配置集合>.add(
      entry_type=<成员类型>,
      payload=<初始配置字典>,
  )
  await <配置集合>.commit()
  ```

  - `entry_type=`：要新增的成员的类型，该类型必须是已登记的成员类或类名字符串。
  - `payload=`：新成员的初始配置字典，在提交时会自动使用该配置激活成员。
  - `uid=`：新成员的 UUID。省略时自动生成。
  - `regenerate_uuids=`：默认为 `False`。启用时为嵌套集合中的成员重新生成 UUID，通常在复制包含嵌套集合的配置时使用。引用字段保留传入的值。

- **读取成员**

  配置集合可视为以 UUID 为键的有序字典，可以使用字典的 `get()`、`keys()`、`values()` 和 `items()` 方法访问配置集合的成员。

  ```python
  <配置对象> = <配置集合>[<成员 UUID>]
  <配置集合> = <配置对象>.<集合字段名>
  ```

- **删除成员**

  ```python
  <配置集合>.remove(uid=<成员 UUID>)
  await <配置集合>.commit()
  ```

- **调整成员顺序**

  ```python
  <配置集合>.set_order(order=<成员顺序序列>)
  await <配置集合>.commit()
  ```

  - `order=`：按目标顺序排列的 UUID 对象或 `CollectionOrderItem` 序列，必须包含集合中的全部成员，且每个 UUID 只出现一次。

- **动态成员类型**

  配置集合支持新增、移除或重载已登记的成员类型。静态类型提示仍由 `ConfigCollection[T]` 中的 `T` 决定。

  使用 `ConfigCollection[T].build(...)` 构造集合时，如果方括号里的 `T` 是**单个 ConfigEntry 类**，动态更改成员类型时，额外要求传入的类**必须是 T 本身或其子类**。

  **重载成员类型** 与 **替换成员类型** 成功时，会标记旧实例为 `软删除` 并创建新实例。读取 `软删除` 实例的配置字段或通过配置框架修改旧实例时，会抛出 `DeletedNodeError`。在重载后应按 UUID 重新获取实例，并重新绑定实例级订阅（当前旧实例上的订阅不会被自动重载到新实例中）。

  - **新增成员类型**

    配置集合按类名登记类型。传入已经登记的同一个类时会跳过，传入与已登记类同名的另一个类时会报错。

    ```python
    <配置集合>.add_type(entry_cls=<新成员类型>)
    await <配置集合>.commit()
    ```

  - **移除成员类型**

    移除成员类型时会自动在同一事务内删除该类型的全部成员。任一成员被锁定或删除守卫拒绝删除时，整次操作失败。

    ```python
    <配置集合>.remove_type(entry_type=<已登记的成员类型或类型名>)
    await <配置集合>.commit()
    ```

  - **重载成员类型**

    更新某一成员类型的类型定义，会使用旧有配置数据重建该类型的全部成员，并保留成员 UUID 和顺序。

    ```python
    <配置集合>.reload_type(entry_type=<同名的新成员类型或已登记的类型名>)
    await <配置集合>.commit()
    ```

    - `entry_type=`：传入与已登记类型同名的新类时，更新登记并用新类重建成员；传入类名字符串时，使用当前登记的类重建；直接传入当前登记的类时会跳过。

  - **替换成员类型**

    用指定类型和配置字典重建单个成员，保留原成员的 UUID、顺序位置和传入配置中的嵌套成员 UUID。

    ```python
    <配置集合>.replace_type(
        uid=<成员 UUID>,
        entry_type=<目标成员类型>,
        payload=<转换后的配置字典>,
    )
    await <配置集合>.commit()
    ```

## 扩展配置字段

除标准的 `Pydantic` 字段外，配置框架提供了扩展配置字段，用以满足更复杂的需求。

_本节中的读写指直接对字段对象的读写，导出指生成 API 返回值与持久化导出数据。_

### 字符串扩展

```python
<字段名>: <字符串扩展类型> = <默认字符串>
```

| 类型                    | 输入处理                                                                                                                                                                                                                                     |
| ----------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `JsonDictString`        | 保留有效的 JSON 对象字符串，其余输入回退为 `"{ }"`                                                                                                                                                                                           |
| `JsonListString`        | 保留有效的 JSON 数组字符串，其余输入回退为 `"[ ]"`                                                                                                                                                                                           |
| `UrlString`             | 保留包含协议和网络位置的 URL，其余输入清空                                                                                                                                                                                                   |
| `KeyboardKeyString`     | 按键名转为小写，非法输入回退到字段声明的默认值。声明时应提供合法键位作为默认值                                                                                                                                                               |
| `WindowsNameString`     | 清理 Windows 名称中的非法字符和首尾空白及点号，长度限制为 255 个字符，空名称或保留名回退为 `"默认用户名"`                                                                                                                                    |
| `CliArgumentString`     | 检查命令行参数语法，合法时保留原字符串，语法错误时清空                                                                                                                                                                                       |
| `CliArgumentListString` | 用于在同一个字段里为不同用途保存多组启动参数，格式为 `<程序相对路径 1>%<命令行参数 1>\|<程序相对路径 2>%<命令行参数 2>\|...`。`\|` 分隔各组，每组的 `<程序相对路径>%` 可省略，每组首个 `%` 后的内容均视为参数。校验时跳过空段，检查参数合法性 |

### 路径字段

路径字段将输入归一化为绝对 `Path`，空值或不符合规则的路径会清空为 `None`。归一化会展开环境变量和用户目录，并解析符号链接及 Windows 快捷方式。

- **预设路径类型**

  ```python
  <字段名>: <路径类型> = None
  ```

  | 路径类型         | 接受的路径                                                                                                          |
  | ---------------- | ------------------------------------------------------------------------------------------------------------------- |
  | `FilePath`       | 已存在的文件，排除盘符根目录、受保护的系统路径、AUTO-MAS 工作目录及其上、下级路径                                   |
  | `FolderPath`     | 已存在的目录，排除盘符根目录、受保护的系统路径、AUTO-MAS 工作目录及其上、下级路径                                   |
  | `ScriptRootPath` | 已存在的脚本目录，排除盘符根目录、受保护的系统路径                                                                  |
  | `ExecutablePath` | 已存在的 `.exe`、`.bat`、`.cmd` 或 `.com` 文件，排除盘符根目录、受保护的系统路径、AUTO-MAS 工作目录及其上、下级路径 |
  | `LoosePath`      | 文件或目录，不要求已存在                                                                                            |

- **自定义路径规则**

  ```python
  <字段名>: Annotated[
      Path | None,
      path_policy(
          kind=<路径类型>,
          extensions=<后缀序列>,
          must_exist=<是否要求存在>,
          blocked_dirs=<阻止目录序列>,
      ),
  ] = None
  ```

  - `kind=`：`"file"`、`"folder"` 或 `"any"`，分别接受文件、目录或两者。
  - `extensions=`：允许的文件后缀列表或元组，默认不限，目录类型不能设置该项。点号可省略，忽略大小写。`kind="any"` 时，后缀只约束已确认是文件的路径。
  - `must_exist=`：默认为 `False`，允许尚未创建的路径。要求路径存在且允许目录时，如果原路径不是目录，会尝试去掉最后一个后缀，若存在该目录则采用。
  - `blocked_dirs=`：传入目录路径或 `PathBlockPreset` 枚举成员组成的列表或元组，省略时不额外排除目录。

    - 目录路径项：排除目录本身及其下级路径
    - `PathBlockPreset.ROOTS`：文件系统根目录
    - `PathBlockPreset.SYSTEM`：Windows 系统目录及其上、下级路径
    - `PathBlockPreset.PROGRAM_FILES_ROOTS`：`Program Files` 与 `Program Files (x86)` 目录
    - `PathBlockPreset.APP`：AUTO-MAS 工作目录及其上、下级路径

    还可以直接传入预设的目录元组常量，`SYSTEM_BLOCKED_DIRS` 包含 `ROOTS`、`SYSTEM` 和 `PROGRAM_FILES_ROOTS`；`STANDARD_BLOCKED_DIRS` 在此基础上加入 `APP`。

### 日期时间字段

为防止开发者忘记处理字段的时区，以及更便捷地处理不同地区服务器的不同跨天时间点，配置框架为原有的 `datetime` 添加了自动时区处理能力。_例如，若某服务器在北京时间凌晨四点切换为下一天，可以将对应的日期字段设定为 `UTC+4`，截取日期时就能直接区分不同的服务器日期。_

- **写入**：带时区的输入自动换算成 UTC 再保存，无时区的输入按 UTC 解释。
- **读取**：自动换算为预设时区。
- **导出**：直接使用 UTC 时区值。

```python
<字段名>: Annotated[datetime, tz(<预设时区或时区方法>)] = <默认时间>
```

- `tz(<预设时区或时区方法>)`：未声明 `tz(...)` 时视为 `UTC+8`。可以直接传入 `datetime.timezone` 对象，表示固定的预设时区。也可以传入同步回调，用来表示动态时区。时区回调应当是一个接受日期时间字段所属 `ConfigGroup` 实例作为参数的同步方法，返回 `datetime.timezone` 对象。回调应保持只读，且不能再次读取当前字段，否则会递归调用。

### 加密字段

为字符串字段的 `Annotated` **末尾**添加 `encrypted()` 以声明该字段需要加密保存。

- **写入**：校验字段后，非空明文加密并保存，空字符串与密文直接保存。
- **读取**：自动解密并返回明文，解密失败时返回原始密文。
- **导出**：持久化时保持密文；API 输出和备份时会尝试解密。

框架允许不同的系统采用不同的加密方案：

- Windows：Windows DPAPI 按设备加密，密文可在同一台设备上由不同用户解密。
- 其他系统：尚未实现。

```python
<字段名>: Annotated[str, encrypted()] = <默认字符串>
```

### 引用字段

类似数据库中的外键，引用字段用于关联另一个已登记集合的一项数据成员。

```python
<字段名>: Annotated[
    UUID | str,
    ref(
        target=<目标集合的引用名称>,
        default=<无效引用的回退值>,
        allow_values=<允许的特殊值元组>,
        entry_types=<允许的目标类型元组>,
        display_field=<目标成员的展示字段路径>,
        on_delete=<引用失效策略>,
        on_delete_callback=<自定义处理方法>,
    ),
] = <字段默认值>
```

- `target=`：目标集合登记的引用名称，对应[配置集合的构造](#配置集合-collection)中的 `name=`。引用激活前，目标集合必须已登记。
- `default=`：无效引用的回退字符串，同时是字段默认值，默认为 `"-"`。
- `allow_values=`：可直接保存的特殊字符串元组，默认不额外放行。需要把某个值作为合法的「未选择」选项时，将它列入此处。
- `entry_types=`：允许引用的配置类元组，子类也匹配。省略或传空元组时，接受目标集合中的任意成员类型。
- `display_field=`：界面选项的展示字段路径，如 `"Info.Name"`。选项值仍是成员 UUID。
- `on_delete=`：目标被删除，或重载、替换成员类型后不再符合引用约束时采用的策略。目标变更和关联的配置修改在同一事务内执行。

  - `RefDeleteAction.SET_DEFAULT`：默认策略，将引用改回 `default=` 指定的值。
  - `RefDeleteAction.RESTRICT`：拒绝使引用失效的操作。
  - `RefDeleteAction.CASCADE`：删除持有引用的 Entry，要求它是集合成员。
  - `RefDeleteAction.CUSTOM`：调用 `on_delete_callback=` 指定的处理方法。

- `on_delete_callback=`：仅在 `on_delete=RefDeleteAction.CUSTOM` 时必填。可传接收持有引用的 Entry 和集合变更事件的同步或异步函数，也可传该 Entry 上的方法名。

### 虚拟字段

类似 `Python` 中的 `@property` 装饰器，虚拟字段允许将一个所属 Entry 上的同步方法包装为一个只读字段。

- **写入**：不可写，批量更新时自动跳过。
- **读取**：调用字段绑定的同步方法，将结果作为字段值返回。
- **导出**：API 输出时返回同步方法结果；持久化和备份时自动跳过。

```python
class <配置对象类型>(ConfigEntry):
    class <分组类型>(ConfigGroup):
        <字段名>: Virtual[<返回类型>] = None

    <分组名>: <分组类型>

    @virtual_field("<分组名>.<字段名>")
    def <取值方法>(self) -> <返回类型>:
        return <计算结果>
```

### 触发器字段

触发器字段将一个任务的控制方法抽象为一个配置字段对象，可以通过对该字段的读写来启停任务，确认任务运行状态。

在显式事务内给触发器字段赋值时，命令在根事务提交成功后才受理，发生回滚时命令会被丢弃。命令受理失败时，已经提交的普通字段仍然生效。

- **写入**：`True` 表示启动任务，已有任务时不重复启动，沿用当前任务；`False` 表示请求取消当前任务；`None` 表示不执行操作。
- **读取**：返回当前正在运行的任务 UUID，空闲时返回 `None`。请求取消后须等待任务结束才会转为 `None`。
- **导出**：API 输出时与读取结果一致；持久化和备份时自动跳过。

```python
class <配置对象类型>(ConfigEntry):
    class <分组类型>(ConfigGroup):
        <字段名>: Trigger = trigger(
            timeout=<执行时限>,
            description=<字段描述>,
        )

    <分组名>: <分组类型>

    @trigger_field("<分组名>.<字段名>", finish_on_cancel=<取消后是否继续执行>)
    async def <处理方法>(self):
        <执行操作>
```

- `timeout=`：执行时限，单位为秒，必须为有限正数，默认为 900 秒。
- `finish_on_cancel=`：默认为 `False`。设为 `True` 后，取消请求会让处理器继续运行到退出，适用于必须完整收尾的操作。

### 界面元数据

配置框架会根据字段声明生成界面提示，可通过 `ui()` 覆盖组件、格式和显隐等界面提示信息，通过 `select()` 补充下拉选项的来源和顺序。

- **界面提示**

  ```python
  <字段名>: Annotated[
      <字段类型>,
      ui(
          widget=<控件名>,
          format=<展示格式>,
          deps=<依赖字段列表>,
          secret=<是否提示敏感内容>,
          visibility=<显隐规则>,
      ),
  ] = <默认值>
  ```

  - `widget=`：指定界面控件，省略时由框架推导。
  - `format=`：指定界面展示格式，例如时间控件使用的 `"hm"`。
  - `deps=`：界面需要联动的字段名称列表。
  - `secret=`：为 `True` 时标记敏感内容，加密字段默认启用。
  - `visibility=`：可传 `UiVisibility` 值，需要动态变化时，传接收所属 Entry 并返回该枚举的同步只读函数。
    - `UiVisibility.SHOW`：正常显示，允许更新。
    - `HIDE`：不显示，默认更新时跳过。
    - `DISABLE`：显示，默认更新时跳过。
    - `MASKED`：显示占位符，允许写入新值。赋值占位符时保持原值。

  也可以通过装饰器绑定动态显隐方法。

  ```python
  @ui_visibility("<分组名>.<字段名>")
  def <显隐方法>(self) -> UiVisibility:
      return <显隐状态>
  ```

  Entry 的分组注解上可以直接使用 `ui(visibility=...)`，为整组设置显隐规则，注意整组不能使用 `MASKED`。

- **下拉选项提示**

  ```python
  <字段名>: Annotated[
      <字段类型>,
      select(endpoint=<选项接口>, ordered=<选项值是否有序>),
  ] = <默认值>
  ```

  - `endpoint=`：提供动态选项的接口地址，静态选项类型会自动推导无需单独设置。
  - `ordered=`：标记多选类字段的选中值是否有序，默认为 `False`。

- **组织表格提示**

  在 Entry 中声明多个同一 `ConfigTableColumn` 子类的分组，可以将这些分组组织为同一张表的多列。分组中的字段对应表格的行。字段提示和表格布局分别通过 `entry.ui.hints` 和 `entry.ui.tables` 读取。

### 历史字段兼容

字段移动或改名时，在目标字段上用 `legacy()` 标记原来的位置。冷态构造和激活读取配置时，如果输入中没有提供目标字段，框架会尝试从旧位置取值。目标字段显式传入 `None` 时也视为已提供。

```python
<新字段名>: Annotated[
    <字段类型>,
    legacy(group=<原分组名>, name=<原字段名>, convert=<值转换函数>),
] = <默认值>
```

- `group=`：旧配置中该字段所属的分组名。
- `name=`：旧配置中的字段名。
- `convert=`：可选的转换函数，接收旧值，返回交给新字段校验的值。

## 配置信号与事件

配置框架拥有专门的配置变更信号与事件，具备数据驱动业务的能力。与 `PostgreSQL` 的 `AFTER 触发器` 类似，在变更操作之后，回调仍在触发它的事务内执行。

### 事件类型

- **字段变更事件 `FieldChangeEvent`**

  - `kind`：变更类型
    - `init_set`：初始化时变更，初始化时字段值不变也会发送信号。
    - `set`：运行时修改，仅在字段值变化时发送。
  - `node`：字段所属的配置对象实例，与回调收到的 `sender` 相同。`config` 是它的兼容别名，`config_uid` 返回它的 UUID。
  - `group`：字段所属的分组名。
  - `field`：发生变更的字段名。
  - `value`：已校验与归一化的新值。加密字段传递明文值。
  - `old_value`：原字段值，初始化时一般是默认值。加密字段传递明文值。

- **集合变更事件 `CollectionChangeEvent`**

  - `kind`：变更类型
    - `init_add`：初始化时加入成员。
    - `add`：运行时添加成员。
    - `remove`：删除成员。
    - `set_order`：调整成员顺序。
    - `add_type`：登记成员类型。
    - `remove_type`：移除成员类型。
    - `reload_type`：重载成员类型。
    - `replace_type`：替换单个成员的类型。
  - `collection`：发生变更的配置集合实例，与回调收到的 `sender` 相同。
  - `uid`：`init_add`、`add`、`remove` 和 `replace_type` 涉及的成员 UUID。替换成员类型时保留原 UUID。
  - `entry`：与 `uid` 对应的成员实例。`init_add`、`add` 和 `replace_type` 提供新实例。`remove` 提供已软删除的实例。
  - `old_order`：`set_order` 操作前的旧成员顺序。
  - `order`：`set_order` 操作后的新成员顺序。
  - `entry_type`：操作涉及的成员类。`add_type` 提供新登记的类，`remove_type` 提供被移除的类，`reload_type` 提供重载后的类，`replace_type` 提供新实例的类。
  - `reloaded_uids`：`reload_type` 重建的成员 UUID 元组，按集合中的成员顺序排列。没有成员需要重建时为空元组。

### 订阅与退订

可以订阅配置节点的类信号，也可以直接订阅具体的配置节点实例信号：

- **类级订阅**：接收该类所有实例发出的事件，子类的订阅不继承父类。
- **实例级订阅**：只接收该实例发出的事件。

**回调函数**是支持附加属性的同步或异步函数，接收发送者 `sender` 和变更事件 `event`。公开订阅使用弱引用，回调被回收后订阅随之失效。

```python
<配置类或实例>.connect(
    receiver=<回调函数>,
    phase=<事件阶段>,
    kind=<事件类型>,
    group=<分组名>,
    field=<字段名>,
)
<配置类或实例>.disconnect(
    receiver=<原回调函数>,
    phase=<事件阶段>,
    kind=<事件类型>,
    group=<分组名>,
    field=<字段名>,
)

@<配置类或实例>.connect(
    phase=<事件阶段>,
    kind=<事件类型>,
    group=<分组名>,
    field=<字段名>,
)
def <回调函数>(sender, event):
    <处理逻辑>

@<配置类或实例>.disconnect(
    phase=<事件阶段>,
    kind=<事件类型>,
    group=<分组名>,
    field=<字段名>,
)
def <回调函数>(sender, event):
    <处理逻辑>
```

- `receiver=`：回调函数。
- `phase=`：按事件阶段筛选
  - `"runtime"` 只接收运行时事件，默认值。
  - `"init"` 只接收初始化事件。
  - `"both"` 接收初始化和运行时事件。
- `kind=`：按事件类型筛选，默认为 `None`，接收所选阶段的全部类型。订阅初始化事件时，`"set"` 可匹配 `init_set`，`"add"` 可匹配 `init_add`。
- `group=`：按分组名筛选，默认 `None`，接收任意分组的字段事件。订阅集合事件时省略此参数。
- `field=`：按字段名筛选，默认 `None`，接收任意字段的事件。订阅集合事件时省略此参数。

退订需要在原来的配置类或实例上使用同一回调对象和相同的过滤参数。

重新定义同名函数不会自动更新旧的订阅，而是会创建新的订阅。框架按函数对象识别订阅，所以只要旧函数仍被引用，新旧回调就可能同时收到事件。要替换旧回调，需要在函数名被重新绑定前，退订原订阅。不能在函数重新定义时直接使用 `@disconnect(...)`，退订装饰器作用于它装饰的函数，这会尝试退订新的函数。

### 信号的事务

- **回调在根事务提交前执行**。框架先应用配置变更，再派发事件并等待回调结束。回调可以读到本次变更后的配置。
- **回调修改配置后，要在返回前提交**。用 `asyncio.create_task()` 启动的任务有独立的事务身份。如果回调等它提交，双方可能互相等待。详见[排队、重入与后台任务](#排队、重入与后台任务)。
- **异常传出事务边界时，该边界内的修改一起回退**。这包括触发事件的修改和回调中提交的关联修改。
- **事务内的订阅变更立即影响同一事务后续派发的事件**。根事务提交后，更新才对其他事务生效。事务回退时，该边界内的订阅和退订操作一并丢弃。实现见[信号事件](#信号事件)。

## 自动持久化与导出

通过 `file=` 绑定 YAML 文件的配置实例能够自动持久化，当配置实例发生修改时，若该实例的根节点进行过持久化登记，就会自动持久化到对应文件。

修改生效后会等待一段时间再正式持久化，以实现短时间内多次修改合并处理，避免频繁修改时反复写入文件消耗存储寿命。

I/O 错误会自动重试直到达到次数上限，其他错误不自动重试。持久化错误不会自动上抛，快照会保存在 `config_manager.persistence_errors`。

- **立即落盘**

  需要确认已提交的配置写入 YAML 文件时，在事务外调用 `flush()`。它跳过自动保存的等待时间，尝试保存所有可保存的根。

  ```python
  await config_manager.flush()
  ```

- **注销持久化登记**

  框架允许注销持久化登记，建议在注销前使用 `flush()` 保留最后一次修改。

  ```python
  config_manager.unregister_root(node=<已登记的根节点>)
  ```

- **手动导出**

  你可以快速导出所有登记的配置，文件将按原有相对目录结构写入目标目录。导出使用 `audience="export"`，加密字段会以明文写入配置文件。

  ```python
  await config_manager.export(target=<导出目录>)
  ```

## 在 FastAPI 中使用配置框架

配置框架采用 `Pydantic` 定义基础数据模型，与 `FastAPI` 具备良好的兼容性。

### 声明请求与响应模型

以已有的 Entry 类为字段来源，生成接口请求或响应中的 `data` 模型。

- **普通请求模型 | `ConfigInput`**

  用于接收字段更新请求，忽略未知的字段或分组。

  ```python
  from pydantic import BaseModel

  from app.config.wire import ConfigInput


  class <更新请求模型>(BaseModel):
      data: ConfigInput[<配置类>]
  ```

- **严格请求模型 | `StrictConfigInput`**

  用于接收字段更新请求，拒绝未知字段或分组。

  ```python
  from pydantic import BaseModel

  from app.config.wire import StrictConfigInput


  class <更新请求模型>(BaseModel):
      data: StrictConfigInput[<配置类>]
  ```

- **响应模型 | `ConfigPayload`**

  用于返回字段查询请求，不包含子节点。

  ```python
  from app.config.wire import ConfigPayload
  from app.models.schema import ConfigSnapshotOut, OutBase

  <配置响应数据模型> = ConfigPayload[<配置类>]


  class <查询响应模型>(ConfigSnapshotOut):
      data: <配置响应数据模型>
  ```

- **包含嵌套集合的响应模型 | `config_contract()`**

  用于返回字段查询请求，包含子节点。

  ```python
  from app.config.wire import config_contract
  from app.models.schema import ConfigSnapshotOut

  <配置响应数据模型> = config_contract(<配置类>, include_collections=True)


  class <查询响应模型>(ConfigSnapshotOut):
      data: <配置响应数据模型>
  ```

### 查询配置

让业务方法读取当前配置并返回可供 API 使用的字典，路由用这个字典构造响应的 `data`。如果页面需要在配置变更后自动刷新，把本次查询依赖的 Entry 或 Collection 的 UUID 填入 `uiSources`。详见[配置界面同步机制](#配置界面同步机制)。

```python
@router.post("/<查询路径>", response_model=<查询响应模型>)
async def <查询路由函数>() -> <查询响应模型>:
    data = await <配置业务对象>.<查询配置方法>()
    return <查询响应模型>(
        data=<配置响应数据模型>(**data),
        uiSources=[<配置节点 UUID>],
    )
```

查询结果包含集合时，集合内部的 `order` 按顺序列出成员的 UUID 和类型，`data` 则用 UUID 索引各成员的配置。

### 修改配置

用 `model_dump(exclude_unset=True)` 提取本次要更新的字段，保证仅更新明确提供的值。

```python
@router.post("/<更新路径>", response_model=OutBase)
async def <更新路由函数>(body: <更新请求模型>) -> OutBase:
    changes = body.data.model_dump(exclude_unset=True)
    await <配置业务对象>.<更新配置方法>(changes)
    return OutBase()
```

## 配置界面同步机制

考虑到配置字段与前端页面的强关联，配置框架具备主动向前端推送页面变化通知的功能。在配置查询中，会一并给出此次查询对应的页面 UUID，下次该配置发生变化时，会告知前端涉及的页面 UUID。

后端通过 WebSocket 通知前端页面变化。若连接断开，前端每 5 秒重新查询当前可见页面正在使用的配置，重连后停止轮询。

查询响应的 `uiSources` 用于传递本次查询对应的界面 UUID 列表。一般查询单个配置对象时填 Entry 的 UUID，查询集合列表时填 Collection 的 UUID，如果结果依赖多个节点，就把它们的 UUID 一起填入。

## 进阶机制

### 初始化顺序

Entry 进入 `INITIALIZING` 后，先初始化普通字段，再递归激活嵌套集合，最后为引用字段建立订阅。初始化结束后，节点切换为 `ACTIVE`；激活结果在根事务提交后才对其他执行者生效。

#### 逐字段初始化与快速初始化

默认初始化为逐字段路径：每个字段或成员在自己的局部边界内校验、提交和派发 Init 信号。单个字段失败时，之前成功的字段可以保留，最后通过 `ConfigAggregateError` 汇总错误。根激活自己开启普通事务时，成功部分提交到 live，节点成为 `ACTIVE`，随后抛出聚合异常。

启动方可以显式使用 `activate(fast_init=True)` 尝试快速初始化。它只把**当前 Entry 的整个字段循环**放入一个局部事务，减少重复建立和合并工作区的开销：

1. 所有字段仍按原顺序逐个赋值、校验并发送即时 Init 信号。
2. 任一字段校验、直接 Init 信号链或受管关联修改失败，快速事务完整回退。
3. 回退完成后，从首字段重新执行默认逐字段路径；快速尝试中已成功的字段不会复用。
4. 接收者和第三方服务可能在快速尝试与逐字段重试中各执行一次，配置框架不负责识别、去重或回滚外部副作用。

嵌套 Entry 各自选择和回退自己的字段边界；集合成员的新增、重载和替换仍保留成员级边界。若激活位于已有父事务内，成功内容只并入父事务工作区；父事务随后失败时整体回退，节点恢复冷态及待激活载荷。

`fast_init` 不是全局初始化模式，也不改变普通运行时 `commit()` 的边界。没有传入该参数时，激活继续使用逐字段路径。

#### 激活标记与热态限制

节点自己的 `NodeState` 与整棵递归激活操作的标记是两个概念。标记在构造时从直接父节点承接，`build()` 和递归 `activate()` 不需要额外传递；没有进行中标记的独立入口才创建新标记。

初始化尚未完成时，锁、触发器、公开实例订阅、删除守卫、file/name 登记等热态生命周期操作只限制本次激活树内的节点。树外已稳定节点仍按原热态规则工作；冷态字段赋值和激活内部的引用订阅继续执行。节点在 `ACTIVE` 后仍要等所属根事务完成发布，才可以向其他执行者提供稳定热态服务。

#### 引用目标与启动依赖

启动时先登记引用目标并激活其成员，再激活依赖它们的配置。根集合通过 `name=` 登记；需要作为引用目标的嵌套集合通过 `config_manager.register_collection()` 登记。主程序由 `app/core/config/manager.py` 安排顺序：先加载全局配置和插件登记，等插件提供的配置类型就绪，再激活游戏配置及引用游戏的业务配置。

### 信号事件

配置节点的信号基于 Blinker 实现。派发事件时，框架先同步调用接收者包装器，再依次等待返回的 awaitable；接收者的具体顺序由 Blinker 决定，框架只保证等待异步处理结果。需要严格先后关系的步骤，应在同一接收者内按顺序调用。

事务首次修改某个节点类的订阅时，框架复制该类当前的有效信号，为当前边界建立独立的信号工作区。派发事件时，从当前边界向外查找最近的工作区，找不到时使用 live 信号。子边界提交时将工作区并入父事务，根事务提交时将工作区发布为 live 信号。回退时丢弃相应边界的工作区。

Blinker 没有公开的完整订阅枚举接口。框架建立信号工作区时，会从接收者、发送者和关联索引重建独立的订阅索引，同时保留弱引用和清理行为。订阅索引属于框架与 Blinker 的内部实现边界，业务代码应使用公开的 `connect()` 和 `disconnect()` 接口。

### 事务机制

#### 默认边界：一次操作及其信号链

一个热态字段赋值只向当前执行者的队列暂存操作，尚未改变生效值。`commit()` 取得执行资格后快照本作用域的队列，依次处理各个操作：

1. 先取得全局执行入口，再取得节点提交锁。
2. 没有有效事务时，为当前操作创建事务上下文；首次修改节点时建立工作区。
3. 在工作区应用字段或集合操作，执行校验，派发相应信号。
4. 普通信号回调及其直接调用、直接 `await` 的方法继续处于同一个 Task，复用该事务；关联节点的修改也进入各自工作区。
5. 回调正常结束且没有遗漏的暂存操作时，合并所有关联工作区；异常离开事务边界则丢弃它们。
6. 关闭事务上下文并清理作用域，再登记需要保存的持久化根。

默认情况下，**一个 `commit()` 批次中的每项操作分别构成事务**。各操作独立成功或回退；框架继续处理后续操作，最终通过 `ConfigAggregateError` 汇总失败。全局入口和节点锁覆盖整个批次，其他执行者的提交在批次结束后执行；各操作的回退边界保持独立。`fast_init` 只在激活调用方为当前 Entry 的字段循环建立额外局部边界，不改变这里的运行时提交规则。

例如，同时暂存 A、B 后提交，A 成功、B 失败时，A 可以已经生效。显式事务将多个操作组织为共同成功或共同回退的边界。

#### 显式事务与嵌套

同一 Task 中的嵌套 `transaction()` 复用已有事务；需要独立回退边界时使用 `transaction(new_boundary=True)`。事务内的 `commit()` 返回时，暂存操作已在工作区执行；这些修改要到根事务提交才会对其他执行者生效。

初始化和普通运行时修改使用同一种事务上下文。上下文只记录边界、父链、真实执行者和参与节点；配置工作区仍由节点实例持有，信号工作区由节点类持有。子边界成功时只并入父工作区，只有根事务发布 live、修改代次并登记保存请求。

::: info 异常传播与回退边界
异常传出根事务时，整笔事务回退。若异常只传出 `new_boundary=True` 建立的子边界，并在外层被捕获，则只回退该子边界。若在同一边界内捕获异常且事务最终正常提交，工作区中的修改仍然生效。
:::

事务保护范围为框架管理的配置状态及相应订阅工作区。已经发送的通知、设备操作、数据库写入和外部文件修改属于外部副作用，其生命周期与配置回退分离。

#### 如何判定工作区读取

事务上下文同时校验生命周期和真实执行者：有 Task 时核对 Task 对象，否则核对 Thread 对象。仅继承了 `ContextVar` 不代表取得事务参与资格。

普通属性读取按以下优先级解析：有效 Init 工作区 → 有效普通工作区 → live。当前节点尚未建立工作区时，直接读取 live。集合成员身份及业务引用指向框架管理的 live 节点；工作区是事务期间的临时状态，随提交或回退结束。

`node.effective` 返回当前上下文的有效节点；`config.Info.Name` 等属性读取通过该有效状态解析字段。**对原始 live 对象调用 `model_dump()` 导出 live 存储值**，用于持久化和 FastAPI 输出。`node.effective.model_dump()` 则可导出当前事务的工作区内容；在最外层事务提交前，该内容属于待提交状态。

日期时间的存储字段也遵守这一序列化规则，直接导出该对象内存中的 UTC 值，不走属性读取的时区转换。API 的虚拟字段、UI 提示等计算结果仍按各自声明求值，不等同于普通存储字段快照。

#### 排队、重入与后台任务

全局执行入口采用单事务模型，串行协调提交、显式事务、自动保存及 `flush()`。锁顺序为全局入口 → 节点锁，跨节点关联修改按同一顺序重入，消除了分别持有节点锁后互相申请执行资格的反向等待。

同一实际执行者可以重入，其他执行者按等待队列取得资格。异步入口挂起等待，普通工作线程的同步入口阻塞等待。事件循环线程中的 `transaction_sync()` 若遇到其他所有者占用，则抛出异常，事件循环保持可运行状态。

独立 `asyncio.create_task()` 使用自己的事务身份，读取 live，修改时申请执行资格并排队。若父事务持有入口并等待该子任务提交，子任务等待入口释放，双方会形成循环等待。同 Task 的直接调用或直接 `await` 将关联修改纳入父事务；事务退出后启动的独立任务则使用独立的提交边界。

没有工作区读取资格的执行者读取 live，另一个事务的暂存值保持隔离。普通属性读取反映每次访问时的有效状态；管理器导出入口通过全局执行入口提供跨根的稳定导出范围。

#### 暂存归属、取消与收尾

暂存队列按“真实执行者 + 普通或 Init 作用域”隔离，只存在 live 节点中，不复制到工作区，不持久化，也不转移给其他 Task。

- 赋值和提交使用同一作用域：事务外暂存由事务外提交读取，显式事务内暂存由该事务内提交读取。
- 当前作用域为空、但同一执行者还有其他作用域的暂存时，`commit()` 抛出异常，其他作用域的队列保持原状。
- 信号回调通过 `commit()` 应用其产生的修改；回调返回或事务退出时存在本链未提交暂存，会报错并回退。
- 回退清理当前作用域残留，不清除其他任务队列；Task 结束后清理其未提交暂存。
- 等待执行入口被取消时，撤销排队或归还已经交接的资格。执行中的取消会回退当前操作，将已回退及未执行的操作归还相应队列；不会自动重试业务修改。事务作用域关闭后，其残留仍按作用域规则清理。
