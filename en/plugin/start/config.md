# Configuration File

::: warning Note

The structure and declaration method of configuration files may change in the future. Check whether this document is still up to date.

The plugin system configuration file is independent from other main app configuration files. The configuration file mentioned here refers only to the plugin system configuration file.

*TOML may be used in the future.*

:::

The plugin system configuration file is located at:

```text
AUTO-MAS/config/PluginConfig.json
```

Because readability may be significantly redesigned later, this document does not describe the file format in detail for now. This does not affect plugin development. Plugin developers do not need to care how plugin configuration is saved. You only need to declare what configuration your plugin needs, and MAS handles saving and reading configuration data.
