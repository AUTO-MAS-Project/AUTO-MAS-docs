# 登记专项适配

适配插件在插件类上声明类型键和 `decl`，交给宿主登记。类型键标识适配类型，`decl` 集中声明宿主管理该类型时需要的配置类和业务实现。

## 游戏适配

游戏适配插件为宿主提供游戏安装发现、游戏与设备配置以及设备控制能力。宿主据此登记游戏安装，并管理游戏客户端或模拟器中的设备。

接入步骤见[游戏适配插件 · 声明游戏类型](../game-adapters.md#声明游戏类型)。

```python
decl = GameTypeDecl(
    game_class=<游戏配置类>,
    device_class=<设备配置类>,
    control_class=<设备控制器类>,
    search=<异步安装发现函数>,
    resolve_installation=<旧安装路径解析函数或 None>,
)
```

## 托管适配

托管适配插件为宿主提供托管、账号及可选的计划表配置，并提供任务展开器。宿主据此创建该类型的托管配置，并在调度时按运行模式展开任务。

接入步骤见[独立专项适配 · 声明托管类型](../script-adapters/standalone.md#声明托管类型)。

```python
decl = ScriptTypeDecl(
    script_type=<托管配置类>,
    user_type=<账号配置类>,
    expander_class=<任务展开器类>,
    plan_type=<计划表配置类或 None>,
    supported_modes=<支持的运行模式元组>,
    expect_builder=<模拟器期望配置生成函数或 None>,
    prefetch_check=<异步预下载检查函数或 None>,
    activity_game=<用于活动展示和提醒的游戏标识或 None>,
    game_kinds=<可绑定的游戏目标类型元组>,
    supports_mod_avd=<是否支持 ModAVD>,
)
```
