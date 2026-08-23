# Script Management

AUTO-MAS supports managing and scheduling multiple game scripts. This section explains how to use different scripts in AUTO-MAS.

## Guide Index

### [MAA](/en/docs/script-guide/maa)

Arknights - MaaAssistantArknights

- Supports all Annihilation and daily automation tasks
- Supports multi-account management for official and Bilibili servers
- Supports stage configuration through weekly plans

### [MaaEnd](/en/docs/script-guide/maaend)

Arknights: Endfield - MaaEnd Assistant

- Supports quick Protocol Space adjustments and automatic check-ins
- Supports PC and emulator control, with PC recommended
- Supports stage configuration through weekly plans, currently in beta

---

### [M9A](/en/docs/script-guide/m9a)

Reverse: 1999 - M9A

- Supports daily automation, event farming, automatic Artificial Somnambulism, and more
- Supports MuMu and LDPlayer emulators
- Supports only the MFAAvalonia UI

---

### [OK-WW](/en/docs/script-guide/okww)

Wuthering Waves - OK-WW

- Wuthering Waves project in the ok-script family, configured separately from OkNte for Neverness to Everness
- The MAS entry currently handles DailyTask and MultiAccountDailyTask (`-t 1` and `-t 7`)
- Supports full automatic game lifecycle management in MAS, including startup and shutdown
- Provides Script, User, and Direct control sources, with optional task configuration takeover for high-frequency fields
- Uses only official Wuthering Waves resources and launcher; WeGame is not supported

---

### [HSR](/en/docs/script-guide/hsr)

Honkai: Star Rail - HSR specialization, with M7A and SRA dual engines

- Supports both March7thAssistant (M7A) and StarRailAssistant (SRA)
- Covers daily Trailblaze Power consumption, reward collection, Divergent Universe, Currency Wars, and more
- Allows the Trailblaze Power, reward, divergent, and currency modules to independently choose M7A or SRA
- Automatically retries failed tasks and avoids polluting external script configuration

---

### [BetterGI](/en/docs/script-guide/bettergi)

Genshin Impact - BetterGI

- Native GUI direct control; accounts are managed natively by BetterGI, complex settings are done in BetterGI's interface
- Manages the One Dragon built-in config groups (8 toggles) and custom config groups independently per user
- Supports reward party, combat party, and auto-battle strategy fields, auto-reading combat scripts under `User/AutoFight`
- Account switching, notifications, and extra scripts can each be configured independently

---

### [General Scheduling](/en/docs/script-guide/general)

For scripts that can run tasks on startup and print logs

- Supports most mainstream scripts, including March7thAssistant, SRC, zzzOD, and M9A
- Provides ready-made configuration templates for quick setup
- Supports flexible custom script management plans

---

### [March7thAssistant](/en/docs/script-guide/march7th)

Honkai: Star Rail - March7thAssistant

- Under development
- Can be used together with automatic login scripts

---

## Reading Recommendations

- **New users**: start with the [MAA guide](/en/docs/script-guide/maa) if you play Arknights
- **Wuthering Waves players**: read the [OK-WW guide](/en/docs/script-guide/okww)
- **Reverse: 1999 players**: read the [M9A guide](/en/docs/script-guide/m9a)
- **Honkai: Star Rail players**: read the [HSR guide](/en/docs/script-guide/hsr)
- **Genshin Impact players**: read the [BetterGI guide](/en/docs/script-guide/bettergi)
- **Other games**: read [General Scheduling](/en/docs/script-guide/general) and use an existing template
- **Advanced users**: learn the configuration management model in [General Scheduling](/en/docs/script-guide/general) and customize your scheduling plan
