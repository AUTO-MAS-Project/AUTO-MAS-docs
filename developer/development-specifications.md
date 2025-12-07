# 开发规范

## Git 分支

### 1. 主分支

- 分支名：`main`
- 作用：主分支，用于存储稳定版代码，保证所有功能稳定，能够直接跑通。


### 2. 主开发分支

- 分支名：`dev`
- 作用：主开发分支，用于存储包含当前最新功能的代码，所有完成开发且通过测试的代码 *（包括各种 pr）*，合并到该分支，保证能够直接跑通。


### 3. 发行版分支

- 分支名：`release/{version}`
- 作用：由版本发布工作流自动创建，用于存储各个发行版的最新后端代码，各个发行版软件持续会从各自对应的发行版分支拉取最新后端代码，以实现部分功能更新与问题修复的热更。


### 4. 开发分支

- 分支名：`随意~`
- 作用：开发分支，项目核心开发者用于存储开发中的功能，不一定能跑通，当功能开发完成后，将会合并到 `dev` 分支。


## Git 提交信息

Git 提交采用Conventional Commits 标准，即：

```
<type>(<scope>): <subject>
```

### 1. type

| 类型         | 说明                                     |
| ------------ | ---------------------------------------- |
| **feat**     | 新功能（feature）                        |
| **fix**      | 修复 bug                                 |
| **docs**     | 文档修改（例如 README）                  |
| **style**    | 代码格式调整（不影响逻辑，如空格、缩进） |
| **refactor** | 代码重构（既不是新功能，也不是修复）     |
| **perf**     | 性能优化                                 |
| **test**     | 增加或修改测试                           |
| **chore**    | 构建流程或依赖管理上的调整               |
| **build**    | 影响构建系统或外部依赖的变更             |
| **ci**       | CI/CD 配置修改                           |

### 2. scope

1. **只修改了某个文件**

    填入去掉后缀的修正文件名称，保留大小写。

    - 例如：修改 `logger.py`，使用 `logger`

        ```fix(logger): 修改logger日志等级 ```

2. **修改了多个文件**

    填入修正文件的 **父文件夹名**，涉及多个文件夹时，填入所有发生修改文件夹的 **父文件夹名**。

    - 例如：修改 `app/api/a.py`、`app/api/b.py`，使用 `api`

        ```fix(api): 修改api模块 ```

### 3. subject

**所做修改的一句话总结**，可隔一行书写详细修改内容

```
feat(core): 改进数据库连接模块

- 优化连接池初始化逻辑
- 减少启动时间约 30%
- 修复可能导致连接泄漏的问题
```


## 版本记录

本项目依靠 `res/version.json` 文件记录版本信息，文件格式为：

```json
{
    "main_version": "5.0.0.0",      # 当前版本号
    "version_info": {               # 新版本更新记录
        "5.0.0.0": {                # 版本条目
            "新增功能": [           # 修改类型
                "全新架构升级",      # 修改条目
                ... 
            ],
            "修复BUG": [
                ...
            ],
            "程序优化": [
                ...
            ]
        },
        "5.0.0.1": {
            ...
        }
        ...
    }
}
```

- 版本更新信息与公告都根据该文件生成，为了保证您所做的修改被正确登记，当您完成某项任务时，需要自行修改该文件并提交。
- 若 `当前版本号` 已经完成发布，您不能直接在对应版本条目下新增 `修改条目`，也无需修改 `main_version` 字段，而应该在 `version_info` 中添加下个版本的版本条目，并在新版本条目下添加您的 `修改条目`。


## 后端注释

::: tip 总览
后端注释应该采用 **Google Python Style Guide – Docstring Conventions**，简称 **Google Docstring** 。
:::

### 方法注释
```
def add_numbers(a: int, b: int) -> int:
    """
    ## (一句话简介，当然长点也无所谓）
    相加两个数字并返回他们的和.
    
    ## 形参解释
    Args:
        a (int): 第一个数字.
        b (int): 第二个数字.
    
    ## 返回值解释
    Returns:
        int: 数字之和.
    
    ## 报错类型解释
    Raises:
        ValueError: 输入的数字不是int.
    """
    
    ## 逻辑代码
    if not all(isinstance(x, int) for x in (a, b)):
        raise ValueError("形参必须是integers.")
    return a + b

```

### 配置类注释

`ConfigBase` 的子类为 **配置类**。配置类需要为 **每一个ConfigItem** 添加解释，并根据 `Group` 做好分类。

```
class GlobalConfig(ConfigBase):
    """全局配置"""
	
	## Function ------------------------------------------------------------
	## 历史记录保留时间
    Function_HistoryRetentionTime = ConfigItem(
        "Function",
        "HistoryRetentionTime",
        0,
        OptionsValidator([7, 15, 30, 60, 90, 180, 365, 0]),
    )
	## 是否允许睡眠
    Function_IfAllowSleep = ConfigItem(
        "Function", "IfAllowSleep", False, BoolValidator()
    )
    ## 是否启用静默模式
    Function_IfSilence = ConfigItem("Function", "IfSilence", False, BoolValidator())
    ## 是否启用bilibili自动
    Function_IfAgreeBilibili = ConfigItem(
        "Function", "IfAgreeBilibili", False, BoolValidator()
    )
    
    ## notify --------------------------------------------------------------
    ## 是否发送运行结果
    Notify_IfSendStatistic = ConfigItem(
        "Notify", "IfSendStatistic", False, BoolValidator()
    )
    ......
```



## 后端函数调用

::: tip 总览
能一眼理解的用位置参数；否则用关键字参数。布尔和多参数强制使用关键字。
:::

为了提高代码可读性、减少误用、提升协作效率，项目中调用函数时应该遵循以下规则：

####  1. 简单且含义明确的函数 → 使用位置参数

当参数少（通常 ≤2 个）且含义清晰时，可直接使用位置参数。

**示例：**

```
add(1, 2)
math.sqrt(4)
```

------

####  2. 含义不够直观、参数较多、或容易混淆的函数 → 使用关键字参数

关键字参数能提升可读性，不依赖位置顺序，更安全。

**示例：**

```
UserItem(user_id=str(uid), name=config.get("Info", "Name"), status="等待")
```

------

####  3. 所有布尔参数必须使用关键字参数

避免出现可读性极差的调用。

不推荐：

```
send_notification(True, False)
```

推荐：

```
send_notification(enable=True, urgent=False)
```

------

####  4. 当参数数量超过 2 个时，优先使用关键字参数

提升可维护性，减少顺序错误风险。

**示例：**

```
connect(host="localhost", port=3306, timeout=5)
```



