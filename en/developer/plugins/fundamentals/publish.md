# Packaging and Publishing

Plugin developers upload distributions to PyPI so users can discover and install them from the plugin market. Follow the release metadata requirements below so the index and host can identify your distribution.

## Prepare a Release

Create `pyproject.toml` and `README.md` in the plugin project root. Fill in the placeholders in the following templates.

### Python Project Configuration

This template uses Setuptools to build the distribution, discover the import package under `src/`, and include `README.md` in the distribution metadata.

```toml
[build-system]
requires = ["setuptools>=77", "wheel"]
build-backend = "setuptools.build_meta"

[project]
name = "<distribution name>"
version = "<release version>"
description = "<short distribution description>"
readme = { file = "README.md", content-type = "text/markdown" }
requires-python = "<Python version range>"
license = "<SPDX license identifier>"
license-files = ["<license file path>"]
authors = [{ name = "<author name>" }]
keywords = ["auto-mas-plugin", "auto-mas-id:<plugin ID>"]
dependencies = ["auto-mas-core<SDK version range>", "<optional runtime dependency>"]

[project.urls]
Homepage = "<project homepage URL>"
Repository = "<source repository URL>"
Documentation = "<plugin documentation URL>"
Issues = "<issue tracker URL>"

[tool.setuptools.packages.find]
where = ["src"]
include = ["<import package name>", "<import package name>.*"]

[tool.setuptools.package-data]
"<import package name>" = ["<package data pattern>"]
```

- `<distribution name>` is a valid PyPI project name. Automatic discovery requires the normalized name to contain `automas` or `auto-mas`. Use `auto-mas-<plugin name>` as a naming convention.
- `keywords` identifies plugin distributions.
  - Include `auto-mas-plugin`.
  - Include exactly one `auto-mas-id:<plugin ID>` keyword. The plugin ID must be valid as a filename.
- `<release version>` is a PEP 440 version, such as `1.2.0`.
- `<Python version range>` must match the target host's Python version. The recommended range is `>=3.12,<3.13`.
- `<SDK version range>` is the SDK version constraint you have verified with your plugin. The recommended range is `>=5,<6`.
- `[project.urls]` is optional. Include links your project provides.
- `<import package name>` matches the top-level directory under `src/` that contains `__init__.py`.
- `<package data pattern>` is a path or pattern relative to the import package, such as `i18n/*.json`. You can list multiple patterns. Omit `[tool.setuptools.package-data]` if your package needs no additional resources.

### README

Declare localized display metadata in an `auto-mas:i18n` block at the start of `README.md`. It is published as part of the PyPI project description.

```markdown
<!-- auto-mas:i18n
{
  "display_name_i18n": {
    "<language key>": "<market display name>"
  },
  "description_i18n": {
    "<language key>": "<market description>"
  },
  "tags_i18n": {
    "<language key>": ["<market tag>"]
  }
}
-->

# <Plugin name>

<Plugin purpose and main features>

## Requirements

- AUTO-MAS: <supported host versions>
- <External software or other runtime requirements>

## Installation and Usage

1. Install and enable "<market display name>" in the AUTO-MAS Plugin Center.
2. <Where to open the plugin and what to configure>
3. <Steps for using the plugin's main feature>

## Report Issues

<Where to report issues and what information to include>
```

## Build and Check the Distribution

1. **Build the distribution.** Run the following commands from your plugin repository root with the target Python version to build a wheel and source distribution and check the metadata.

   ```powershell
   python -m pip install build twine
   python -m build
   python -m twine check --strict dist/*
   ```

2. **Inspect the wheel.** Open the wheel in `dist/` and confirm that the top-level package entry point exports `PLUGIN` and that the required runtime resources are included. Check the version, dependencies, identity keywords, and description in `.dist-info/METADATA` against the release metadata.

## Publish to PyPI

This GitHub Actions workflow runs when you push a `v<release version>` tag. The build job checks the tag against `project.version`, builds a wheel and source distribution, and checks the resulting files. The publishing job downloads the build artifacts and uploads them to PyPI through Trusted Publishing.

### Workflow Template

```yaml
name: Build and publish to PyPI

on:
  push:
    tags: ["v*"]

permissions:
  contents: read

jobs:
  build:
    name: Build and validate
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@11d5960a326750d5838078e36cf38b85af677262
        with:
          persist-credentials: false
      - uses: actions/setup-python@a26af69be951a213d495a4c3e4e4022e16d87065
        with:
          python-version: "<build Python version>"
      - name: Check the release tag
        env:
          RELEASE_TAG: ${{ github.ref_name }}
        run: |
          python - <<'PY'
          import os
          import tomllib
          from pathlib import Path

          project = tomllib.loads(Path("pyproject.toml").read_text(encoding="utf-8"))["project"]
          expected_tag = f"v{project['version']}"
          if os.environ["RELEASE_TAG"] != expected_tag:
              raise SystemExit(f"Expected release tag {expected_tag}")
          PY
      - name: Build and check distributions
        run: |
          python -m pip install build==1.6.1 twine==7.0.0
          python -m build
          python -m twine check --strict dist/*
      - uses: actions/upload-artifact@ea165f8d65b6e75b540449e92b4886f43607fa02
        with:
          name: python-distributions
          path: dist/
          if-no-files-found: error

  publish:
    name: Publish to PyPI
    needs: build
    runs-on: ubuntu-latest
    environment:
      name: "<GitHub publishing environment name>"
      url: "https://pypi.org/project/<distribution name>/"
    permissions:
      id-token: write
    steps:
      - uses: actions/download-artifact@d3f86a106a0bac45b974a628896c90dbdf5c8093
        with:
          name: python-distributions
          path: dist/
      - name: Upload with Trusted Publishing
        uses: pypa/gh-action-pypi-publish@7245906fb61fa33a13554f9d5fb856fe61758086
```

- `<build Python version>` matches the plugin's `requires-python` and target host, for example `3.12`.
- `<GitHub publishing environment name>` matches the environment created in your GitHub repository, for example `pypi`.
- `<distribution name>` matches `project.name` in `pyproject.toml`.

### Run the Workflow

1. **Save the workflow.** Replace the placeholders and save the file at `.github/workflows/<workflow filename>` in your plugin repository. In **Settings → Environments**, create the publishing environment named in the template.
2. **Configure a Trusted Publisher.** For an existing PyPI project, open **Manage → Publishing** on the project. For a first release, open **Publishing** in your PyPI account, add a pending publisher, and supply the distribution name. Select GitHub Actions and enter the GitHub repository owner, repository name, workflow filename, and publishing environment name.
3. **Push the release tag.** Check that the current commit contains the workflow and the intended release version. Create and push the matching tag.

   ```powershell
   git tag "v<release version>"
   git push "<plugin repository remote>" "v<release version>"
   ```

4. **Confirm the release.** In GitHub Actions, check the results of **Build and validate** and **Publish to PyPI**. Confirm the version and distribution files on the PyPI project page.

## Plugin Market Discovery

The catalog workflow refreshes listed and manually added projects every 30 minutes. It is scheduled to scan for new candidates daily at 00:07 UTC (08:07 Beijing time) and publishes successful catalog updates to the data center. The client reads that catalog first and searches PyPI directly if the catalog is unavailable. [Catalog workflow](https://github.com/AUTO-MAS-Project/AUTO-MAS-Plugin-Index) · [AUTO-MAS Data Center](https://data.auto-mas.top/files/auto-mas/Client/plugins)

New projects are picked up when discovery runs. After publishing to PyPI, contact the project team to request a manual catalog refresh with discovery enabled if you need the plugin listed sooner.
