# Plan Table Specification

## 1. Responsibilities

The plan table framework handles dates only and has no knowledge of specialized business logic.
It returns the complete key without splitting it into fields or making business decisions based on its contents. A key may be a string or a deeply nested object.

## 2. General Plan Table Structure

The backend type of a plan table is `XXXPlanConfig`. A plan table contains basic information and eight date slots:

```json
{
  "Info": {
    "Name": "Weekly Material Plan",
    "Mode": "Weekly"
  },
  "ALL": {
    "Key": {}
  },
  "Monday": {
    "Key": {}
  },
  "Tuesday": {
    "Key": {}
  },
  "Wednesday": {
    "Key": {}
  },
  "Thursday": {
    "Key": {}
  },
  "Friday": {
    "Key": {}
  },
  "Saturday": {
    "Key": {}
  },
  "Sunday": {
    "Key": {}
  }
}
```

Each date slot has only one framework-defined field: `Key`. The framework stores and returns it as opaque data.

The `Info` fields are as follows:

| Field | Allowed values | Default | Description |
| --- | --- | --- | --- |
| `Name` | String | `New MaaEnd Plan Table` | Plan table name |
| `Mode` | `ALL`, `Weekly` | `ALL` | Global mode or weekly plan mode |

## 3. Maintenance Locations

| Scope | Location | Purpose |
| --- | --- | --- |
| General date-key container | `WeeklyKeyPlanConfig` in `app/models/config.py` | Stores slots and returns the key for a given date |
| MaaEnd API contract | `app/models/schema.py` | Defines the two nested key structures and enum constraints |
| MaaEnd key conversion | `app/models/config.py` | Converts fixed configurations and validates them before execution |
| MaaEnd task injection | `app/task/maaend/AutoProxy.py` | Interprets the key and modifies MaaEnd tasks |
| Frontend key utilities | `frontend/src/utils/maaEndProtocolSpace.ts` | Edits, displays, normalizes, and provides backward compatibility for legacy structures |
| Plan table UI | `frontend/src/views/plan/tables/MaaEndPlanTable.vue` | Edits the MaaEnd key associated with each date |

Each specialized integration must own its key schema, editing UI, and consumption logic. Do not add specialized fields back to the plan table framework or make the framework dispatch business logic based on key contents.
