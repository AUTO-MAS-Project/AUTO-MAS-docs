# Game Check-in Tool

Opening each community app every day to tap the check-in button gets old. Fill in your credentials here once, and your automation runs will handle check-ins along the way.

Four communities are supported: **Skland**, **miyoushe**, **Kuro Community**, and **Tajiduo**. You don't have to tell it which characters you have — it reads the game characters bound to your account, checks in for each one, and lists every character's status in the results.

::: warning Read This Before You Start

- Check-in requests are made locally by AUTO-MAS. Tokens are only used to reach the official API of the matching community, and are not sent to third-party services unrelated to check-in.
- A token is a login credential. Don't send it to anyone, and don't put it in public logs, screenshots, or config repositories.
- **Get token with account and password** uses your account and password only inside that one login request. Neither is saved: the input is cleared when the request finishes or fails, and nothing is written to config, logs, or notifications.
- AUTO-MAS does not offer SMS-code login. When you use a password or a QR code to obtain credentials, make sure your network and your account are in a safe state.
- Automatic check-in carries risks: account risk control, API changes, and failed check-ins. Evaluate that yourself and accept the consequences of using it.

:::

## Basic Usage

1. Open the **Game Community Check-in** tool in AUTO-MAS.
2. Set **Enable check-in** to **Enabled** at the top.
3. Edit the community credentials for each user. One user can have several communities configured at once.
4. Turn on **Notify after check-in** and **Check in on startup** if you want them.
5. Click **Check in all** to run a check-in immediately by hand.

The community tags in the user list show which communities are configured and the most recent result. Hover over a tag for details such as character, game, status, and rewards.

### Three Ways It Triggers

| When it runs | How often per day | How the notification is sent |
| --- | --- | --- |
| **Along with an automation task** | One attempt per account per day; once done, no repeat requests | Folded into the task completion notification |
| **When the app starts** | Once per startup | Sent as its own notification |
| **When you click Check in all** | Any time, not limited by "already checked in today" | Sent as its own notification |

::: tip You clicked Check in all and it told you to retry later
An automatic check-in is running right now. Wait for it to finish, then click again.
:::

Communities you haven't given credentials to won't appear in the notification, so you won't see a pile of empty entries.

## Skland

Skland check-in automatically looks up the **Arknights** and **Arknights: Endfield** characters under your Hypergryph account. Each character's check-in status is recorded separately.

### Get a Token From the Web

