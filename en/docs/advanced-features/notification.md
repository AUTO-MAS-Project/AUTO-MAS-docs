# Notifications

AUTO-MAS provides flexible **notification** features. You can configure which notifications to send and which channels to use.

## Global Notifications

Global notifications can be configured in **Settings > Notification Settings**.

According to your push settings, notifications can be sent at any time.

![notification-1](/docs/img/advanced-features/notification-1.png)

## User Notifications

You can also configure user-level notifications in **User Configuration > Notification Settings**, allowing each user to have an independent notification plan.

> User notifications do not override global settings. They send one extra notification to the specified user after the global notification is sent.

![notification-2](/docs/img/advanced-features/notification-2.png)

## SMTP Email Notification Channel

**SMTP** is a reliable email transfer protocol. AUTO-MAS uses **SMTP-SSL** to send email notifications.

::: tip **AUTO-MAS private-domain email is available**

1. What is AUTO-MAS private-domain email?

- AUTO-MAS private-domain email uses the `auto-mas.top` domain through Alibaba Cloud Enterprise Mail.

2. What are its benefits?

- The domain looks more recognizable and professional.

3. How do I use AUTO-MAS private-domain email?

- The login page is [Alibaba Mail Enterprise Edition](https://qiye.aliyun.com), the SMTP server is `smtp.qiye.aliyun.com`, and the authorization code is the `login password`. Other usage is the same as normal email.

4. How do I apply for an AUTO-MAS private-domain email account?

- Send an application email to `DLmaster_361@auto-mas.top`. The email must include: `email name (English letters only)`, `security phone number`, and `application reason`. After the mailbox is created, a reply containing the `initial password` will be sent.
:::

### SMTP Server Address

Select the correct SMTP server address according to the email service provider of the sender mailbox.

| Email service provider | SMTP server address |
| ---------------------- | ------------------- |
| **QQ Mail** | smtp.qq.com |
| **163 Mail** | smtp.163.com |
| **Gmail** | smtp.gmail.com |
| **Outlook/Hotmail** | smtp-mail.outlook.com |
| **Yahoo Mail** | smtp.mail.yahoo.com |

If your email service is not listed, visit its help center or search for its SMTP server address.

### Get an Authorization Code

An **authorization code** is a special password used instead of your mailbox password for third-party client login. You need to fill in the authorization code of the sender mailbox. Common steps are:

1. **QQ Mail**

<Pill name="QQ Mail official guide" image="https://res.wx.qq.com/t/webmail/webmail/res/static/images/projects/login/loginpage/qqmail_logo_default_35h.e071fb4.png" link="https://service.mail.qq.com/detail/0/75"/>

- Log in to your [QQ Mail Account and Security Center](https://wx.mail.qq.com/account).
- Go to **Account and Security > Security Settings > SMTP/IMAP Service**, enable the service, and obtain the authorization code.

2. **163 Mail**

<Pill name="163 Mail official guide" image="https://help.mail.163.com/style/img/logo-163.png" link="https://help.mail.163.com/faqDetail.do?code=d7a5dc8471cd0c0e8b4b8f4f8e49998b374173cfe9171305fa1ce630d7f67ac2a5feb28b66796d3b"/>

- Log in to [163 Mail](https://email.163.com).
- Go to **Settings > POP3/SMTP/IMAP**, find **IMAP/SMTP Service**, and enable it.
- In the popup, click **Continue enabling** and follow the instructions to send an SMS from your phone.
- The popup generates an **authorization password**, which is your authorization code.

3. **Gmail**

- Log in to [Gmail](https://mail.google.com).
- Go to **Settings > See all settings > Forwarding and POP/IMAP > IMAP access**, then select **Enable IMAP**.
- Go to **User > Manage your Google Account > Security > 2-Step Verification** and enable **2-Step Verification**.
- Go to **2-Step Verification > App passwords** and create an **app password**. This password is your authorization code.

4. **Outlook/Hotmail**

- Log in to your **Outlook account**.
- Go to **My Account > Security and privacy > More security options**.
- Create an app password under **App passwords**.

5. **Yahoo Mail**

- Log in to your **Yahoo account**.
- Go to the account's **Security settings**.
- Find **Generate app password** or a similar option to create an app password.

::: warning Note

- For your security, do not share authorization codes with others, and rotate them regularly.
- Some mailbox authorization codes are shown only once. Save them immediately. Some authorization codes expire; replace them before expiration.
- AUTO-MAS encrypts local authorization code data with **Windows DPAPI**. This encryption uses the current user's login credentials as part of the encryption key, which means only the same user on the same computer can decrypt the data. If you need to migrate configuration files across devices, re-enter the authorization code.
- The SMTP email notification service **allows the sender mailbox and recipient mailbox to be the same**. If you do not have an extra mailbox, use the same email address for both sender and recipient.
:::

## ServerChan Notification Channel

**ServerChan** is a communication tool between a **phone** and a **server or smart device**. Its main purpose is:

- Let servers, routers, and other devices push messages to a phone.
- In AUTO-MAS, it is used to **push messages to your phone after automation succeeds**.

More information:

<Box :items="[
{ name: 'ServerChan Turbo', link: 'https://sct.ftqq.com/', image: 'https://the7.ft07.com/sct/images/favicon.png' },
{ name: 'ServerChan³', link: 'https://sc3.ft07.com/', image: 'https://the7.ft07.com/sct/images/favicon.png' },
]"/>

::: warning Note
In 2024, ServerChan introduced a new App push channel. It differs from the original **ServerChan Turbo** (SCT) and was renamed **ServerChan³** (SC3).

In the following configuration, **SCT** refers to **ServerChan Turbo**, and **SC3** refers to **ServerChan³**.
:::

### SendKey

**SendKey** is the authentication method used by the **ServerChan** platform. AUTO-MAS can push messages precisely to your device only when a **SendKey** is provided.

SCT platform key:
<Pill name="SCT SendKey" image="https://the7.ft07.com/sct/images/favicon.png" link="https://sct.ftqq.com/sendkey"/>

SC3 platform key:
<Pill name="SC3 SendKey" image="https://the7.ft07.com/sct/images/favicon.png" link="https://sc3.ft07.com/sendkey"/>

::: warning Note
You only need to choose one of these two platforms. Pick according to your actual use case.
:::

#### ServerChanChannel Code

**SC3 supports only App push**, so the **ServerChanChannel code** is available only for the SCT platform.

The following are **available message channel codes**. Fill in the corresponding numeric code.

| Channel | Code |
| ------- | ---- |
| Official Android app beta | 98 |
| WeCom app message | 66 |
| WeCom group bot | 1 |
| DingTalk group bot | 2 |
| Feishu group bot | 3 |
| Bark iOS | 8 |
| Test account | 0 |
| Custom | 88 |
| PushDeer | 18 |
| Fangtang service account | 9 |

::: tip **Multiple channel format**
Separate multiple channels with `|`, as shown below:

- Incorrect: `1 | 0 | 9`
- Correct: `1|0|9`

If the format is incorrect, the system will use the **default push channel**.
:::

### Tag Content

This is a **new SC3 platform feature** and applies only to **SC3**.

::: tip **Tag format**
Separate multiple tags with `|`, as shown below:

- Incorrect: `AUTO-MAS | Status`
- Correct: `AUTO-MAS|Status`

If left empty or formatted incorrectly, push messages will not include tag information.
:::

## WeCom Group Bot Notification Channel

::: info Tip
This method only needs to be configured once in WeCom. After that, messages can be received directly in WeChat.
:::

1. Register an enterprise account
   - Open the <Pill name="WeCom official website" link="https://work.weixin.qq.com/" image="https://open.work.weixin.qq.com/favicon.ico" /> on a computer and follow the instructions to register an enterprise account.
   - After registration, log in to the WeCom client using the WeChat account or phone number bound during registration.

2. Add a group bot
   - **Desktop**: enter an internal group chat, click the **...** menu in the upper-right corner, and select **Add group bot**.
   - **Mobile**: enter an internal group chat, tap the **...** menu in the upper-right corner, and select **Add group bot**.
   - The bot name and avatar can be filled in freely.

3. Get the group bot Webhook URL
   - The bot creator can view the corresponding Webhook URL in the bot information.
   - **Mobile**: enter the group chat, tap the **...** menu in the upper-right corner, select **Group bot**, then tap the corresponding bot to see the Webhook URL.
   - **Desktop**: in the group chat, right-click the corresponding bot and select **View profile** to get the Webhook URL.

4. Configure push
   - Fill the obtained Webhook URL into AUTO-MAS **Push WeCom bot notification** configuration to enable message push.
