# FAQ

For more issues, see <Pill name="AUTO-MAS GitHub Issues" :image="{ light: '/icons/github.svg', dark: '/icons/github-dark.svg', }" link="https://github.com/AUTO-MAS-Project/AUTO-MAS/issues"/> .

For script-specific issues, read the script documentation or contact the script developers.

## Questions

### Does AUTO-MAS benefit paid account runners?

- When paid account runners use AUTO-MAS, it benefits paid account runners. When regular users use AUTO-MAS, it benefits regular users.
- The wider AUTO-MAS spreads, the more it benefits users. Consider helping promote AUTO-MAS.

### Is my data, such as account passwords, safe?

To keep sensitive information, such as login credentials and access tokens, stored securely on your local machine, AUTO-MAS uses the **Windows Data Protection API (DPAPI)**.

**DPAPI** is a Windows mechanism for encrypting and decrypting sensitive local data. Its master key is derived from the user's login password or system startup key and protected by the operating system kernel, so it is not exposed to applications in plaintext. This means:

- Only you can decrypt the data when logged in to your **Windows account**.
- Even if someone copies your configuration files, they cannot decrypt the data on another computer or account.
- Encryption, decryption, and key management are handled by Windows automatically.

::: warning Warning
Existing data may fail to decrypt in the following cases:

1. **Changing or reinstalling the system**
   If you reinstall Windows or use a new computer account, the encryption key from the original account is lost and the app cannot read old data.
2. **Deleting or resetting the user account password**
   DPAPI encryption keys are bound to your Windows login credentials.
   If you reset the password abnormally, such as through offline modification or system repair tools, Windows cannot decrypt old encrypted files.
3. **Copying data to another computer or account**
   DPAPI-encrypted data is valid only on the original account and computer. Data copied to another environment cannot be decrypted if the key does not match.
:::

## Troubleshooting

### Backend startup fails, then the app keeps reporting Network Error

Check the frontend error page or open `debug/app.log` for the error details.

- **[Errno 10048] error while attempting to bind on address ('0.0.0.0', 36163): Only one usage of each socket address is normally permitted.**

  **The port is occupied.** The default AUTO-MAS backend port is `36163`. Check whether the port is already in use.

- **ModuleNotFoundError: No module named 'xxx'**

  **A dependency is missing.** Delete `environment/.requirements_hash` under the app root and restart the app. If the issue remains, delete the `environment` folder and restart the app.

- **ImportError: DLL load failed while importing onnxruntime_pybind11_state: A dynamic link library (DLL) initialization routine failed.**

  **A system runtime is missing.** AUTO-MAS depends on the **Microsoft Visual C++** runtime. If it is missing, download and install it from [Microsoft Visual C++](https://learn.microsoft.com/en-us/cpp/windows/latest-supported-vc-redist?view=msvc-170#latest-supported-redistributable-version) or directly from [Microsoft Visual C++ x64](https://aka.ms/vc14/vc_redist.x64.exe).

::: tip Note

If neither the frontend error page nor the log file shows an error, run a terminal, PowerShell, or CMD as administrator and execute:

```bash
cd {AUTO-MAS root directory}
.\environment\python\python.exe main.py
```

Replace `{AUTO-MAS root directory}` with the AUTO-MAS installation directory.

The terminal will print error logs that can be used for troubleshooting.

:::

### Emulator startup fails

- All scripts and apps started by AUTO-MAS run with **administrator privileges**. When emulator multi-instance mode is used, some emulator instances may not have administrator privileges and therefore cannot start new emulator instances with administrator privileges. Make sure all current emulator instances and the emulator multi-instance manager are running as administrator:

  1. Close all emulator instances and the emulator multi-instance manager.
  2. Restart the task in AUTO-MAS and check whether it runs normally. If it still fails, restart the computer and start the task directly from AUTO-MAS.
  3. When using the emulator or its multi-instance manager later, **right-click > Run as administrator**. For convenience, you can create a shortcut and enable **Right-click > Properties > Shortcut > Advanced > Run as administrator**.

### Why can't AUTO-MAS open the MAA settings window?

- If **minimize MAA immediately after startup** and **hide to tray when minimized** are enabled in MAA, find MAA in the tray area and continue configuring it there. If this is too time-consuming, try enabling **silent mode**.

### Script configuration reports: the main program must be a subpath of the script root directory

- Check whether the **script root directory** option is correct. You must set this value before setting paths such as the main program path.

### How do I safely save MAA settings?

- Configure MAA in AUTO-MAS and click **Save configuration** after finishing.

### The emulator still does not minimize automatically after silent mode is enabled

- Check whether the emulator boss key is configured correctly and whether there are key conflicts.

### Why did the scheduling queue not run automatically?

- Confirm that **Scheduled run** is **enabled** for the scheduling queue and that the app has not been closed unexpectedly. AUTO-MAS cannot run during sleep or hibernation.

### Can AUTO-MAS run during sleep or hibernation?

- **No.** Known scripts do not support running during sleep or hibernation.