1. Log in to [Skland on the web](https://www.skland.com/).
2. In the same browser session, open the [Hypergryph account credential endpoint](https://web-api.skland.com/account/info/hg).
3. Find `data.content` in the JSON that comes back and copy **only that field's value**:

   ```json
   {
     "code": 0,
     "data": {
       "content": "<Token>"
     }
   }
   ```

4. Paste the token into the **Skland** field in the user edit window and save.

Don't copy the outer JSON, the quotation marks around `content`, or any other field. To switch accounts, clear your browser cookies and log in again — logging out on the web page directly can invalidate the original credential.

### Get a Token With Your Password

1. Click **Get token with account and password** in the **Skland** section of the user edit window.
2. Enter your Hypergryph account phone number and password in the separate popup window.
3. On success, AUTO-MAS validates the returned credential and saves it automatically. A failed login does not overwrite an existing token.

Your account and password only exist in memory for that one request. The input is cleared when the window closes and on both success and failure. This feature does not offer SMS-code login.

## miyoushe

miyoushe automatically reads the characters bound to your account across **Genshin Impact, Honkai: Star Rail, Zenless Zone Zero, Houkai Gakuen 2, Honkai Impact 3rd, and Tears of Themis**, then checks the check-in status for each one.

### Recommended: Get a Token by QR Code

1. Click **Get token by QR code** in the **miyoushe** section of the user edit window.
2. Scan the QR code with the miyoushe app and confirm the login.
3. The credential saves to the current user automatically once you confirm.

QR codes expire. If you see "QR code expired" or "QR code status invalid", click **Regenerate QR code** and scan again. Scanning obtains an authentication cookie; AUTO-MAS does not write that cookie to the frontend log.

### Filling In a Cookie by Hand

You can also run `document.cookie` in the developer tools of a logged-in miyoushe web session and paste the cookie string containing the authentication fields into the input. Don't paste page HTML, a full API response, or unrelated cookies. Without authentication fields such as `cookie_token` and `stoken`, the character lookup and check-in that follow may fail.

## Kuro Community

Reads the **Punishing: Gray Raven** and **Wuthering Waves** characters you have bound, and checks in for each one.

### Get a Token

This is the awkward one: **there is no web page or QR code option**. The token has to be dug out of the client's local login data, and where it lives changes with the client version and your system, so there is no single set of steps to give you here.

1. Log in to the account you want to check in with, using the Kuro Community client.
2. Find the token in the local login data.
3. Paste it into the **Kuro Community** field in the user edit window and save.

For step 2, the open-source project [Kuro_login](https://github.com/mxyooR/Kuro_login) shows one approach.

::: warning That's a Third-Party Project, Not an Official AUTO-MAS One
Read its code and satisfy yourself that the source is trustworthy before using it. Handing login credentials to any third-party tool carries risk that you take on yourself. AUTO-MAS neither asks for nor receives that project's account credentials.
:::

If the account has no characters bound that can be checked in, no Kuro Community entry appears in the results. An expired token shows the reason for the failure in the results.

## Tajiduo

Looks up the game characters on your account and checks in.

One thing worth knowing: if you also want to see your remaining Cloud NTE time, that credential **goes in together with the Tajiduo credential**. It isn't a separate login, so don't go looking for its own entry point.

### Get a Token With Your Password

1. Click **Get token with account and password** in the **Tajiduo / Cloud NTE** section of the user edit window.
2. Enter your Tajiduo account or phone number and password in the separate popup window.
3. On success, AUTO-MAS validates `accessToken`, `refreshToken`, and the user ID, then saves the credential automatically.

If the login fails, the returned fields are incomplete, or risk control kicks in, you won't get a "token saved" confirmation and your existing credential won't be overwritten. Your account and password only exist inside that one login request and are cleared afterwards. This feature does not offer SMS-code login or unverified QR-code login.

### Filling In Credentials by Hand

The input accepts either of these:

- Just a `refreshToken`.
- A credential JSON, which may contain `accessToken`, `refreshToken`, `uid`, `deviceId`, `roleName`, and similar fields.

If you're configuring Cloud NTE as well, add `cloudToken`, `cloudUserId`, and optionally `cloudDeviceId` to the JSON. When credentials refresh, AUTO-MAS updates the saved token during the check-in run, so you don't have to log in again every day.

## Reading the Results

| Status | What it means | What to do |
| --- | --- | --- |
| Success | This request completed the check-in | Nothing |
| Already checked in | Already done earlier today; nothing claimed twice | Nothing, this is normal |
| Failed | The request failed, the credential expired, or character info couldn't be read | Get a new token |
| Risk control | The community wants extra verification, or is refusing requests for now | Try again later; don't retry repeatedly |

Results are shown as community + game + character name. Communities without a token don't take up space, so you won't see empty `0/0` entries.

## Common Questions

### The token expired, or check-in keeps failing

First confirm the account can still log in normally in the official client or on the web. If even the official login fails, the problem isn't AUTO-MAS. If it works, get a fresh token:

- **miyoushe**: scan the QR code again, that's the easiest.
- **Skland, Tajiduo**: reopen their respective password windows.
- **Kuro Community**: dig it out of the client again.

### One of my games isn't in the results

The tool only handles the **bound characters** the community API reports. Confirm that game really is bound to this community account, then click **Check in all** again.

### The login failed but it says the token was saved

That shouldn't happen — a password login has to pass validation before anything is written to config, and a failure saves nothing. If you do hit it, look at the error shown in the interface and report it to the developers.

::: warning Don't Post Your Credentials With a Report
Redact any account, password, cookie, or token from the error message before sharing it.
:::
