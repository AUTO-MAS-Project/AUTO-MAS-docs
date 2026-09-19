# Configuration Semantics

The AUTO-MAS configuration system consists of two layers: **base** and **overlay**. The effective configuration of a running task is the result of stacking these two layers. This document defines the standard terminology for both layers and the exact meaning of each configuration source.

The four historical terms "script level / user level / direct control / quick configuration" are being unified according to this document. New code, copy, and documentation must use the terminology defined here.

## Runtime Configuration Model

```
Effective configuration during a task = base ⊕ overlay
                                          (overlay takes precedence)

┌─ overlay layer ── exists during the task · highest precedence · restored after ─┐
└─────────────────────────────────────────────────────────────────────────────────┘
┌─ base layer ───── persisted on disk · one of three sources ─────────────────────┐
│          Shared        │     Independent     │         Native                   │
└─────────────────────────────────────────────────────────────────────────────────┘
```

## base: The Base Configuration

**base** is the main configuration of a running task. It is persisted on disk and lives across tasks. Its source is one of the following three states.

| Term | Former name | Definition | Disk owner |
| --- | --- | --- | --- |
| **Shared** | Script level | base is stored on the entry side; all accounts under that entry share one base | Entry |
| **Independent** | User level | base is stored in the account directory; owned by a single account | Account |
| **Native** | Direct control | base is the external script's own configuration; MAS neither maintains nor mirrors it | External script |

### Common Boundary of the Three States

The three states only determine the **disk owner** of base. They do not guarantee that the effective runtime configuration is identical: every account may enable its own overlay, so the configuration actually in effect during a task differs per account.

"Shared" therefore only describes the multi-account sharing of the base file. It **does not** mean "change once, effective for everyone" — if other accounts have enabled their own overlays, their runtime configuration will not be fully overwritten by that change.

## overlay: The Overlay Layer

**overlay** is a thin configuration layer stacked on top of base during task execution.

- **Precedence**: higher than base; it overrides the corresponding base fields.
- **Does not rewrite the source**: although it is bound to the account settings page, it does not modify the configuration source (the three base states).
- **Restored afterwards**: it overwrites base before the task and correctly restores the original base afterwards; the snapshot mechanism covers success, failure, cancellation, timeout, exception, and crash paths.
- **Composable**: it can be combined with any base source, forming a 3×2 matrix.

| base source | Without overlay | With overlay |
| --- | --- | --- |
| Shared | Use the shared base directly | Shared base ＋ account overlay |
| Independent | Use the independent base directly | Independent base ＋ account overlay |
| Native | Use the script's native configuration | Native configuration ＋ MAS-managed field overrides |

::: warning Terminology pending

The following terms are not finalized. Code, copy, and documentation keep their current names for now; **do not add new semantics based on the current names**. They will be replaced once finalized.

1. **Script entry name** (the entry created by "New Script" in MAS): candidates "Managed" and "Instance".
2. **Account entry name** (the game account under an entry, formerly "User"): candidate "Account".
3. **Quick configuration panel name**: candidates "Advanced" and "Priority".
4. **overlay translation**: candidates "Overlay" and "Priority layer"; whether it should unify with item 3 under "Priority" is undecided.

:::

## Terminology Replacement Table

| Current term | Target term | Language usage (current → target) |
| --- | --- | --- |
| Script level | Shared | "Use the script-level shared configuration, shared by all users" → "Use the shared base configuration, shared by all accounts under the entry" |
| User level | Independent | "Configure user-level MaaEnd" → "Configure the independent base configuration" |
| Direct control | Native | "Script direct control uses the configuration currently saved by SRA" → "Native configuration uses the configuration currently saved by SRA" |
| Script direct control | (removed entirely) | Once direct control is renamed to native, this phrase is dropped and uniformly expressed as "native configuration" |
| Quick configuration | Pending (Advanced / Priority) | "Use the high-frequency task fields in the quick configuration panel below to override the current script configuration" → "Use the high-frequency task fields in the 〔pending〕 panel below to override the current script configuration" |
| Script (MAS entry) | Pending (Managed / Instance) | "New general script" → "New general 〔pending〕" |
| User (MAS entry) | Pending (Account) | "New user" → "New 〔pending〕" |
| overlay / base (developer terms) | Pending / Base layer | "The overlay layer directly overrides the base layer configuration" → "The 〔pending〕 layer directly overrides the base layer configuration items" |

### Rejected Candidates

| Candidate | Conflict |
| --- | --- |
| Instance | The zzz-od upstream concept "instance" means account (an upstream private concept that cannot be changed) |
| Scheme | MAA's internal configuration scheme |
| Project | MaaFW's `interface.json` project |
| Config group | BetterGI's configuration group |

## The Special Case of zzz-od

zzz-od cannot be singularized under the general model: it has a **single entry (single program) with multiple accounts** structure, and the "account entry" and the "upstream instance (= account)" are two distinct concepts — each entry allows only one native-source account, and multiple accounts are configured in that account's instance management.

Therefore:

- If "Instance" is used as the script entry name, it conflicts with the zzz-od upstream "instance" (= account).
- The physical location of the three base states in zzz-od differs from standard adapters (the instance views all live in one script-level configuration).
- zzz-od terminology must be discussed separately and **does not follow the general conclusions of this document**.

## Scope of Change

Once the terminology is finalized, the following must be updated:

- Frontend UI copy and i18n (`frontend/src/locales/zh-CN.ts`)
- Backend schema descriptions and configuration comments (`app/models/schema.py`, `app/models/config.py`)
- Frontend OpenAPI generated type comments (**never edit by hand**; regenerate instead)
- The `Info.Mode` stored values (`"脚本"` / `"用户"` / `"直控"`): renaming them requires a compatibility migration (there is precedent in the "简洁/详细/自定义" → "脚本/用户/直控" migration)
- Hard-coded comparisons in adapters (such as `== "直控"`)
- This document and the repository glossary

## Related Discussion

- [AUTO-MAS#879](https://github.com/AUTO-MAS-Project/AUTO-MAS/issues/879): terminology unification proposal and vote
