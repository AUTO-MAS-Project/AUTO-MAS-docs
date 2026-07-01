# Configuration Management

The AUTO-MAS backend uses a specialized configuration management module. It supports automatic correction of configuration fields and automatic synchronization to files.

## Related Code

### Configuration Base Class

- Location: `app/models/ConfigBase.py`
- Purpose: defines the basic methods of the configuration management module and provides multiple automatic configuration validators.

### Configuration Template

- Location: `app/models/config.py`
- Purpose: defines configuration fields actually used by the program, including names, types, default values, validators, descriptions, and hierarchy.

### Configuration Management Module

- Location: `app/core/config.py`
- Purpose: provides the global variable `Config` as the entry point for accessing program configuration.

## Configuration Fields

A **configuration field** is an instance of `ConfigItem`. It defines the group, name, default value, and validation method of a configuration field.

### Define a Configuration Field

```python
Group_Name = ConfigItem("Group", "Name", "default value", [Validator()])
```

**Meaning:** defines a configuration field named `Name`, belonging to group `Group`, with default value `"default value"`, validated and corrected by `Validator()`.

### Configuration Field Validators

After a validator is defined, the configuration management module automatically calls it to validate and correct the field. Validators may also be used for special field logic.

**Existing validators:**

1. Basic configuration validator
   - Definition: `ConfigValidator()`
   - Purpose: validator base class and default configuration validator. It does not validate the field.

2. Range validator
   - Definition: `RangeValidator(min, max)`
   - Purpose: checks whether a number is within the specified range. If not, it is corrected to the minimum or maximum.

3. Options validator
   - Definition: `OptionsValidator(options)`
   - Purpose: checks whether the field value is in a list of options. If not, it is corrected to the first option.

4. UUID format validator
   - Definition: `UUIDValidator()`
   - Purpose: checks whether the field value is a valid UUID. If not, it is corrected to a random UUID.

5. Date/time format validator
   - Definition: `DateTimeValidator(date_format)`
   - Purpose: checks whether the field value matches the specified date/time format. If not, it is corrected to the default time in that format.

6. JSON format validator
   - Definition: `JSONValidator(type)`
   - Purpose: checks whether the field value can be parsed as a dictionary or list in JSON format. If not, it is corrected to an empty dictionary or empty list.

7. Encrypted data validator
   - Definition: `EncryptValidator()`
   - Purpose: checks whether the field value can be decrypted by DPAPI. If not, it is corrected to `"Data corrupted, please reset"`.

8. Virtual configuration field validator
   - Definition: `VirtualConfigValidator(function)`
   - Purpose: defines a virtual configuration field. The value of a virtual field cannot be set. When the field value is read, `function` is called and its return value is returned. It is similar to Python's `@property`, allowing code to access a function result through the configuration item interface.

9. Boolean validator
   - Definition: `BoolValidator()`
   - Purpose: checks whether the field value is a boolean. If not, it is corrected to `True`.

10. File/folder path validator
    - Definition: `FileValidator()` / `FolderValidator`
    - Purpose: checks whether the field value is a valid absolute path and not a shortcut. If possible, it is corrected to the corresponding absolute path; otherwise it resets to the current working directory.

11. Username validator
    - Definition: `UserNameValidator()`
    - Purpose: checks whether a username is valid. It must be a non-empty string, contain no illegal characters, not be a reserved name, be at most 255 characters, and not start or end with spaces or dots. Invalid values are corrected to `"Default username"`.

12. Multi-configuration UID validator
    - Definition: `MultipleUIDValidator(default, related_config, config_name)`
    - Purpose: checks whether the field value is the UID of a configuration in a specified **multi-configuration management instance**. If not, it is corrected to the default value.

## Single Configuration Templates

A **single configuration template** is a class inherited from `ConfigBase`. It defines which **configuration fields** and **multi-configuration management instances** should exist under a configuration variable. After instantiation, it becomes a **single configuration management instance** and can be used for actual configuration management.

### Define a Single Configuration Template

```python
class ConfigModel(ConfigBase):

    Group_Name = ConfigItem("Group", "Name", "default value", [Validator()])
    MultiConfig = MultipleConfig([ConfigModel]) # Multi-configuration management instance
    ... # Class attributes

    def __init__(self) -> None:

        self.Group_Name = ConfigItem("Group", "Name", "default value", [Validator()])
        self.MultiConfig = MultipleConfig([ConfigModel]) # Multi-configuration management instance
        ... # Instance attributes
        super().__init__()
```

**Class attributes and instance attributes**

- If a configuration item is defined as a class attribute, all instances share the same configuration item data.
- If a configuration item is defined as an instance attribute, each instance has independent configuration item data.

**Where to call `super().__init__()`**

- Configuration items defined before `super().__init__()` are treated as configuration fields of the **single configuration management instance**.
- Configuration items defined after `super().__init__()` are not treated as configuration fields of the **single configuration management instance**. They cannot be accessed through methods such as `set()` and will not be saved to configuration files.

### Use a Single Configuration Management Instance

Read the related function definitions and comments in the program.

## Multi-Configuration Management Instances

A **multi-configuration management instance** is an instance of `MultipleConfig`. It has characteristics of a **dictionary**, a **single configuration management instance**, and a **configuration field**. You can access **single configuration management instance** values by `UID` like a dictionary, bind the instance to a configuration file and save it like a **single configuration management instance**, or use it as an attribute of a **single configuration management instance** like a **configuration field**.

### Define a Multi-Configuration Management Instance

```python
MultiConfig = MultipleConfig([ConfigModel_1, ConfigModel_2, ...])
```

Similar to `List[type]`, defining a **multi-configuration management instance** requires specifying possible **single configuration management instance** types as its values.

### Use a Multi-Configuration Management Instance

Read the related function definitions and comments in the program.
