# MAA 配置方法

## 什么是 MAA？

MAA 是一个明日方舟第三方软件，能够轻松完成明日方舟日常代理、肉鸽存钱等重复性无趣工作。

**详情信息请查阅**：

<Box :items="[
{ name: 'MAA 官网', link: 'https://maa.plus/', image: 'https://maa.plus/favicon.ico', },
{ name: 'MAA GitHub', link: 'https://github.com/MaaAssistantArknights/MaaAssistantArknights', image: { light: '/icons/github.svg', dark: '/icons/github-dark.svg', }, },]"/>

## 安装 MAA

1. 前往 <Pill name="MAA 官网" image="https://maa.plus/favicon.ico" link="https://maa.plus"/>、<Pill name="MAA 仓库" :image="{ light: '/icons/github.svg', dark: '/icons/github-dark.svg', }" link="https://github.com/MaaAssistantArknights/MaaAssistantArknights/releases/latest"/> 或 <Pill name="Mirror 酱" image="https://mirrorchyan.com/favicon.ico" link="https://mirrorchyan.com/zh/projects?rid=MAA&scouce=AUTO_MAA-Web"/> 下载软件压缩包。
2. 将 MAA 压缩包解压至任意文件夹。


## 配置脚本

1. 进入 **脚本管理**，单击 **新建脚本** 并选择 **MAA脚本** 以添加脚本实例管理页面。
![AUTO_MAS配置3](/docs/img/script-guide/maa/AUTO-MAA-1.png)

2. 在 **打开的脚本配置** 中的 **MAA路径** 单击 **选择文件夹**，打开 MAA 软件所在目录。
![AUTO_MAS配置4](/docs/img/script-guide/maa/AUTO-MAA-2.png)

3. 在 **模拟器管理** 中选择模拟器和模拟器实例。
> 如果此处没有模拟器，请先完成 **模拟器管理配置**

4. 点击 **配置 MAA** 在 MAA 中配置。
![AUTO_MAS配置5](/docs/img/script-guide/maa/AUTO-MAA-5.png)

5. 手动取消勾选 **开机自启动MAA**，并完成 ADB 连接相关配置，其余配置可以根据您的喜好设置。

6. 完成配置后，关闭 **MAA**，并在 AUTO-MAS 中点击 **保存配置**。
![AUTO_MAS配置6](/docs/img/script-guide/maa/AUTO-MAA-6.png)

## 配置用户

1. 在 **脚本管理** 的脚本表格内，单击 **添加用户** 以添加一个用户。
![AUTO_MAS配置7](/docs/img/script-guide/maa/AUTO-MAA-7.png)
2. 按照设置卡相关提示填写用户信息。
![AUTO_MAS配置8](/docs/img/script-guide/maa/AUTO-MAA-8.png)

::: info 注意
**详细** 配置模式下不要忘了 **设置具体配置**。
:::
::: tip 账号 ID 小贴士

由于 MAA 在 **B服账号切换** 功能中使用的 OCR（文字识别）技术准确率有限，我们建议如下填写方式以提高识别和切换成功率：

#### 通用建议

- MAA 的账号切换功能只需**成功识别一个唯一片段**即可完成切换。
- 请填写**仅该账号独有的部分片段**，避免与其他账号重复。
- 建议在 MAA 中测试填写片段能否正常切换账号，确认无误后再填写至 AUTO_MAS。
- 若同区服仅有一个账号，也可将 `账号ID` 留空。

#### 官服

- 官服账号ID为手机号，通常只需填写**后四位数字**即可，无需填写完整手机号。~~（你也不想无意中泄露自己的手机号吧~）~~ 
- 示例：
  - 账号1：`133XXXX1234`
  - 账号2：`133XXXX5678`
  - 若要切换账号1，可填写 `2`、`4`、`12`、`34`、`123`、`234`、`1234` 等仅账号1拥有的片段。

#### B服

- B服账号为 B站昵称，可能包含**中文、英文、数字、特殊符号、日文**等复杂字符，OCR 模型识别准确率偏低。因此，**不建议填写完整昵称**，建议填写 **不易识别错误的、唯一片段**，避免：
  - 生僻字（如：`黍`，可能被识别为其他文字）
  - 下划线 `_`（常被识别错误或漏识）
- 示例：
  - 账号1：`DLmaster_361` → 可填写 `master`、`361`
  - 账号2：`黍的XX_1234` → 可填写 `1234`、`的`、`XX`，**不建议填写“黍”或下划线**

经过这些修改，你应该能够稳定的进行账号切换
:::

### 配置说明

以下为 **自动代理** 模式下，AUTO-MAS 针对 MAA 的配置策略。

1. 用户配置页展示的配置项优先生效。
2. **剿灭** 任务中，仅开启 **开始唤醒**、**刷理智** 任务，**刷理智** 中仅进行 **剿灭模式** 关卡代理；**日常** 任务中，开启 **任务配置** 中启用的任务。
3. **定时执行** 保持关闭，**任务完成后行为**、**启动 MAA 后行为**、**MAA 最小化相关设置**、**更新相关设置** 按实际配置与执行情况自动调整。
4. **简洁** 配置模式下其他配置沿用 **MAA 全局设置**；**详细** 配置模式下其他配置沿用 **用户具体配置**。

## 计划表

依靠计划表，您可以以周为单位定制关卡代理方案。

![plan](..\img\advanced-features\plan-1.png)

十分通俗易懂！

切换配置模式为周计划模式后，你就可以决定不同时间刷什么。

切换为简化视图，可以拥有类似 mower 的编辑体验。

之后，你可以在 MAA 用户界面的关卡配置模式中选择计划表。

![plan](..\img\advanced-features\plan-2.png)

## 森空岛自动签到

::: warning 注意
AUTO-MAS 在本地处理签到请求，不会将 Token 上传至第三方服务器。

自动签到有一定风险，AUTO-MAS 不对自动签到所造成的任何结果负责。使用此功能代表您同意自行承担相关风险。
:::

### 获取鹰角网络通行证登录凭证

1. 登录[森空岛网页端](https://www.skland.com/)。

2. 访问此[网址](https://web-api.skland.com/account/info/hg)。

   返回如下信息

   ```json
   {
     "code": 0,
     "data": {
       "content": "<Token>"
     },
     "msg": "接口会返回您的鹰角网络通行证账号的登录凭证，此凭证可以用于鹰角网络账号系统校验您登录的有效性。泄露登录凭证属于极度危险操作，为了您的账号安全，请勿将此凭证以任何形式告知他人！"
   }
   ```

3. 将 `<Token>` 输入到软件对应选项卡中。

4. 若您需要连续获取多个账号的 **鹰角网络通行证登录凭证**，请通过 **清理浏览器 cookie** 清除登录状态。直接在网页端退出登录将导致 Token 过期失效。

::: tip 提醒
注意不要把包裹 `content` 内容的引号，或是页面返回的整个内容输入到选项卡中！
:::