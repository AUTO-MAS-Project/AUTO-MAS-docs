# 常见问题

这里没有的问题，去 <Pill name="AUTO-MAS GitHub Issues" :image="{ light: '/icons/github.svg', dark: '/icons/github-dark.svg', }" link="https://github.com/AUTO-MAS-Project/AUTO-MAS/issues"/> 翻一下。

如果问题出在脚本本身（比如 MAA 识别不出关卡），请查阅对应脚本的文档或联系脚本作者——AUTO-MAS 只负责调度脚本。

## 疑问解答

### **AUTO-MAS 利好代肝吗？**

- 代肝用 AUTO-MAS 就是利好代肝，用户用 AUTO-MAS 就是利好用户。
- 而且 AUTO-MAS 的传播范围越广，就越利好用户，所以还不快帮 AUTO-MAS 宣传一波！

### 我的账号密码安全吗？

账号密码、Token 等敏感信息由 Windows 自带的加密功能（DPAPI）加密后保存在本地，AUTO-MAS 不会把它们上传到任何服务器。

这套加密和你的 Windows 登录账号绑在一起，所以：

- 只有你本人登录这台电脑的这个 Windows 账号时，程序才解得开。
- 别人就算把配置文件整个拷走，换台电脑也解不开。

::: warning 警告
  在以下情况下，系统可能无法解密原有数据：

  1.  **更换或重装系统**
      如果您重新安装 Windows 或使用新的电脑账户，原账户的加密密钥将丢失，程序将无法读取旧数据。
  2.  **删除或重置用户账户密码**
      DPAPI 的加密密钥与您的 Windows 登录凭据绑定。
      如果您使用非正常方式重置密码（如离线修改、系统修复工具修改等），Windows 无法重新解密旧的加密文件。
  3.  **复制数据到其他电脑或账户**
      DPAPI 加密的数据仅在原账户和计算机上有效。复制配置文件到其他环境时，数据会因密钥不匹配而无法解密。
:::

## 故障排查

### 软件一直在报 Network Error

这说明后端没起来。先在报错页面上，或者打开 `debug/app.log`，找到具体错误，再对着下面处理：

- **`[Errno 10048] error while attempting to bind on address ('0.0.0.0', 36163)`**

  端口被别的程序占了。AUTO-MAS 后端用的是 `36163` 端口，查一下是谁占着，把它关掉。

- **`ModuleNotFoundError: No module named 'xxx'`**

  依赖没装全。删掉安装目录下的 `environment/.requirements_hash` 后重启软件，重新安装依赖；还不行就删除整个 `environment` 文件夹再重启。

- **`ImportError: DLL load failed while importing onnxruntime_pybind11_state`**

  系统缺 **Microsoft Visual C++ 运行库**。装一下就好：[直接下载 x64 版](https://aka.ms/vc14/vc_redist.x64.exe)（或从 [微软官方页面](https://learn.microsoft.com/zh-cn/cpp/windows/latest-supported-vc-redist?view=msvc-170#latest-supported-redistributable-version) 挑版本）。

::: tip 报错页面和日志里都看不到错误怎么办

手动跑一次后端，让错误直接显示在终端里。以管理员身份打开终端（PowerShell 或 CMD），执行：

```bash
cd {AUTO-MAS 根目录}
.\environment\python\python.exe main.py
```

`{AUTO-MAS 根目录}` 换成你的实际安装路径。终端里打出来的报错，就是排查线索。

:::

### 模拟器启动失败

原因基本都是权限不一致。AUTO-MAS 启动的脚本和模拟器一律带管理员权限，但如果此时已经有模拟器实例是用普通权限开着的，就没法再以管理员权限启动新实例了。所以**只要有一个实例是普通权限开的，后面的多开就会失败**。

1. 把所有模拟器实例和多开器全部关掉。
2. 回 AUTO-MAS 重新启动任务。还不行就重启电脑，然后直接从 AUTO-MAS 启动，中间别手动开模拟器。
3. 以后自己手动开模拟器和多开器时，都要 **右键 > 以管理员身份运行**。嫌麻烦就给它建个快捷方式，**右键 > 属性 > 快捷方式 > 高级 > 勾选"用管理员身份运行"**，之后双击就是管理员权限。

### 点了配置 MAA，但 MAA 窗口没出来

MAA 大概是躲到托盘里去了。如果你在 MAA 里开了 **启动后直接最小化** 加 **最小化时隐藏至托盘**，就会这样——去右下角托盘区把它点出来继续配。觉得每次都这样太烦，可以改用 **静默模式**。

### 报错"主程序必须是脚本根目录的子路径"

**脚本根目录** 没设或者设错了。这个值是其他路径的基准，必须先把它设对，才能设主程序路径。

### MAA 的设置怎么才算保存成功？

从 AUTO-MAS 里点进去配 MAA，配完回到 AUTO-MAS 点 **保存配置**。绕过 AUTO-MAS 直接开 MAA 改的设置，不会被记录。

### 开了静默模式，模拟器却没最小化

检查模拟器 **老板键** 填对了没，以及这个按键有没有被别的软件抢走（快捷键冲突）。

### 调度队列到点了却没自动跑

两件事挨个查：**定时运行** 的开关是不是真的启用了；软件是不是被关掉了，或者电脑睡眠/休眠了（见下条）。

### 睡眠 / 休眠状态下能跑吗？

**不能。** 睡眠休眠时程序整个是停住的，目前所有脚本都不支持这么用。要定时跑就得让电脑保持唤醒。