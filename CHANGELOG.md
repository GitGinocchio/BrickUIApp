# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.1] - 2026-04-12

### 🚀 Features

- Add release workflow and Release.toml, remove rust.yml
- Add CI workflow, changelog config, and update release process
- [**breaking**] Register, login and user pages ([#65](https://github.com/BrickUIApp/BrickUIApp/pull/65))
- Changed release-plz behavior

### 🐛 Bug Fixes

- Topbar + folder structure + get_taskbar_apps (not working)
- Remove build step from release workflow
- Add cargo-release and update dependencies
- Rename releaze-plz.toml to release-plz.toml
- Disabled publishing ([#131](https://github.com/BrickUIApp/BrickUIApp/pull/131))
- *(ci/cd)* Test new config ([#132](https://github.com/BrickUIApp/BrickUIApp/pull/132))
- Release-plz
- Cargo.toml version

### 🚜 Refactor

- Update release config and add CHANGELOG.md
- Move cliff.toml and update Cargo.toml excludes
- Update pre-release hook to add CHANGELOG.md
- Downgrade brickui version and update release hook
