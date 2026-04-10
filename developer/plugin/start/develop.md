# 插件开发

::: warning

插件系统还在积极开发中，可能会有其他变化

:::

本章节将会带领你按照标准流程开始开发插件。

::: tip 提示

本节中介绍的命令行非专门提醒都需要在应用目录和python虚拟环境下运行

即克隆的AUTO-MAS/ 目录与自行创建的.venv

:::

## 创建新插件

在应用目录下运行以下指令

```bat
python script/plugin_tool.py [--name <name>] [--description <description>] [--init-git <bool>]
```

如果不提供参数，也可以在交互式命令行中填写，无需一次写完。

- **--name:** 插件的包名
- **--description:** 插件的介绍，后续可以更改
- **-m, --monorepo:** 创建 monorepo 的插件
- **-G, --no-git:** 跳过 git 初始化

我们假设你创建了一个叫example的插件，那么你将看到如下目录结构。

```diff
AUTO-MAS
├── plugins
│   └── example
│       ├── src
│       │   └── plugin.py
│       └── pyproject.toml
│       └── README.md
└──config
     └── PluginConfig.json
```

打开plugin.py