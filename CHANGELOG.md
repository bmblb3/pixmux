# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.0](https://github.com/bmblb3/pixmux/releases/tag/v0.1.0) - 2025-09-06

### Added

- New project
- Launch with `pixmux /path/to/csv/file.csv`
- Quit app with `q`
- App has `Data`, `Image` tabs
    - Navigate tabs with `Tab`
- Data tabs loads a csv
    - csv must have atleast two cols.
    - ONE col must have the header "_"
        - The entries of this column must point to immediate parent directory of image files
    - The other columns can be named anything except "_" and must contain data
    - Navigate rows with `Up/Down`
- Image tab is a multiplexer where the following keymaps apply
    - `r`: split a pane to right
    - `b`: split a pane to bottom
    - `x`: remove a pane
    - `h,j,k,l`: navigate panes (current pane has a yellow border)
    - `w,a,s,d`: resize panes (tmux style)
    - `ö,ä`: cycle images in the current pane
    - Navigate rows with `Up/Down` (same as `Data` tab)
