# Zed Meson Muon Extension

![Muon Version Badge](https://img.shields.io/badge/Muon_LSP-v0.5.0-green)
![license](https://img.shields.io/github/license/jobpaardekooper/zed-meson-muon)
================================================================================

Extension for the [Zed](https://zed.dev) code editor, adding support for the
[Meson](https://mesonbuild.com) build system by using [Muon](https://muon.build/) `v0.5.0` as the LSP.

## Features

- Syntax Highlighting
- Formatting
- Jump-To-Definition
- Jump-To-Subdir
- A basic set of diagnostics
- Basic autocompletion

Muon (the underlying language server) has a [tacking issue regarding its LSP feature set](github.com/muon-build/muon/issues/159). Please contribute to Muon if you find it useful and would like to see a more extensive feature set supported by this extension!

## Language Server

This extension uses [Muon](https://muon.build/) for the language server. [MesonLSP](https://github.com/JCWasmx86/mesonlsp) used to be the standard for meson projects and editor integration, but the project has since been archived due to a lack of maintenance. Muon is actively maintained and can run in LSP mode although it does not yet have all features that used to be available in MesonLSP.

### Language Server Selection

The extension first checks if you have Muon in path and if so, it uses you already installed version. If Muon is not found, the extension automatically downloads the correct version for your architecture and operating system so no extra configuration is needed.

## Formatting

Muon supports formatting code in LSP mode, but currently is not possible to configure the format when running in LSP mode.

If you want to change how code is formatted you need to manually change the [Zed formatter settings](https://zed.dev/docs/configuring-zed#formatter) for Meson to a formatter that you prefer.

If the custom formatter configuration changes the tab size, you should also specify this using the [Zed tab size setting](https://zed.dev/docs/configuring-zed#tab-size).

## Acknowledgments

This extension was created by taking parts of the now unmaintained [zed-meson](https://github.com/trshbn/zed-meson) extension.
