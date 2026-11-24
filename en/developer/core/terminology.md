# Terminology

These terms have specific meanings in AUTO-MAS. Use them consistently when discussing the code and its behavior.

## Script Terms

### Managed Script | Script

A managed script is the unit that AUTO-MAS uses to manage and schedule external automation software. Each managed script corresponds to one external program and organizes its configuration.

A managed script contains one or more accounts.

### Account | User

AUTO-MAS saves multiple configurations for each script. An account represents one such configuration. Since each configuration usually corresponds to a user or login, the project calls it an account.

An account corresponds to one script configuration and has no child units.

### Plan Table | Plan

A plan table is a form that rotates settings on a schedule. Some accounts need different settings on different days of the week. Several accounts may also need to change the same setting together. Linking accounts to a plan table simplifies those changes.

Each account can link to one plan table. Multiple accounts can share the same table.

## Configuration Terms

AUTO-MAS stores script configuration files directly. This lets it manage many external programs, although some script settings still need to be edited in the program's own interface. AUTO-MAS uses a base and an overlay to make frequent edits easier.

### Base Layer / Configuration Source | Base

The base holds the full underlying configuration. Edit it using the external program's own interface. The three sources differ by where the files are stored.

- **Shared | shared**

  Files in `data/<managed script UUID>/Default` hold the managed script's shared configuration. Changes made directly in the external program do not update this stored copy. All accounts under that managed script can use it as their base.

- **Independent | independent**

  Files in `data/<managed script UUID>/<account UUID>` hold one account's separate configuration. Changes made directly in the external program do not update this stored copy. That account has exclusive use of the copy.

- **Native | native**

  The files stay in the external program. Accounts using this source automatically pick up changes made directly in that program.

### Overlay

The base supplies most settings. The overlay supplies settings that are changed frequently and can be edited in the AUTO-MAS interface. It offers two levels.

- **Override key settings**

  Override settings that determine whether the program can run correctly, such as automatically starting a run on launch.

- **Override regular settings**

  Override frequently edited settings such as stage selection.

## Game Terms

### Game

A game is a managed runtime environment, such as an Android emulator or a Windows game client.

A game can contain multiple devices.

### Device

A device is the specific target selected for a task. For an Android emulator, it is an emulator instance. For a Windows game launcher, it is an individual game target.

Each device belongs to one game. A managed script or account can link to one device, and several managed scripts or accounts can share the same device.
