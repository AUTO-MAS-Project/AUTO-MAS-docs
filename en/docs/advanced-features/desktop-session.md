# Desktop Session

Desktop Session runs PC games and managed scripts on a separate Windows desktop. You can watch the game in the session preview and keep using the mouse and keyboard on your main desktop. This experimental feature requires the x64 desktop app on Windows 10 version 1809 or later, or Windows 11.

The steps below follow **dev after v5.9.0-beta.1 (be8ef6d5c)**.

## Open a session and run tasks

For Whimbox, start AUTO-MAS as administrator. Select the script or its queue here, or open the Whimbox editor and select **Run configuration → Execution session → Desktop Session**. Home and Scheduler use the saved choice when you manually start that script on its own. If permissions change, the choice is preserved. Restore permissions or select **Main session** in the editor.

1. Go to **Settings → Desktop Session** and select a managed script or queue. Every managed script in a queue must support Desktop Session, and the queue's completion action must be set to **Do nothing**. Run emulator tasks in the main session. See the [Desktop Session compatibility table](/en/docs/script-guide/#desktop-session).
2. The default resolution is **1920 × 1080**. You can also choose **Match primary display**. Start with the default minimize mode, **Minimize to background (recommended)**.
3. Click **Open session and start tasks**. To open the desktop by itself, clear the selection and click **Open desktop session**.
4. If **Verify Windows password** appears after the environment check, enter the password for your current Windows account. If the check or connection fails, follow the instructions that match the error. [Environment check](#environment-check) · [Password and connection](#authentication)
5. Use **Scheduler** in the main window to check progress or stop tasks. If **Confirm task dispatch** appears, check where each queue or managed script will run. Choose **Confirm** to dispatch as listed or **Run Desktop Session tasks only** to skip the tasks listed under the main session.

To use Desktop Session for a particular managed script, edit it and set **Execution session** to **Desktop Session**. This setting applies when you manually start that script by itself. Queues and scheduled runs use their own dispatch rules.

Click **Hide** to tuck away the preview while keeping the session connected. Click **Show** when you need the preview again.

## Environment check failed {#environment-check}

In the session window, check the entries under **System environment**. After addressing the reported problems, click **Check environment again** in the same window.

If **Try to repair environment** is available, review the proposed changes, click **Confirm repair**, and approve the Windows administrator prompt. The app can enable Windows child sessions or local Remote Desktop and start Remote Desktop Services. If that service is disabled, the app sets it to Manual and tries to start it. The changes are kept if the recheck passes. Enabling Remote Desktop may allow inbound RDP under your current firewall and network settings.

| Check result | What to do |
| --- | --- |
| Windows x64 or an interactive desktop is required | Sign in to Windows and open the x64 desktop app. If running from source, use x64 Python as well. |
| Windows child sessions are unavailable or Remote Desktop is disabled in local settings | Use **Try to repair environment** if available. If the button is missing, address the other failed checks first and check again. |
| Remote Desktop connections are disabled by Group Policy | On a work or school PC, ask the device administrator whether Remote Desktop is allowed. |
| Remote Desktop Services or Task Scheduler is stopped | Press `Win + R` and enter `services.msc`. Check **Remote Desktop Services** (`TermService`) or **Task Scheduler** (`Schedule`), and start the affected service. If it is missing, fails to start, or is managed by policy, give the service name and Windows error to the administrator. |
| Local RDP listener is missing or the port is invalid | Check **Windows Settings → System → Remote Desktop** and **Remote Desktop Services**. If the RDP port was changed, ask the administrator to check the port configuration. On Windows Home, you can still try to connect if the check only reports that no standard RDP listener was detected. |
| Remote Desktop Services is not using the native DLL | If you installed RDP Wrapper or modified Remote Desktop components, follow the tool's uninstall instructions to restore the original Windows configuration, then recheck. |
| Remote Desktop Services has no ServiceDll setting | Ask the device administrator to check the Remote Desktop Services configuration, then recheck. |
| Missing `termsrv.dll`, unverified file signature, or a Windows component fails to load | Open Command Prompt as administrator and run `DISM.exe /Online /Cleanup-image /Restorehealth`, then `sfc /scannow`. Restart Windows and recheck. [Microsoft's System File Checker instructions](https://support.microsoft.com/en-us/windows/experience/backup-recovery/using-system-file-checker-in-windows) |
| Windows PowerShell could not verify the signature | Check that the built-in **Windows PowerShell** opens. Contact the device administrator if policy blocks it. |
| Could not read the child-session setting or service status | Reopen AUTO-MAS from your signed-in Windows desktop and recheck. If it still fails, collect the session logs. [Collect session logs](#collect-logs) |

If the app reports that the repair failed, the recheck still failed, or it could not confirm that the original settings were restored, click **Export desktop session logs** and include the ZIP and full error in your report. If the recheck fails, the app attempts to restore the original settings. If it cannot confirm the restoration, ask the administrator to check the settings listed in the repair dialog. [Collect session logs](#collect-logs)

## Password verification and Remote Desktop connection failures {#authentication}

Enter the actual password for your current Windows account in **Verify Windows password**. This applies if you normally sign in to Windows with a PIN, fingerprint, or face recognition. If the saved credentials cannot be decrypted or belong to a different account, reconnect with the current account's password.

1. For a **local account**, use the local sign-in password. If you only set up a PIN, check whether the account has a password under **Windows Settings → Accounts → Sign-in options**.
2. For a **Microsoft account**, make sure the PC is online. If the app reports that Windows Hello-only sign-in is enabled, you can turn off that setting under **Windows Settings → Accounts → Sign-in options** and confirm you can sign in with a password before retrying. Contact the device administrator if the setting is managed. [Microsoft's Windows sign-in options](https://support.microsoft.com/en-us/accounts-billing/security/sign-in-options-in-windows)
3. If password verification, Remote Desktop authentication, or the Remote Desktop connection still fails, note the time, status, and substatus of **Windows Security event 4625** if shown. Otherwise, press `Win + R`, enter `eventvwr.msc`, and look for event **4625** at the same time under **Windows Logs → Security**. [Microsoft's event 4625 reference](https://learn.microsoft.com/windows/security/threat-protection/auditing/event-4625)

| Common status or substatus | What to do |
| --- | --- |
| `0xC000006A` (incorrect password) | Check the account password, keyboard layout, and letter case before re-entering it. |
| `0xC0000234` (account locked) | Stop retrying until the account is unlocked. For a work or school account, contact the administrator. |
| `0xC000015B` (required logon right missing) | Ask the administrator to check the account's Remote Desktop sign-in permissions. |

If the password was verified but could not be encrypted and saved, you can continue the current connection and may need to enter the password again next time. If Windows Home reports a child-session compatibility issue, check the credentials and Security event first. If the connection still fails after ruling out authentication problems, run tasks in the main session or consider manually upgrading to Windows Pro.

## Task dispatch, preview, and shutdown {#task-errors}

| What you see | What to do |
| --- | --- |
| The queue is unavailable and asks you to set its completion action to Do nothing | Edit the queue, set its completion action to **Do nothing**, then select the queue again in Desktop Session settings. |
| A managed script or queue is unsupported, or Desktop Session is missing from **Execution session** | Check the script's controller, launch method, and permissions. Move unsupported scripts to a separate queue for the main session. [Desktop Session compatibility table](/en/docs/script-guide/#desktop-session) |
| A queue or managed script no longer exists | Return to **Settings → Desktop Session** and select an existing queue or managed script. |
| A task could not be dispatched | Click **View session logs** and check the entries around the time of the error. Check **Scheduler** to see whether the task was dispatched. [Collect session logs](#collect-logs) |
| The preview stops responding, exits, disconnects, or reports that the child session changed | Reopen the session and check the previous run in **Scheduler** before starting the tasks again. |
| The image or script stalls after minimizing, or the resolution differs from the connection settings | Click **Show** to restore the preview. If the problem persists, select **Minimize to background (recommended)** and **1920 × 1080** in the session settings, then close and reopen the session. |
| An existing Windows child session was detected | Check whether an app such as BetterGI opened it. To reuse it, choose **Take over**. When closing a session you took over, **Disconnect only** stops the MAS tasks in the session and leaves other programs running. |
| The desktop session could not be closed | Check in **Scheduler** whether the task has ended, view the session logs, and try closing again. Save open work in other programs in the session before choosing **Sign out of session**. |

## Collect session logs {#collect-logs}

For environment repair errors, use **Export desktop session logs** in the session window. If that button is unavailable, choose **View session logs** and collect `host.log` and `control.log`. If the preview cannot open, use **Settings → Advanced Settings → Export a log archive** in the main window.

Include the AUTO-MAS version, Windows version and edition, the full error, what you were doing, and when it happened. For dispatch failures, also include the managed script type, launch method, and queue completion action. Use `winver` to check your Windows version. Remove account names, local paths, and other private details before sharing.
