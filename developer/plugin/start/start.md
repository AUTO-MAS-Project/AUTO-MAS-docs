# 开发起步

本文将指引你从零开始，搭建MAS开发环境并启动开发环境的MAS。

## 选择适合你的开发软件

MAS后端及插件系统采用Python3.12，桌面采用Electron，详细请看 [技术栈](../../getting-start)。

插件系统将尽其所能的避免让插件开发者为了配置项处理前端，因此你理论上只需要会使用Python即可开发MAS插件。

本文中我们将使用VScode作为教程使用的开发工具，当然如果你也能够使用JetBrains的PyCharm甚至是Spyder（你真的要这么做吗？）。

## 安装Python 

MAS理所当然需要[Python](https://www.python.org/downloads/)，目前我们规定使用3.12的python，尽管你似乎可以使用±1版本的python来成功运行后端，但是3.13的版本可能存在潜在的兼容问题，而过早的版本可能导致部分库无法运行，因此如果你打算长期开发MAS插件，我们不建议你使用除了3.12以外的其他版本。

![img](/developer/img/python_download_page.png)

点击Download下载，随后根据安装包流程完成安装。

> 如果你的电脑上没有python，可以勾选 Add to Path将python解释器添加到你的系统变量中。

安装完成后，打开命令行工具（CMD/powershell等），输入

```bash
python --version	#显示类似 Python 3.12.x 即为安装成功
```

## 安装Node.js

MAS使用了Electron37.0.0，需要Node 22.16.0或更高的版本。

前往[Node.js官网](https://nodejs.org/zh-cn/download)选择适合的版本（推荐使用LTS版本），下载并安装

![img](/developer/img/nodejs_download_page.png)

安装完成后，打开命令行工具（CMD/powershell等)，输入

```bash
node -v	#显示类似 v22.16.0 即为安装成功
```

## 安装Git（推荐)

MAS需要使用Git来拉取项目，而我们也推荐你使用Git来管理自己插件。

前往[Git官网](https://git-scm.com/install/)，下载适合你系统的Git，直接下载最新版本即可。

如果你下载遇到困难，也可以使用[CNPM镜像下载](https://registry.npmmirror.com/binary.html?path=git-for-windows/)。

安装Git一路下一步即可，安装完成后，打开命令行工具（CMD/powershell等），输入

```bash
git --version	#显示类似 git version 2.53.0 即为安装成功
```

最后你还需要设置你的姓名和邮箱。它们将会默认作为你创建的插件的作者，也会出现在你的提交记录中：

```bash
git config --global user.name "Your Name"
git config --global user.email "you@example.com"
```

## 注册 代码托管平台账号

通常来说我还会建议你注册一个 GitHub 账号。[GitHub](https://github.com/) 是一个代码托管平台，我们可以在上面创建仓库来存放我们的代码。

当然你也可以使用[Gitee](https://gitee.com/)，[CNB](https://cnb.cool/)等国内替代，由于篇幅有限，请在互联网搜索相关的教程，自行完成注册。如果发现无法注册，也不用担心，你仍然可以在本地进行开发。

我们**鼓励并支持**所有插件开发者通过开源公开自己的插件代码，这样更容易得到社区的认可，也更容易审计和提交贡献。

## 克隆MAS仓库

自行创造一个合适的开发目录，如`D:/dev`，将[MAS Github仓库](https://github.com/AUTO-MAS-Project/AUTO-MAS#)克隆到本地。

```bash
git clone https://github.com/AUTO-MAS-Project/AUTO-MAS.git
```

或使用SSH

```bash
git clone git@github.com:AUTO-MAS-Project/AUTO-MAS.git
```

等待克隆进程完成。

::: warning 注意

你可能留意到我们在CNB，甚至GitCode等平台也有AUTO-MAS的仓库。

除了CNB和Github，其他平台的代码均不由AUTO-MAS-Team上传，与开发者无关。

CNB我们正在尝试支持下载，但Git的提交可能并不是最新的，因此仍然建议你使用Github克隆代码

:::

项目结构大致为这样[项目结构](../../getting-start#项目结构)

## 后端python环境

### 创建虚拟环境

在MAS的仓库下，推荐创建一个python的虚拟环境

```bash
python -m venv .venv
```

### 激活虚拟环境

**Windows (CMD):**

```bash
.venv\Scripts\activate.bat
```

**Windows (PowerShell):**

```bash
.venv\Scripts\Activate.ps1
```

::: tip powershell提示

如果遇到权限错误，先运行：`Set-ExecutionPolicy RemoteSigned -Scope CurrentUser`

:::

当命令行前出现：

```
(.venv) C:\xxxx\AUTO-MAS>
```

即为激活成功

### 安装依赖

```
pip install -r requirements.txt
```

若安装不畅，可以使用镜像源

```bash
# 清华大学
https://pypi.tuna.tsinghua.edu.cn/simple
# 阿里云
https://mirrors.aliyun.com/pypi/simple/
# 腾讯云
https://mirrors.cloud.tencent.com/pypi/simple
# 豆瓣
https://pypi.douban.com/simple/
# 中科大
https://pypi.mirrors.ustc.edu.cn/simple/
# 华为云
https://mirrors.huaweicloud.com/repository/pypi/simple
```

如

```bash
pip install -r requirements.txt -i https://pypi.tuna.tsinghua.edu.cn/simple
```

## 前端Node环境

### 启用Yarn（可选）

MAS推荐使用Yarn v4进行包管理。

```bash
cd frontend
corepack enable
```

验证安装

```bash
yarn --version
```

如果输出版本号为4.x.x（目前使用的是v4.9.1），即为安装成功

### 安装前端依赖

```bash
yarn install
```

在这一步中可能遇到下载问题

#### yarn下载过慢

你可以使用镜像源来解决这个问题，使用

```
yarn config set registry https://registry.npmmirror.com
```

先设置一次镜像源，随后在运行

```
yarn install
```

#### Electron下不下来总超时

使用 Electron 国内镜像。Electron 官方支持自定义下载源，你只需要设置环境变量

cmd：

```bash
export ELECTRON_MIRROR=https://npmmirror.com/mirrors/electron/
```

或使用powershell

```powershell
set ELECTRON_MIRROR=https://npmmirror.com/mirrors/electron/
```

然后

```bash
yarn install
```

## 尝试启动MAS

### 后端

先激活虚拟环境

```
.venv\Scripts\activate.bat
```

直接运行main.py

```bash
(.venv) C:\xxxx\AUTO-MAS>python main.py
```

出现

```
2026-04-07 18:46:23.001 | INFO     | 主程序 | Application startup complete.
2026-04-07 18:46:23.003 | INFO     | 主程序 | Uvicorn running on http://0.0.0.0:36163 (Press CTRL+C to quit)
```

即为启动成功

### 前端

进入前端目录

```
cd frontend
```

运行

```
yarn dev
```

出现可视窗口即为启动成功



至此你已经成功启动

## 配置VSCode调试与运行

为了方便你后续启动后端和前端，你可以在此处配置launch.json。

![img](/developer/img/vscode_run_and_debug.png)

```json
{
  "version": "2.0.0",
  "configurations": [
    {
      "name": "后端 Dev (.venv)",
      "type": "debugpy",
      "request": "launch",
      "program": "${workspaceFolder}/main.py",
      "cwd": "${workspaceFolder}",
      "env": {
        "AUTO_MAS_DEV": "1"
      },
      "python": "${workspaceFolder}\\.venv\\Scripts\\python.exe",
      "console": "integratedTerminal"
    },
    {
      "name": "前端 dev",
      "type": "node-terminal",
      "request": "launch",
      "command": "npm run dev",
      "cwd": "${workspaceFolder}/frontend"
    }
  ]
}
```

这样即可一（两）键启停MAS。

## 环境变量

你可以使用环境变量获取一些MAS在开发环境的特性。

环境变量 `AUTO_MAS_DEV=1`是后端的开发环境的识别手段，后端检测到此变量时，会自动以更适合开发的名字提供。

前端则使用以下逻辑来进行判断，无需做额外的处理。

```javascript
  process.env.NODE_ENV === 'development' ||
  (import.meta as any).env?.DEV === true ||
  window.location.hostname === 'localhost'
```



至此，你已完成了MAS的基础准备，现在你可以开始起步开发插件了！
