# Advanced Features

Once the basics are working, these features make life easier:

- [Emulator Management](./emulator) — register your emulator once and AUTO-MAS fills in the emulator settings for every managed script. Set this up first if you use an emulator.
- [Push Notifications](./notification) — send a message to your email or phone when automation finishes.
- [Failure Replays](./failure-replay) — set up the OBS Replay Buffer and view videos from failed automatic runs in History.
- [Game Check-in](./game-sign) — automatically complete daily check-ins on supported game community platforms.
- [MCP Service](./mcp) — let an AI operate AUTO-MAS for you.
- [Custom Appearance Packages](./appearance) — import a ZIP package to customize colors, backgrounds, sidebar menu icons, and cursors.
- [Desktop Mascot Packages](./mascot) — create standalone mascot packages with idle, working, mouse-tracking, task result, and drag animations.
- [Scheduled Computer Startup](./Timed-Startup) — automatically power on your computer on a schedule.
- [Automatic Windows Login](./skip-password) — skip the Windows lock screen and password prompt at startup.
- [Scheduled and Delayed App Startup](./Timed-app-start) — launch AUTO-MAS on a schedule or after a startup delay.
- [Troubleshooting with the Community Bot](./cnb-bot) — upload log bundles in the AUTO-MAS QQ group for automated bot analysis.

## Reference

### Common Date/Time Format Symbol Reference

You will need this when configuring [General Scheduling](/en/docs/script-guide/general): rewrite the date and time in your log using the symbols below.

| Symbol | Meaning | Example |
|--------|---------|---------|
| `%Y` | Four-digit year | 2025 |
| `%m` | Two-digit month | 07 |
| `%d` | Two-digit day | 16 |
| `%H` | Hour, 24-hour format | 14 |
| `%M` | Minute | 30 |
| `%S` | Second | 45 |
| `%f` | Milliseconds, 3 digits | 123 |
| `%f` | Microseconds, 6 digits | 123456 |
| `%A` | Full weekday name | Wednesday |
| `%a` | Abbreviated weekday name | Wed |
| `%B` | Full month name | July |
| `%b` | Abbreviated month name | Jul |
