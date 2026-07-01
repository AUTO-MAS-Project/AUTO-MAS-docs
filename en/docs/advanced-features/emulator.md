# Emulator Management

Emulator management is a distinctive MAS feature. It is designed to solve common bugs caused by emulator behavior and emulator adaptation once and for all.

![emulator management](/docs/img/advanced-features/emulator-1.png)

*The screenshot shows example content. It will look like this only after configuration.*

## Search for Emulators

During initial configuration, you can use automatic search or manual search. If you use automatic multi-instance manager search, one click is enough.

MAS automatically searches for emulators installed under their **default installation paths**.

If you changed the default path, MAS cannot find the emulator automatically and you need to add it manually.

First, understand the configuration fields.

## Configuration Explanation

**Emulator name**: a display name used only inside MAS.

**Emulator type**: the emulator software name, such as MuMu emulator or LDPlayer. Select it from the dropdown.

::: tip Best Practice

AUTO-MAS strongly recommends MuMu 12 or LDPlayer because they have good performance, reliable screenshots, and mature multi-instance manager support. If you are installing an emulator from scratch, consider one of them.

:::

**Emulator path**: the directory of the emulator **multi-instance manager** for the corresponding software.

::: tip Common Emulator Paths

MuMu 12 v4: `MuMu installation directory\shell\MuMuManager.exe`

MuMu 12 v5: `MuMu installation directory\nx_main\MuMuManager.exe`

LDPlayer: `LDPlayer installation directory\LDPlayer9\dnplayer.exe`

:::

**Maximum wait time**: how long AUTO-MAS waits for the emulator to start when a script uses this emulator. The script program starts only after the emulator starts.

**Boss key**: pressed automatically when silent mode is used.

## Notes

Emulator management is essentially a multi-instance manager integration. AUTO-MAS uses a series of command-line commands to obtain emulator information and then fills the corresponding script configuration fields automatically.
