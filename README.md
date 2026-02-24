# Zed Meson Extension

![Muon Version Badge](https://img.shields.io/badge/Muon_LSP-v0.5.0-green)
![MesonLSP Version Badge](https://img.shields.io/badge/MesonLSP-v4.3.7-purple)
![license](https://img.shields.io/github/license/jobpaardekooper/zed-meson-muon)
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

Both Muon's LSP mode and MesonLSP are supported by this extension. Both have support for automatic installation. The extension will first check if you have the configured language server in your `PATH` and if so, it will use that. If not, the extension will automatically download the correct version for your architecture and operating system so no extra configuration should be needed.

> [!NOTE]  
> Muon has a [tracking issue regarding its LSP feature set](https://github.com/muon-build/muon/issues/159). Since MesonLSP has been archived and is no longer being developed, it is important that Muon's LSP functionality gets improved. Please contribute to Muon if you find it useful and would like a future-proof, actively developed and fully featured LSP for Meson (not just in Zed)!

Currently, `mesonlsp` is used as the default language server. In the future, once Muon's LSP features are more mature, the extension might switch to using Muon as the default language server. You can already use Muon as the language server through the extension settings described below.

## Configuration

To change the configuration of this extension:

- First, make sure this extension has been installed and any `meson.build` file has been opened inside of Zed.
- [find your installation location of Zed extensions](https://zed.dev/docs/extensions/installing-extensions#installation-location) and open that folder.
- Inside the `extensions` folder, navigate to `work/meson`
- Here you should find a `settings.json` file that looks as follows:

```json
{
  "lsp": "mesonlsp",
  "search_in_path": true
}
```

The `lsp` setting can be set to either `muon` or `mesonlsp` to select the language server you want to use. The `search_in_path` setting controls whether the extension should search for an already installed language server in the system path before downloading it. If `search_in_path` is set to `false`, the extension will always try to download the language server.

## Formatting with Muon

Muon supports formatting code in LSP mode, but currently is is not possible to configure the format when running in LSP mode.

If you want to change how code is formatted, you need to manually change the [Zed formatter settings](https://zed.dev/docs/configuring-zed#formatter) for Meson to a formatter that you prefer.

If the custom formatter configuration changes the tab size, you should also specify this using the [Zed tab size setting](https://zed.dev/docs/configuring-zed#tab-size).

## Acknowledgments

This extension was created by taking parts of the now unmaintained [zed-meson](https://github.com/trshbn/zed-meson) extension.
