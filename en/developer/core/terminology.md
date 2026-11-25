# Terminology

Familiarize yourself with these core AUTO-MAS terms before diving into the codebase to avoid misunderstandings when reading docs or collaborating.

## Script Terms

### Managed Script | Script

The top-level unit AUTO-MAS uses to manage and schedule an external automation program. Each managed script maps to one external program and organizes all of its configuration and runtime metadata.

A managed script contains one or more accounts.

### Account | User

AUTO-MAS can store multiple configurations for the same script, and an **account** represents one such configuration. Because each configuration typically corresponds to a single player or game account, it is called an account for clarity.

Each account maps to one script configuration and has no child units.

### Plan Table | Plan

A reusable configuration schedule that rotates settings over time. In practice, accounts often need different settings by day of the week, or multiple accounts need the same setting updated together. Binding accounts to a plan table avoids repetitive manual edits.

Each account can bind to at most one plan table, while a single plan table can be shared across multiple accounts.

## Configuration Terms

AUTO-MAS saves account settings by managing the external script's configuration files directly. While this makes it easy to support almost any external program, editing full configurations still requires opening the external tool's own UI. To streamline frequent changes, AUTO-MAS uses a two-layer **Base + Overlay** model.

### Base Layer / Configuration Source | Base

Provides the full underlying configuration where most settings come from. The base layer is edited in the external program's own interface and comes in three storage modes:

- **Shared | shared**

  Stored at `data/<managed script UUID>/Default` as a script-wide copy. Edits made directly inside the external program do not affect this copy, and all accounts under the managed script can share it as their base layer.

- **Independent | independent**

  Stored at `data/<managed script UUID>/<account UUID>` as a private copy for a single account. Edits made directly inside the external program do not affect this copy.

- **Native | native**

  Uses the configuration files inside the external program directly without copying them. Accounts using native mode automatically pick up changes made directly in the external program.

### Overlay

While the base layer supplies the bulk of the configuration, the overlay holds frequently changed settings and can be edited directly inside the AUTO-MAS UI. It supports two override levels:

- **Override key settings**

  Overrides only critical settings required for automated execution (for example, auto-running on launch).

- **Override regular settings**

  Overrides frequently adjusted gameplay or task settings (for example, stage selection).

## Game Terms

### Game

A managed runtime environment in AUTO-MAS, such as an Android emulator application or a Windows game client.

One game environment can contain multiple devices.

### Device

The concrete runtime target selected by a task. For an Android emulator, a device is a specific emulator instance; for a Windows game launcher, a device is a specific game target.

Each device belongs to one game. A managed script or account can bind to one device, and multiple managed scripts or accounts can share the same device.
