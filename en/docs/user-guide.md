# Getting Started

## Prerequisites

### What is AUTO-MAS?

**AUTO-MAS** is a log-monitoring based multi-script, multi-configuration management and automation tool. It controls other script programs, such as MAA, by modifying configuration files and listening to logs, making multi-account automation easier to manage.

> AUTO-MAS: Make ALL Scripts Auto

## Usage

### Install AUTO-MAS

1. Go to the [download page](/en/download/auto-mas) and get the latest installer package.
2. Install according to the package type:
   - Installer package: extract the archive and run `AUTO-MAS-Setup.exe`, then follow the installation wizard.
   - Portable package: extract the archive to the target install location, then run `AUTO-MAS.exe` to start the app.

::: tip Choosing a Package

Some download channels allow you to choose the package type.

- **Installer and portable packages**
  - Packages with `setup` in the name are installer packages and can be installed by running the installer.
  - Packages without `setup` are portable packages and can be used after extracting them to the install location.

- **Full and lite packages**
  - Packages with `full` in the name include most dependencies and usually start faster on first launch.
  - Packages with `lite` in the name do not include dependencies. Dependencies will be downloaded and installed automatically on first launch.
:::

### Trust the App

Before running AUTO-MAS, add the `AUTO-MAS installation directory` and the `script software installation directory` to Windows Defender exclusions and to the trusted/developer directory of any antivirus software. The following steps show how to add Windows Defender exclusions:

Quick link: <Pill name="Windows Security" link="ms-settings:windowsdefender"/>

1. If another antivirus program is installed, enable **Periodic scanning** first.
![Windows Defender configuration 1](/docs/img/WD-1.png)

2. **Virus & threat protection settings > Manage settings**
![Windows Defender configuration 2](/docs/img/WD-2.png)

3. **Exclusions > Add or remove exclusions**
![Windows Defender configuration 3](/docs/img/WD-3.png)

4. **Add an exclusion > Select the corresponding directory**
![Windows Defender configuration 4](/docs/img/WD-4.png)

5. If another antivirus program is installed, disable **Periodic scanning** again.

::: warning Note
Even if another antivirus program is installed, such as Huorong or 360 Extreme Edition, **Windows Defender** may still enable its protection from time to time. This can cause `AUTO-MAS.exe` or other script executables to disappear unexpectedly. Make sure the directories above are excluded in **Windows Defender**.
:::

## Initialization

When AUTO-MAS starts, it automatically initializes software dependencies and updates backend code.

If `Settings -> Update Configuration -> Automatically check for updates` is enabled, AUTO-MAS will check dependencies and update the backend every time it starts.

Backend updates allow AUTO-MAS to deliver some fixes quickly. Restart the app to receive these fixes as soon as possible.

## Configure the App

Most AUTO-MAS pages include inline notes. You can follow the hints inside the app to complete configuration. After going through every page once, you will likely have finished most setup. If you run into configuration issues, return to the relevant documentation page.

- **Configuration backup**: AUTO-MAS stores configuration files in the `data` and `config` folders under the app installation root, and stores history in the `history` folder. Back up these folders to restore configuration data later.
- **Runtime logs**: AUTO-MAS stores backend logs in `debug/app.log` and frontend logs in `debug/frontend.log` under the app installation root. Providing these logs helps developers locate issues quickly.
