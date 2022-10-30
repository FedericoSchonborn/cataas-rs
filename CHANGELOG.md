# Unreleased

- Added `cataacli` command line.
- Added `ClientBuilder`.
  - Added `Client::builder`.
  - Made `Client::user_agent` configurable (defaults to `cataas-rs/{CARGO_PKG_VERSION}`).
- Fixed `Client::tags` request path.
- Relaxed type requeriments of `Cat::tag`, `Say::tag` and `Say::color` (used to require `String`,
  now require `impl Into<String>`).
- Added `FromStr` implementation for `ImageType` and `Filter`.
  - Added `ParseImageTypeError` and `ParseFilterError` error types.

# 0.0.1

Initial release.
