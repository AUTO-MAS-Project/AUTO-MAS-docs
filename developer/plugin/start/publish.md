# 发布插件

本文将从头开始教你如何将编写的插件上传到PyPi，供任何用户下载安装使用。

## 构建源代码

如果你需要发布到插件市场，你需要构建你的源代码。为此你需要先安装twine

你可以选择在虚拟环境中安装，也可以在自己的系统环境安装，在你使用IDE工具时，一般会默认使用虚拟环境，我们也推荐你在虚拟环境中安装twine。

```bash
pip install build twine
```

进入你的插件目录，如

```diff
AUTO-MAS/plugins/ces
```

运行

```bash
python -m build
```

它会生成：

```diff
dist/
 ├── ces-0.1.0.tar.gz
 └── ces-0.1.0-py3-none-any.whl
```

## 发布插件

1. **注册 PyPI 和 TestPyPI 账号**
   - **正式环境**：访问 [pypi.org](https://pypi.org/) 注册账号。
   - **测试环境**：访问 [test.pypi.org](https://test.pypi.org/) 注册账号。这是一个独立的测试服务器，用来验证你的发布流程，不会影响正式的 PyPI。
2. 
