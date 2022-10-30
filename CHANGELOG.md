<!-- markdownlint-disable MD012 MD024 -->

# 0.0.2

## `cataas`

- Added `ClientBuilder`.
  - Added `Client::builder`.
  - Made `Client::user_agent` configurable (defaults to `cataas-rs/{CARGO_PKG_VERSION}`).
- Fixed `Client::tags` request path.
- Relaxed type requirements of `Cat::tag`, `Say::tag` and `Say::color` (used to require `String`,
  now require `impl Into<String>`).
- Added `FromStr` implementation for `ImageType` and `Filter`.
  - Added `ParseImageTypeError` and `ParseFilterError` error types.
- Added `with_*` variants for `Cat` and `Says` parameters.

## `cataacli`

- Initial release.

# 0.0.1

## `cataas`

Initial release.
