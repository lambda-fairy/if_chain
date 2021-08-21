# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [1.0.2] - 2021-08-21
- Fix "trailing semicolon" warning for `let` statements with multiple patterns
  ([#20](https://github.com/lambda-fairy/if_chain/pull/20))

## [1.0.1] - 2020-09-29
- Update GitHub links from lfairy → lambda-fairy.

## [1.0.0] - 2019-05-22
- **Breaking change:** Update minimum Rust version to 1.12.0.
- Expand `if` and `if let` to their equivalent expressions instead of `match`.
- Work around an edge case regarding Rust's treatment of `block` matchers.

## [0.1.3] - 2018-07-20
- Add `local_inner_macros` attribute for compatibility with new-style macro imports.

## [0.1.2] - 2017-02-17
- Support `no_std`.

[Unreleased]: https://github.com/lfairy/if_chain/compare/v1.0.2...HEAD
[1.0.1]: https://github.com/lfairy/if_chain/compare/v1.0.1...v1.0.2
[1.0.1]: https://github.com/lfairy/if_chain/compare/v1.0.0...v1.0.1
[1.0.0]: https://github.com/lfairy/if_chain/compare/v0.1.3...v1.0.0
[0.1.3]: https://github.com/lfairy/if_chain/compare/v0.1.2...v0.1.3
[0.1.2]: https://github.com/lfairy/if_chain/compare/v0.1.1...v0.1.2
