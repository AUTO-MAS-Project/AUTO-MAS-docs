# 通知

AUTO_MAS提供丰富的 **通知** 功能，您可以配置想要推送的通知内容以及通知推送的渠道。

## 全局通知

你可以于设置→通知设置中设置全局通知。

根据你的推送设置，任何时候都发送通知

![notification-1](..\img\advanced-features\notification-1.png)

## 用户通知

你还可以从用户脚本出设置通知。

**该通知不会覆盖全局设置，而是在发送全局通知后额外再发一封**

![notification-2](..\img\advanced-features\notification-2.png)

## SMTP 邮件推送渠道

**SMTP** 是一种可靠有效的电子邮件传输协议，AUTO_MAS 使用 **SMTP-SSL** 推送电子邮件通知。

::: tip **AUTO_MAS 私域邮箱已上线**

1. AUTO_MAS 私域邮箱是什么？

- AUTO_MAS 私域邮箱是基于阿里云企业邮箱的`automaa.xyz`域名邮箱。

2. AUTO_MAS 私域邮箱的优点是？

- 嗯，域名比较特别，看起来专业点？

3. 怎么用 AUTO_MAS 私域邮箱？

- AUTO_MAS 私域邮箱的登录页为[阿里邮箱企业版](https://qiye.aliyun.com)，SMTP 服务器地址为`smtp.qiye.aliyun.com`，授权码为`登录密码`，其他使用方法与普通邮箱一致。

4. 如何申请 AUTO_MAS 私域邮箱？

- 发送申请邮件到`DLmaster_361@automaa.xyz`，要求邮件中包含：`申请邮箱的类别（用户/项目组）`、`邮箱名（纯英文）`、`安全手机号`、`申请原因`。邮箱创建成功后，将会返回一封包含`初始密码`的回执邮件。
  :::

### SMTP 服务器地址

请根据发信邮箱的电子邮件服务提供商选择正确的 SMTP 服务器地址。

| 电子邮件服务提供商  | SMTP 服务器地址       |
| ------------------- | --------------------- |
| **QQ邮箱**          | smtp.qq.com           |
| **163邮箱**         | smtp.163.com          |
| **Gmail**           | smtp.gmail.com        |
| **Outlook/Hotmail** | smtp-mail.outlook.com |
| **Yahoo Mail**      | smtp.mail.yahoo.com   |

若未找到您使用的电子邮件服务，请访问该邮件服务的帮助中心或搜索其 SMTP 服务器地址。

### 获取授权码

**授权码** 是用于替代您的邮箱密码进行第三方客户端登录的一种特殊密码，您需要填写发信邮箱的授权码。常见邮件服务商授权码的一般获取步骤如下：

1. **QQ 邮箱**

<Pill name="QQ邮箱官方教程" image="https://res.wx.qq.com/t/webmail/webmail/res/static/images/projects/login/loginpage/qqmail_logo_default_35h.e071fb4.png" link="https://service.mail.qq.com/detail/0/75"/>

  - 登录到您的 [QQ 邮箱账号与安全中心](https://wx.mail.qq.com/account)。
  - 在 **账号与安全 > 安全设置 > SMTP/IMAP服务** 中开启服务并获取授权码。

2. **163 邮箱**

<Pill name="163邮箱官方教程" image="https://help.mail.163.com/style/img/logo-163.png" link="https://help.mail.163.com/faqDetail.do?code=d7a5dc8471cd0c0e8b4b8f4f8e49998b374173cfe9171305fa1ce630d7f67ac2a5feb28b66796d3b"/>

  - 登录到您的 [163 邮箱](https://email.163.com)。
  - 进入 **设置 > POP3/SMTP/IMAP**，找到 **IMAP/SMTP服务** 并点击开启。
  - 在弹窗中点击 **继续开启**，根据指示在手机中发送短信。
  - 弹窗生成 **授权密码**，该密码便为您的授权码。

3. **Gmail**

  - 登录到您的 [Gmail](https://mail.google.com)。
  - 进入 **设置 > 查看所有设置 > 转发和 POP/IMAP > IMAP 访问**，选择 **启用 IMAP**。
  - 进入 **用户 > 管理您的 Google 账号 > 安全性 > 两步验证**，按提示开启 **两步验证**。
  - 进入 **两步验证 > 应用专用密码**，按提示创建 **应用专用密码**，该密码便为您的授权码。

4. **Outlook/Hotmail**

  - 登录到您的 **Outlook 账户**。
  - 进入 **我的账户 > 安全和隐私 > 更多安全选项**。
  - 在 **应用程序密码** 中创建应用程序密码。

5. **Yahoo Mail**

  - 登录到您的 **Yahoo 账户**。
  - 前往账户的 **安全设置**。
  - 找到 **生成应用程序密码** 或类似选项以创建应用密码。


::: warning 注意

- 为了您的信息安全，请勿将授权码告诉他人，并定期更换。
- 部分邮箱的授权码仅显示一次，请及时保存；部分邮箱的授权码存在有效期，请在到期前及时更换。
- AUTO_MAS 已对本地授权码数据使用 **Windows DPAPI** 加密，这种加密方式将当前用户的登录凭据作为加密密钥的一部分，这意味着只有同一个用户在同一台计算机上才能解密数据。如果您需要跨设备迁移配置文件，请重新输入授权码。
- SMTP 邮件推送服务**允许发信邮箱与收信邮箱相同**，若没有多余的电子邮箱，可以填写相同的发信邮箱与收信邮箱地址。
  :::

## ServerChan 通知推送渠道

**「Server酱」**，英文名 **「ServerChan」**，是一款 **手机** 与 **服务器/智能设备** 之间的通信工具。它的主要作用是：

- 让服务器、路由器等设备推送消息到手机。
- 在这里，它用于 **AUTO_MAS 代理成功后推送消息到手机**。

更多信息请查看：

<Box :items="[
{ name: 'Server酱Turbo版', link: 'https://sct.ftqq.com/', image: 'https://the7.ft07.com/sct/images/favicon.png' },
{ name: 'Server酱³', link: 'https://sc3.ft07.com/', image: 'https://the7.ft07.com/sct/images/favicon.png' },
]"/>

::: warning 注意
Server酱官方在 2024 年推出了一种新的 App 推送渠道，与原来的 **Server酱·Turbo 版** （SCT）不同，并重新命名为 **Server酱³**
（SC3）。

在以下配置中，我们使用 **SCT** 代指 **Server酱·Turbo版**，**SC3** 代指 **Server酱³**。
:::

### SendKey

**SendKey** 是 **Server酱** 平台对用户的认证方式。只有 **提供 SendKey**，AUTO_MAS 才能将消息精准推送到您的设备。

SCT
平台获取Key：<Pill name="SCT SendKey" image="https://the7.ft07.com/sct/images/favicon.png" link="https://sct.ftqq.com/sendkey"/>

SC3
平台获取Key：<Pill name="SC3 SendKey" image="https://the7.ft07.com/sct/images/favicon.png" link="https://sc3.ft07.com/sendkey"/>

::: warning 注意
以上两个平台仅需选择一个即可，请根据您的实际情况选择。
:::

#### ServerChanChannel 代码

**SC3 仅支持 App 推送**，因此 **仅 SCT 平台可填写 ServerChanChannel 代码**。

以下是 **可用消息渠道代码**，您需要填写 **对应的数字代码**。

| 渠道              | 代码 |
| ----------------- | ---- |
| 官方 Android 版·β | 98   |
| 企业微信应用消息  | 66   |
| 企业微信群机器人  | 1    |
| 钉钉群机器人      | 2    |
| 飞书群机器人      | 3    |
| Bark iOS          | 8    |
| 测试号            | 0    |
| 自定义            | 88   |
| PushDeer          | 18   |
| 方糖服务号        | 9    |

::: tip **多个渠道填写格式**
**多个渠道请用 `|` 分隔，格式如下：**

- ❌ `1 | 0 | 9`
- ✔️ `1|0|9`

如果未正确填写，系统将使用 **默认推送渠道**。
:::

### Tag 内容

该功能是 **SC3 平台的新特性**，仅适用于 **SC3**。

::: tip **Tag 填写格式**
**多个 Tag 请用 `|` 分隔**，格式如下：

- ❌ `AUTO_MAS | 代理情况`
- ✔️ `AUTO_MAS|代理情况`

若留空或填写不正确，则推送消息时不会携带 Tag 信息。
:::

## 企业微信群机器人通知推送渠道

::: info 提示
本方法只用在企业微信配置一次，后续可直接在微信中接收消息
:::

1. 注册企业账号
   - 在电脑上打开 <Pill name="企业微信官网" link="https://work.weixin.qq.com/" image="https://open.work.weixin.qq.com/favicon.ico" />，按照指引完成企业账号注册。
   - 注册完成后，用注册时绑定的微信号或手机号登录企业微信客户端。

2. 添加群机器人
   - **电脑端**：进入内部群聊，点击右上角的 **···** 菜单，选择 **添加群机器人**。
   - **手机端**：在内部群聊中，点击右上角的 **···** 菜单，然后选择 **添加群机器人**。
   - 机器人的名称和头像可随意填写

3. 获取群机器人 Webhook 地址
   - 机器人创建者可在查看机器人信息时获取对应的 Webhook URL。
   - **手机端**：进入群聊，点击右上角的 **···** 菜单，选择 **群机器人**，点击对应机器人后即可看到 Webhook 地址。
   - **电脑端**：在群聊中，右键点击相应机器人，选择 **查看资料**，即可获取 Webhook 地址。

4. 配置推送
   - 将获取的 Webhook URL 填写到 AUTO_MAS 的 **推送企业微信机器人通知** 配置项中，即可实现消息推送。