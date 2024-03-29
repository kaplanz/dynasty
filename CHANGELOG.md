# Changelog

All notable changes to this project will be documented in this file.

## [v0.1.0]

### Bug Fixes

- Trim resolver output

### Documentation

- Initial document
- Add install from releases
- Add badges

### Features

- Initial impl
- Split mode args
- Update default config path
- Report error on implemented options
- Add parse defaults
- Parse timeout as `std::time::Duration`
- Re-impl error reporting
- Improve logging
- Perform async requests
- Impl `--dry-run`
- Impl `--daemon`
- Lex resolver command
- Impl `--verbose`

### Miscellaneous Tasks

- Initial release
- Update .gitignore
- Create rust.yml
- Init
- Cargo dist init
- Ordana generate ci
- Disable ordana on push to main
- Deploy with actions

### Refactor

- Update terminology `{dns => addr}`
- Inline `update()`
- Explicitly match on `io::ErrorKind`
- Extract `resolve()`

### Build

- Init
- Initial impl
- Add reqwest feature rustls-tls
- Add repository key

<!-- generated by git-cliff -->
