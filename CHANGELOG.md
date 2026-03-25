# Changelog

## [0.5.2] - 2026-03-25

### Fixes
- `dashboards deploy`: resolve `visualization_id` from `query_id` + `visualization_name` so query-backed widgets are no longer created as empty widgets

## [0.5.1] - 2026-03-24

### Fixes
- `deploy`: fix visualizations with explicit IDs being overwritten when adding a new visualization of the same type

## [0.5.0] - 2026-03-18

### Features
- Track `Cargo.lock` for reproducible binary builds
- `execute`: output a plain JSON rows array; progress messages go to stderr for clean piping; no default row limit
- `deploy`: use `null` visualization id to signal new visualizations (replaces id=0 convention; existing YAMLs with `id: 0` still work)
- `deploy`: no longer requires git — falls back gracefully when git is unavailable or outside a repo

### Fixes
- Add `width` field to `Widget` and `WidgetMetadata` for correct API/YAML round-trips
- Fix `Widget.text` deserialization when API returns `null`
- Fix `dashboards archive` — use POST by ID instead of DELETE (DELETE returns 500)

## [0.4.1] - 2026-03-04

### Fixes
- Fix text widget creation by serializing `visualization_id` as `null` instead of omitting it

## [0.4.0] - 2026-03-03

### Features
- Add `--version` flag to CLI
- Add automatic update check on invocation (checks crates.io, cached for 24h)
- Add `update` subcommand to update stmo-cli via `cargo install`
- Sort visualizations by ID and rewrite YAML on deploy

## [0.3.0] - 2026-02-27

### Features
- Auto-populate `parameterMappings` with `type: dashboard-level` for new widgets during dashboard deploy
- Auto-enable `dashboard_filters_enabled` when any new widget has parameters

## [0.2.0] - 2026-02-27

### Features
- Reuse auto-created visualizations when deploying new queries
- Auto-favorite dashboards after creating with id: 0
- Auto-rename query files after deploying with id: 0

### Fixes
- Fix `archive_dashboard` to use slug instead of ID
- Fix widget creation: add required `width` and `text` fields
- Include response body in API error messages

### Docs
- Improve deploy docs: default behavior and commit ordering
- Add `cargo install stmo-cli` to template Quick Reference
- Update installation docs to use `cargo install stmo-cli`

## [0.1.0] - 2026-02-26

Initial release.
