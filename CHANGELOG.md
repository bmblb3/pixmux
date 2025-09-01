# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.2.10](https://github.com/bmblb3/autofoam/compare/v0.2.9...v0.2.10) - 2025-08-27

### Other

- fix the CD actions so that the built binaries are actually uploaded to github releases

## [0.2.9](https://github.com/bmblb3/autofoam/compare/v0.2.8...v0.2.9) - 2025-08-27

### Other

- Add info downloading pre-built binaries to README
- simplify CI/CD
    - do the verison-bumping in the local repo
    - pushing tags triggers the CD pipeline that builds and publishes

## [0.2.8](https://github.com/bmblb3/autofoam/compare/v0.2.7...v0.2.8) - 2025-08-26

### Other

- add pre-commit config

## [0.2.7](https://github.com/bmblb3/autofoam/compare/v0.2.6...v0.2.7) - 2025-08-25

### Fixed

- **CI/CD**: musl build config

## [0.2.6](https://github.com/bmblb3/autofoam/compare/v0.2.5...v0.2.6) - 2025-08-25

### Added

- **CI/CD**: Compile with musl target for truly static builds
             Rename the build tarfile asset accordingly. (faulty implementation, fixed later)

### Other

- update CHANGELOG

## [0.2.5](https://github.com/bmblb3/autofoam/compare/v0.2.4...v0.2.5) - 2025-08-25

### Fixed

- **CI/CD**: binaries are now autodiscovered, and are available in the top-level dir of the tar file

## [0.2.4](https://github.com/bmblb3/autofoam/compare/v0.2.3...v0.2.4) - 2025-08-25

### Fixed

- **CI/CD**: Now, when a PR is merged to master, the CD pipeline autobumps the version and creates a PR.
             On merging this new PR, the CD pipeline builds the binaries and publishes the updated packages.

## [0.2.3](https://github.com/bmblb3/autofoam/compare/v0.2.2...v0.2.3) - 2025-08-25

### Fixed

- **CI/CD**: Release asset creation and tagging

## [0.2.2](https://github.com/bmblb3/autofoam/compare/v0.2.1...v0.2.2) - 2025-08-25

### Added

- **Binary**: Scalar deviation CLI tool for VTP files (`autofoam-scalar-deviation`)
- **Documentation**: Simple description to README

### Changed

- **CI/CD**: Simplify GitHub workflow to use release_plz (faulty implementation, fixed in later versions)
- **CI/CD**: Build assets only for linux-x86_64
- **Library**: Refactor `autofoam-scalar-area-threshold` into libraries with tests

## [0.2.1](https://github.com/bmblb3/autofoam/compare/v0.2.0...v0.2.1) - 2025-08-20

### Fixed

- **CD/CD**: Build assets for both binaries: `autofoam-scalar-area-threshold` and `autofoam-stl-bbox`

## [0.2.0](https://github.com/bmblb3/autofoam/compare/v0.1.0...v0.2.0) - 2025-08-20

### Added

- **Binary**: `autofoam-scalar-area-threshold`
- **Binary**: `autofoam-stl-bbox`
- **Library**: `stl` - STL file operations
- **Library**: `vtk` - VTK file operations
- **Library**: `coordinates` - Operations for `[f32; 3]` coordinates

## [0.1.0](https://github.com/bmblb3/autofoam/commits/v0.1.0) - 2025-08-20

### Added

- Initial crate release
