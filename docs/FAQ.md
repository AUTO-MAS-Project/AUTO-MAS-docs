# 常见问题

更多问题请参考 <Pill name="AUTO-MAS GitHub Issues" :image="{ light: '/icons/github.svg', dark: '/icons/github-dark.svg', }" link="https://github.com/DLmaster361/AUTO-MAS/issues"/> 。

脚本软件问题请阅读脚本文档或咨询脚本开发者

## 疑问解答

### **AUTO-MAS 与 MAA 有什么关系？**

- AUTO-MAS 原名 AUTO-MAS，最初是 MAA 的多账号调度工具，v5 版本后开始尝试调度一切脚本。
- AUTO-MAS 项目组与 MAA 项目组是完全独立的两班人马。
- AUTO-MAS 代码与 MAA 或 MFW 完全无关，设计思路、用途与架构都完全不同。

### **AUTO-MAS 利好代肝吗？**

- 代肝用 AUTO-MAS 就是利好代肝，用户用 AUTO-MAS 就是利好用户。
- 而且 AUTO-MAS 的传播范围越广，就越利好用户，所以还不快帮 AUTO-MAS 宣传一波！

### 我的数据（账号密码）安全吗？

为了确保您的敏感信息（如登录凭据、访问令牌等）在本地安全存储，我们使用了 **Windows 数据保护 API（DPAPI）**。

**DPAPI** 是 Windows 提供的一套用于加密和解密敏感数据的本地安全机制，主密钥（Master Key）由用户的登录密码（或系统启动密钥）派生而来，并通过操作系统内核保护，不会明文暴露给应用程序。这意味着：

  - 只有您本人登录 **Windows 账号** 时，程序才能解密这些数据。
  - 即使他人复制了配置文件，也无法在其他电脑或账户中解密数据。
  - 加解密过程和密钥管理完全由 Windows 系统自动处理，安全且可靠。

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

### **后端启动失败，跳过后应用内不停报错 Network Error**

打开 `debug/frontend.log` 查看错误信息。

- [Errno 10048] error while attempting to bind on address ('0.0.0.0', 36163): 通常每个套接字地址(协议/网络地址/端口)只允许使用一次。

  **端口被占用**，AUTO-MAS 后端默认端口为 `36163`，请检查端口是否被占用。

- ModuleNotFoundError: No module named 'xxx'

  **缺少依赖**，删除软件根目录下 `environment/.requirements_hash` 文件并重启软件，仍无法解决请删除 `environment` 文件夹并重启软件。

- ImportError: DLL load failed while importing onnxruntime_pybind11_state: 动态链接库(DLL)初始化例程失败。

  **缺少系统环境**，AUTO-MAS 运行依赖于 **Microsoft Visual C++** 系统环境，若缺失该环境，您需要手动从 [Microsoft Visual C++](https://learn.microsoft.com/zh-cn/cpp/windows/latest-supported-vc-redist?view=msvc-170#latest-supported-redistributable-version) 或直接从 [Microsoft Visual C++ x64](https://aka.ms/vc14/vc_redist.x64.exe) 中下载并安装。

### **为什么 AUTO-MAS 无法打开 MAA 设置窗口？**

- 若您在 MAA 中启用了 **启动 MAA 后直接最小化** 与 **最小化时隐藏至托盘**，请您从托盘区找到 MAA 后继续配置。若您认为该操作过于费时，可尝试启用 **静默模式**。

### 脚本配置时报错：主程序必须是脚本根目录的子路径

- 请先检查你的 **脚本根目录** 选项是否正确，你必须先设定此值，才能设置主程序路径等路径

### **如何安全保存 MAA 的设置？**

- 请在 AUTO-MAS 中进行 MAA 配置，并在完成配置后点击 **保存配置**。

### **打开静默模式后模拟器仍未能自动最小化？**

  - 请检查是否正确填写模拟器老板键，并确认是否存在按键冲突情况。

### **调度队列为何没有自动运行？**

- 请确认调度队列的 **定时运行** 已 **开启**，并且 **软件未被意外关闭**，任何睡眠休眠等情况均无法运行AUTO-MAS（见下）。

### 可以休眠/睡眠运行 AUTO-MAS 吗？

- **不能**，已知的所有脚本都不支持在睡眠/休眠中运行。