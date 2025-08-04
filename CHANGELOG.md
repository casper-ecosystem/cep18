# Changelog

All notable changes to this project will be documented in this file. The format is based on [Keep a Changelog].

[comment]: <> (Added: new features)
[comment]: <> (Changed: changes in existing functionality)
[comment]: <> (Deprecated: soon-to-be removed features)
[comment]: <> (Removed: now removed features)
[comment]: <> (Fixed: any bug fixes)
[comment]: <> (Security: in case of vulnerabilities)

## [Unreleased]

## [2.0.0] - 2025-08-05

### Added

- Native eventing (#145) — Adds native events feature
- Legacy compatibility mode (#154) — Enables migration to the 2.0 network and adds test fixtures

### Changed

- Disable integer sign extensions (#139) — Modifies the build script to omit unsupported WASM opcodes
- Use `DEFAULT_ACCOUNTS` in tests (#146) — Updates tests to rely on `DEFAULT_ACCOUNTS`
- Update JavaScript client for 2.0 (#161) — Updates the JS client for 2.0 network support and adds tests

### Deprecated

_(No entries)_

### Removed

- Remove entry point and `condor` and `casper_2` key (#173) — Removes use of 2.0-specific entry point and named key

### Fixed

- Native expansive error topic (#150) — Prevents `MessageTopics` from committing state changes on failed execution
- Prevent revert on upgrade with same events (#160) — Fixes upgrade failure when events have not changed

### Security

- HAL fixes (#172) — Addresses multiple issues identified during the audit

## [1.2.0] - 2024-04-11

[Keep a Changelog]: https://keepachangelog.com/en/1.0.0/
