# 开发规范

## Git提交信息

Git提交采用Conventional Commits 标准，即

```
<type>(<scope>): <subject>
```

### type

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

### scope

#### 情况1：只修改了某个文件

**填入去掉后缀的修正文件名称**，**大小写需要保留**

如logger，就写logger

fix(logger): 修改logger日志等级 

#### 情况2：新增/修改了某个具体的功能

比如，后端的api（新增api）、task（新增专项适配）。前端的scheduler（调度中心界面），queue（调度队列，这里有很多子页面）

那就填 **父文件夹名**，比如api、task等

### subject

**一句话省流**，并可选详细内容

```
feat(core): 改进数据库连接模块

- 优化连接池初始化逻辑
- 减少启动时间约 30%
- 修复可能导致连接泄漏的问题
```

比如

```
feat(login): 添加用户登录功能
fix(build): 修复打包时路径错误
chore(deps): 更新 electron 到 33.0.0
docs(readme): 补充安装说明
refactor(core): 优化数据加载逻辑
style(ui): 调整按钮样式和间距
test(api): 新增接口单元测试
```

## 后端注释

后端注释应该采用 **Google Python Style Guide – Docstring Conventions**，简称 **Google Docstring** 。

比如
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

另外，如果一个类不是一个**方法**，而是一个**配置**，应该给**每一个Configitem**添加解释，并根据Item做好分类

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

