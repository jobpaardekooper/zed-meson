# Zed Meson Extension

![Muon Version Badge](https://img.shields.io/badge/Muon_LSP-v0.5.0-green)
![MesonLSP Version Badge](https://img.shields.io/badge/MesonLSP-v4.3.7-purple)
![license](https://img.shields.io/github/license/jobpaardekooper/zed-meson)
================================================================================

Extension for the [Zed](https://zed.dev) code editor, adding support for the
[Meson](https://mesonbuild.com) build system by allowing both [Muon](https://muon.build/) and [MesonLSP](https://github.com/JCWasmx86/mesonlsp) to be used as language servers.

## Features

- Syntax Highlighting
- Formatting
- Jump-To-Definition
- Jump-To-Subdir
- A basic set of diagnostics
- Basic autocompletion
- Configurable language server with support for both Muon and MesonLSP
  - `muon` as the language server ([features](https://docs.muon.build/features.html#muon-analyze))
  - `mesonlsp` as the language server ([features](https://github.com/JCWasmx86/mesonlsp?tab=readme-ov-file#current-feature-set))

## Language Server

Both Muon's LSP mode and MesonLSP are supported by this extension as separate language servers (`muon` and `mesonlsp`). Both support automatic installation. The extension first checks if the server binary is available in your `PATH`. If not, it downloads the matching version for your architecture and operating system.

> [!NOTE]  
> Muon has a [tracking issue regarding its LSP feature set](https://github.com/muon-build/muon/issues/159). Since MesonLSP has been archived and is no longer being developed, it is important that Muon's LSP functionality gets improved. Please contribute to Muon if you find it useful and would like a future-proof, actively developed and fully featured LSP for Meson (not just in Zed)!

You can choose which Meson language servers run through Zed's built-in language settings.

## Configuration

To choose which Meson language server(s) are used, configure Zed's `languages` settings in your `settings.json`:

```json
{
  "languages": {
    "Meson": {
      "language_servers": ["mesonlsp", "!muon"]
    }
  }
}
```

In `language_servers`, use `!` to disable a server. The current recommended configuration is to use only `mesonlsp` as shown above, since Muon's LSP mode is still in early development and lacks many features. However, you can also choose to use both servers at the same time.

## Formatting with Muon

Muon supports formatting code in LSP mode, but currently it is not possible to configure the format when running in LSP mode.

If you want to change how code is formatted, you need to manually change the [Zed formatter settings](https://zed.dev/docs/configuring-zed#formatter) for Meson to a formatter that you prefer.

If the custom formatter configuration changes the tab size, you should also specify this using the [Zed tab size setting](https://zed.dev/docs/configuring-zed#tab-size).

## Acknowledgments

This extension was created by taking parts of the now unmaintained [zed-meson](https://github.com/trshbn/zed-meson) extension.
