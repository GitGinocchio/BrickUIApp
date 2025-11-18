# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.0](https://github.com/BrickUIApp/BrickUIApp/releases/tag/v0.1.0) - 2025-11-18

### Added

- Add CI workflow, changelog config, and update release process
- Add release workflow and Release.toml, remove rust.yml

### Fixed

- Add cargo-release and update dependencies
- Remove build step from release workflow
- topbar + folder structure + get_taskbar_apps (not working)

### Other

- Update releaze-plz.toml
- Create releaze-plz.toml
- Downgrade brickui version and update release hook
- Update pre-release hook to add CHANGELOG.md
- Release brickui version 0.2.0
- Move cliff.toml and update Cargo.toml excludes
- Move cliff.toml to src-tauri directory
- Update release config and add CHANGELOG.md
- Refactor Bluetooth API and add Tauri opener/shell plugins
- Partial added Classic Bluetooth support and update dependencies
- Update mod.rs
- Windows automatically resize after workarea changes
- Update mod.rs
- Add async atomic YAML save and improve icon handling
- Refactor to async handlers and update state management
- Update lib.rs
- Add brick hot-reload and refactor loader/error handling
- Update mod.rs
- Update mod.rs
- Update mod.rs
- Remove unused imports and clean up code
- Refactor taskbar and workarea management logic
- Add banner field to Brick struct
- Add monitor and window management APIs, refactor workarea logic
- Refactor monitor and window handling for WinAPI
- Add workarea management for monitors
- Rename load_from_yaml to load_yaml and update usage
- Add cursor backup and restore functionality
- Add brick pack/unpack logic and dialog plugin
- Add Windows taskbar and start menu app support
- Refactor Vue type definitions and add submodules
- Refactor brick loader for improved import handling
- Refactor loader and utils structure, add error handling
- Add wallpaper window and background rendering
- Refactor bricks management UI and add brick modal
- Refactor prop types: unify Array and Select handling
- Refactor system tray logic and improve close behavior
- Add advanced system tray settings and UI improvements
- Update tauri.conf.json
- loader fixes
- System Tray Icons, debouncing, icons and minor changes
- Add tray icon with brick toggling support
- single instance plugin and first api types
- minor changes
- Refactor settings and theme management across app and overlay
- Update Tauri bundle config and vendor loader
- Enable Tauri isolation pattern and update dependencies
- Refactor overlay to remove sandbox iframe and legacy code
- global events handling StartMenu hide feature
- Update lib.rs
- Refactor config, add i18n, and introduce sandbox mode
- added gradient prop type
- Gradient Prop, structure refactoring, minor changes
- now props can be added and edited via UI, minor changes
- first working impl of get_taskbar_icons
- Color prop saved feature, bricks loading improvements
- added some prop types, dinamic brick loader, dir str changes
- UI brickprops
- Update brick.rs
- Backend brick loading, app structs and state changes
- Brick struct, events api
- add brick exemple
- securely exposed tauri api, types and minor changes
- taskbar fix maybe?
- csp, working overlay and first tests
- Settings and Overlay first implementation
- first tests
- Initial commit
