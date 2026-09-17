# FAQ

If your question isn't here, have a look through <Pill name="AUTO-MAS GitHub Issues" :image="{ light: '/icons/github.svg', dark: '/icons/github-dark.svg', }" link="https://github.com/AUTO-MAS-Project/AUTO-MAS/issues"/> .

If the problem is in the script itself (MAA failing to recognize a stage, for example), that's the script's business. Check that script's documentation or ask its author. AUTO-MAS only schedules them.

## Questions

### **Does AUTO-MAS benefit paid account runners?**

- When paid account runners use AUTO-MAS, it benefits paid account runners. When regular users use AUTO-MAS, it benefits regular users.
- And the wider AUTO-MAS spreads, the more it benefits users, so go help spread the word.

### Are my account passwords safe?

Yes. The passwords and tokens you enter are encrypted with Windows' own encryption (DPAPI) and stored on your machine. AUTO-MAS never uploads them to any server.

That encryption is tied to your Windows login account, which means:

- The app can only decrypt the data while you are signed in to that Windows account on that computer.
- Even if someone copies the whole config folder, they can't decrypt it on another computer.

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

### The app keeps showing Network Error

That means the backend didn't start. First find the actual error, either on the error page in the app or in `debug/app.log`, then match it against the list below:

- **`[Errno 10048] error while attempting to bind on address ('0.0.0.0', 36163)`**

  Another program has taken the port. The AUTO-MAS backend uses port `36163`. Find what's holding it and close that.

- **`ModuleNotFoundError: No module named 'xxx'`**

  Dependencies are incomplete. Delete `environment/.requirements_hash` in the install directory and restart the app so it reinstalls them. If that doesn't help, delete the whole `environment` folder and restart.

- **`ImportError: DLL load failed while importing onnxruntime_pybind11_state`**

  Your system is missing the **Microsoft Visual C++** runtime. Installing it fixes this: [download the x64 build directly](https://aka.ms/vc14/vc_redist.x64.exe), or pick a version from the [Microsoft page](https://learn.microsoft.com/en-us/cpp/windows/latest-supported-vc-redist?view=msvc-170#latest-supported-redistributable-version).

::: tip When neither the error page nor the log shows an error

Run the backend by hand to force the error out. Open a terminal (PowerShell or CMD) as administrator and run:

```bash
cd {AUTO-MAS root directory}
.\environment\python\python.exe main.py
```

Replace `{AUTO-MAS root directory}` with your actual install path. Whatever the terminal prints is your lead.

:::

### The emulator fails to start

This is almost always mismatched privileges. Everything AUTO-MAS launches runs as administrator, but if an emulator instance is already open without administrator rights, a new instance can no longer be started as administrator. So **one instance opened with normal privileges is enough to break every additional instance after it**.

1. Close every emulator instance and the multi-instance manager.
2. Go back to AUTO-MAS and start the task again. If it still fails, restart the computer and start it straight from AUTO-MAS, without opening the emulator by hand in between.
3. From then on, when you open the emulator or the multi-instance manager yourself, use **right-click > Run as administrator**. If that gets tedious, make a shortcut and set **right-click > Properties > Shortcut > Advanced > Run as administrator**, so double-clicking it always runs as administrator.

### I clicked configure MAA, but the MAA window never appeared

MAA has probably hidden itself in the tray. If you enabled **minimize immediately after startup** plus **hide to tray when minimized** in MAA, this is what happens. Find it in the tray at the bottom right and click it to keep configuring. If that gets old, switch to **silent mode**.

### Error: the main program must be a subpath of the script root directory

The **script root directory** is unset or wrong. Every other path is measured from it, so you have to set it correctly before you can set the main program path.

### How do I know MAA's settings actually saved?

Open MAA from inside AUTO-MAS, configure it, then come back to AUTO-MAS and click **Save configuration**. Settings you change by opening MAA directly, bypassing AUTO-MAS, are not recorded.

### Silent mode is on but the emulator doesn't minimize

Check that the emulator's **boss key** is set correctly, and that no other software has claimed that key combination.

### The scheduling queue didn't run at its scheduled time

Two things to check: whether **Scheduled run** is actually enabled, and whether the app was closed or the computer went to sleep or hibernation (see below).

### Can it run while the computer is asleep or hibernating?

**No.** Sleep and hibernation stop the program entirely, and no current script supports being used that way. For scheduled runs, the computer has to stay awake.
